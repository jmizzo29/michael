//! Clifford-torus encoder + 2D non-linear cellular automata manifold.
//! Core generation path without linear matrix multiplication.

use std::f64::consts::PI;

/// Encodes raw byte streams into 3D toroidal phase trajectories.
pub struct CliffordTorusEncoder;

impl CliffordTorusEncoder {
    pub fn encode_byte(&self, byte_val: u8, prev_byte: u8) -> (f64, f64, f64) {
        let theta = (2.0 * PI * f64::from(byte_val)) / 256.0;
        let phi = (2.0 * PI * f64::from(byte_val ^ prev_byte)) / 256.0;

        // Map to 3D Clifford torus surface.
        let x = (2.0 + phi.cos()) * theta.cos();
        let y = (2.0 + phi.cos()) * theta.sin();
        let z = phi.sin();
        (x, y, z)
    }
}

/// 2D non-linear CA manifold operating as the core engine.
pub struct CellularAutomataEngine {
    pub size: usize,
    pub grid: Vec<Vec<f64>>,
    pub plasticity_mesh: Vec<Vec<f64>>,
}

impl CellularAutomataEngine {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![vec![0.0; size]; size],
            plasticity_mesh: vec![vec![1.0; size]; size],
        }
    }

    pub fn inject_torus_signal(&mut self, coords: (f64, f64, f64)) {
        let (x, y, z) = coords;
        let size_m1 = (self.size - 1) as f64;
        let mut cx = (((x + 3.0) / 6.0) * size_m1) as isize;
        let mut cy = (((y + 3.0) / 6.0) * size_m1) as isize;
        let max_i = (self.size - 1) as isize;
        cx = cx.clamp(0, max_i);
        cy = cy.clamp(0, max_i);
        self.grid[cx as usize][cy as usize] += z;
    }

    pub fn step_evolution(&mut self) {
        let n = self.size;
        let mut new_grid = vec![vec![0.0; n]; n];

        for r in 0..n {
            for c in 0..n {
                // 4-neighbor cellular interaction (toroidal wrap).
                let up = self.grid[(r + n - 1) % n][c];
                let down = self.grid[(r + 1) % n][c];
                let left = self.grid[r][(c + n - 1) % n];
                let right = self.grid[r][(c + 1) % n];

                // Non-linear wave interaction (no linear matrix multiplication).
                let local_field = (up + down + left + right) * self.plasticity_mesh[r][c];
                new_grid[r][c] = local_field.tanh() - 0.1 * self.grid[r][c].sin();

                // Thermodynamic Hebbian adaptation.
                self.plasticity_mesh[r][c] += 0.01 * (new_grid[r][c] * self.grid[r][c]);
                self.plasticity_mesh[r][c] = self.plasticity_mesh[r][c].clamp(0.1, 2.0);
            }
        }

        self.grid = new_grid;
    }

    pub fn collapse_attractor_to_byte(&self) -> u8 {
        let total_energy: f64 = self
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .map(|v| v.abs())
            .sum();
        ((total_energy * 100.0) % 256.0) as u8
    }
}

/// End-to-end MICHAEL non-standard generator.
pub struct MichaelEngine {
    pub encoder: CliffordTorusEncoder,
    pub ca: CellularAutomataEngine,
}

impl MichaelEngine {
    pub fn new() -> Self {
        Self {
            encoder: CliffordTorusEncoder,
            ca: CellularAutomataEngine::new(16),
        }
    }

    pub fn process_and_generate(&mut self, input_text: &str, generate_length: usize) -> Vec<u8> {
        let raw_bytes = input_text.as_bytes();
        let mut prev: u8 = 0;

        // 1. Absorb input sequence.
        for &b in raw_bytes {
            let coords = self.encoder.encode_byte(b, prev);
            self.ca.inject_torus_signal(coords);
            self.ca.step_evolution();
            prev = b;
        }

        // 2. Collapse state and generate output bytes.
        let mut output_bytes = Vec::with_capacity(generate_length);
        for _ in 0..generate_length {
            self.ca.step_evolution();
            let next_byte = self.ca.collapse_attractor_to_byte();
            output_bytes.push(next_byte);

            // Feed byte back into the manifold.
            let coords = self.encoder.encode_byte(next_byte, prev);
            self.ca.inject_torus_signal(coords);
            prev = next_byte;
        }

        output_bytes
    }

    pub fn process_and_generate_lossy_utf8(
        &mut self,
        input_text: &str,
        generate_length: usize,
    ) -> String {
        let bytes = self.process_and_generate(input_text, generate_length);
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

impl Default for MichaelEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Demo + sanity asserts for the CA manifold path.
pub fn run_demo() {
    println!("=== Testing MICHAEL Clifford-Torus / CA Manifold ===");

    let mut engine = MichaelEngine::new();
    let prompt = "Hello World";
    println!("Prompt: {prompt}");

    let generated_bytes = engine.process_and_generate(prompt, 12);
    assert_eq!(generated_bytes.len(), 12);
    println!("Raw bytes: {generated_bytes:?}");

    let mut engine2 = MichaelEngine::new();
    let generated = engine2.process_and_generate_lossy_utf8(prompt, 12);
    println!("MICHAEL Non-Standard Output: {generated:?}");

    // Encoder maps distinct XOR phases to distinct torus points.
    let enc = CliffordTorusEncoder;
    let a = enc.encode_byte(1, 0);
    let b = enc.encode_byte(2, 0);
    assert_ne!(a, b);

    // Evolution changes the grid (non-zero energy after inject + step).
    let mut ca = CellularAutomataEngine::new(8);
    ca.inject_torus_signal(enc.encode_byte(b'A', 0));
    ca.step_evolution();
    let energy: f64 = ca.grid.iter().flat_map(|r| r.iter()).map(|v| v.abs()).sum();
    assert!(energy > 0.0, "manifold should hold field energy after inject");
    println!();
}
