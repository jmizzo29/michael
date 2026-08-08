//! MICHAEL-X: continuous optical-chemical field via Belousov–Zhabotinsky RD PDEs.
//! Pure Rust — no ndarray / numpy.

use std::f64::consts::PI;

/// Non-digital continuous field simulator (activator/inhibitor + holographic memory).
pub struct OpticalChemicalFieldEngine {
    pub size: usize,
    pub d: f64,
    /// Activator field u.
    pub u: Vec<f64>,
    /// Inhibitor field v.
    pub v: Vec<f64>,
    /// Holographic refractive-index memory grid.
    pub refractive_index: Vec<f64>,
}

impl OpticalChemicalFieldEngine {
    pub fn new(grid_size: usize, diffusion_rate: f64) -> Self {
        let n = grid_size * grid_size;
        Self {
            size: grid_size,
            d: diffusion_rate,
            u: vec![1.0; n],
            v: vec![0.0; n],
            refractive_index: vec![1.0; n],
        }
    }

    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.size + c
    }

    /// Converts raw text into spatial phase perturbation waves.
    pub fn encode_text_to_phasefront(&mut self, text: &str) {
        for (idx, &b) in text.as_bytes().iter().enumerate() {
            let x = (idx * 7) % self.size;
            let y = (usize::from(b) * 13) % self.size;
            let angle = (f64::from(b) / 255.0) * 2.0 * PI;
            let i = self.idx(x, y);
            self.u[i] += angle.cos();
            self.v[i] += angle.sin();
        }
    }

    /// Discrete Laplacian with toroidal wrap (roll ±1 on each axis).
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

    /// Solves one continuous reaction-diffusion PDE step:
    /// du/dt = D·∇²u + (u(1−u) − v·u)·n  
    /// dv/dt = D·∇²v + (u − v)
    pub fn step_field_pde(&mut self, dt: f64) {
        let lap_u = self.laplacian(&self.u);
        let lap_v = self.laplacian(&self.v);
        let cells = self.size * self.size;

        let mut du = vec![0.0; cells];
        let mut dv = vec![0.0; cells];

        for i in 0..cells {
            let u = self.u[i];
            let v = self.v[i];
            let n = self.refractive_index[i];
            du[i] = self.d * lap_u[i] + (u * (1.0 - u) - v * u) * n;
            dv[i] = self.d * lap_v[i] + (u - v);
        }

        for i in 0..cells {
            self.u[i] += du[i] * dt;
            self.v[i] += dv[i] * dt;

            // Holographic adaptation: interference updates refractive memory.
            let interference = (self.u[i] * self.v[i]).abs();
            self.refractive_index[i] += 0.001 * (interference - self.refractive_index[i]);
        }
    }

    /// Decodes spatial field resonances back into a generated character.
    pub fn read_bifurcated_output(&self) -> char {
        let cells = self.size * self.size;
        let mut peak_mag = 0.0_f64;
        let mut peak_i = 0usize;

        for i in 0..cells {
            // |u + i v|
            let mag = (self.u[i] * self.u[i] + self.v[i] * self.v[i]).sqrt();
            if mag > peak_mag {
                peak_mag = mag;
                peak_i = i;
            }
        }

        let _ = peak_i; // peak location available if needed for diagnostics
        let code = ((peak_mag * 100.0) as i64).rem_euclid(95) as u8 + 32;
        char::from(code)
    }
}

impl Default for OpticalChemicalFieldEngine {
    fn default() -> Self {
        Self::new(64, 0.16)
    }
}

/// Demo harness matching the Python optical-chemical field test.
pub fn run_demo() {
    println!("=== Testing MICHAEL-X Optical-Chemical Field Engine ===");

    let mut engine = OpticalChemicalFieldEngine::new(64, 0.16);
    let prompt = "MICHAEL RESONANCE";

    println!("Injecting Field Phase-Front: '{prompt}'");
    engine.encode_text_to_phasefront(prompt);

    // Perturbation must leave the uniform rest state.
    let energy: f64 = engine
        .u
        .iter()
        .zip(engine.v.iter())
        .map(|(u, v)| (u - 1.0).abs() + v.abs())
        .sum();
    assert!(energy > 0.0, "phasefront inject must disturb the field");

    println!("Evolving Reaction-Diffusion PDE Field...");
    let mut generated_chars = String::new();
    for step in 0..20 {
        engine.step_field_pde(0.5);
        if step % 2 == 0 {
            generated_chars.push(engine.read_bifurcated_output());
        }
    }

    assert_eq!(generated_chars.len(), 10);
    println!("Decoded Field Output Stream: '{generated_chars}'");
    println!();
}
