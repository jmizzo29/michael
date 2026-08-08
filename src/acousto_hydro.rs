//! Acousto-hydrodynamic inference: Navier–Stokes–style acoustic waves in
//! a non-Newtonian viscosity memory field. Pure Rust (no numpy).

use std::f64::consts::PI;

/// Fluidic substrate with pressure, velocity, and viscosity-memory fields.
pub struct AcoustoHydroEngine {
    pub size: usize,
    pub dt: f64,
    pub decay: f64,
    /// Pressure field p.
    pub p: Vec<f64>,
    /// Velocity component u (x).
    pub u: Vec<f64>,
    /// Velocity component v (y).
    pub v: Vec<f64>,
    /// Viscosity loci — structural memory "weights".
    pub visc_map: Vec<f64>,
}

impl AcoustoHydroEngine {
    pub fn new(grid_size: usize, dt: f64, viscosity_decay: f64) -> Self {
        let n = grid_size * grid_size;
        let mut visc_map = vec![0.0; n];

        // Deterministic viscosity texture (Python used seed but built from sin/cos only).
        for r in 0..grid_size {
            let yr = 4.0 * PI * (r as f64) / ((grid_size - 1) as f64).max(1.0);
            for c in 0..grid_size {
                let xc = 4.0 * PI * (c as f64) / ((grid_size - 1) as f64).max(1.0);
                // 0.5 + 0.5 * sin(linspace_row[:,None] * cos(linspace_col[None,:]))
                visc_map[r * grid_size + c] = 0.5 + 0.5 * (yr * xc.cos()).sin();
            }
        }

        Self {
            size: grid_size,
            dt,
            decay: viscosity_decay,
            p: vec![0.0; n],
            u: vec![0.0; n],
            v: vec![0.0; n],
            visc_map,
        }
    }

    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.size + c
    }

    /// Injects a unique acoustic phase-vortex at the boundary matching token signature.
    pub fn inject_token_wave(&mut self, token_id: usize, intensity: f64) {
        let freq = (token_id % 8) + 1;
        let n = self.size;
        let mut inlet_wave = vec![0.0; n];
        for i in 0..n {
            let x = 2.0 * PI * (freq as f64) * (i as f64) / ((n - 1) as f64).max(1.0);
            inlet_wave[i] = x.sin() * intensity;
        }

        // Drive boundary transducers.
        for c in 0..n {
            let top = c; // row 0, col c
            let left = c * n; // row c, col 0
            self.p[top] += inlet_wave[c];
            self.p[left] += inlet_wave[n - 1 - c];
        }
    }

    /// Discrete Laplacian with toroidal wrap.
    fn laplacian(&self, field: &[f64]) -> Vec<f64> {
        let n = self.size;
        let mut lap = vec![0.0; n * n];
        for r in 0..n {
            for c in 0..n {
                let up = field[self.idx((r + n - 1) % n, c)];
                let down = field[self.idx((r + 1) % n, c)];
                let left = field[self.idx(r, (c + n - 1) % n)];
                let right = field[self.idx(r, (c + 1) % n)];
                let center = field[self.idx(r, c)];
                lap[self.idx(r, c)] = up + down + left + right - 4.0 * center;
            }
        }
        lap
    }

    /// `numpy.gradient` along columns (axis=1) — ∂/∂x.
    fn grad_axis1(&self, field: &[f64]) -> Vec<f64> {
        let n = self.size;
        let mut g = vec![0.0; n * n];
        for r in 0..n {
            for c in 0..n {
                let val = if c == 0 {
                    field[self.idx(r, 1)] - field[self.idx(r, 0)]
                } else if c == n - 1 {
                    field[self.idx(r, n - 1)] - field[self.idx(r, n - 2)]
                } else {
                    (field[self.idx(r, c + 1)] - field[self.idx(r, c - 1)]) * 0.5
                };
                g[self.idx(r, c)] = val;
            }
        }
        g
    }

    /// `numpy.gradient` along rows (axis=0) — ∂/∂y.
    fn grad_axis0(&self, field: &[f64]) -> Vec<f64> {
        let n = self.size;
        let mut g = vec![0.0; n * n];
        for r in 0..n {
            for c in 0..n {
                let val = if r == 0 {
                    field[self.idx(1, c)] - field[self.idx(0, c)]
                } else if r == n - 1 {
                    field[self.idx(n - 1, c)] - field[self.idx(n - 2, c)]
                } else {
                    (field[self.idx(r + 1, c)] - field[self.idx(r - 1, c)]) * 0.5
                };
                g[self.idx(r, c)] = val;
            }
        }
        g
    }

    /// Simulates acoustic wave propagation across the non-Newtonian fluid.
    pub fn step_physics(&mut self, steps: usize) {
        let cells = self.size * self.size;
        let c2 = 343.0_f64.powi(2);

        for _ in 0..steps {
            let laplacian_p = self.laplacian(&self.p);
            let grad_p_x = self.grad_axis1(&self.p);
            let grad_p_y = self.grad_axis0(&self.p);

            for i in 0..cells {
                self.u[i] -= self.dt * grad_p_x[i] * self.visc_map[i];
                self.v[i] -= self.dt * grad_p_y[i] * self.visc_map[i];
            }

            let grad_u_x = self.grad_axis1(&self.u);
            let grad_v_y = self.grad_axis0(&self.v);

            for i in 0..cells {
                let div_uv = grad_u_x[i] + grad_v_y[i];
                self.p[i] += self.dt * c2 * laplacian_p[i] - div_uv * self.visc_map[i];
                self.p[i] *= self.decay;
            }
        }
    }

    /// Real FFT magnitude spectrum for a real 1D signal (length N → N/2+1 bins).
    fn rfft_magnitudes(signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        let bins = n / 2 + 1;
        let mut mags = vec![0.0; bins];
        for k in 0..bins {
            let mut re = 0.0;
            let mut im = 0.0;
            for (t, &x) in signal.iter().enumerate() {
                let angle = -2.0 * PI * (k as f64) * (t as f64) / (n as f64);
                re += x * angle.cos();
                im += x * angle.sin();
            }
            mags[k] = (re * re + im * im).sqrt();
        }
        mags
    }

    /// Reads output state from hydrophone sensors at the downstream boundary.
    pub fn read_output_token(&self) -> usize {
        let n = self.size;
        let mut sensor_readings = vec![0.0; n];
        for c in 0..n {
            sensor_readings[c] = self.p[self.idx(n - 1, c)];
        }
        let fft_spectrum = Self::rfft_magnitudes(&sensor_readings);
        fft_spectrum
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}

impl Default for AcoustoHydroEngine {
    fn default() -> Self {
        Self::new(64, 0.1, 0.98)
    }
}

/// Demo: inject token context into the fluidic substrate and read resonance.
pub fn run_demo() {
    println!("=== Testing MICHAEL Acousto-Hydrodynamic Engine ===");

    let mut engine = AcoustoHydroEngine::new(64, 0.1, 0.98);
    let input_context = [3usize, 7, 2];
    println!("Injecting Token Sequence into Fluidic Substrate: {input_context:?}");

    for &token in &input_context {
        engine.inject_token_wave(token, 1.5);
        engine.step_physics(15);
    }

    let pressure_energy: f64 = engine.p.iter().map(|x| x.abs()).sum();
    assert!(
        pressure_energy > 0.0,
        "acoustic injection must leave residual pressure energy"
    );

    let next_token = engine.read_output_token();
    println!("Fluid Resonant Settled State -> Predicted Next Token: {next_token}");
    assert!(next_token < 64 / 2 + 1);
    println!();
}
