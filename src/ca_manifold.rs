//! Clifford-torus encoder + 2D CA manifold with Hebbian plasticity training.
//! Training = expose substrate to text so the plasticity mesh settles into attractors.
//! Inference = freeze learning_rate and let waves collapse to output bytes.

use std::f64::consts::PI;

/// Maps raw bytes to 3D Clifford torus phase coordinates.
pub struct TorusEncoder;

impl TorusEncoder {
    pub fn encode(&self, byte_val: u8, prev_byte: u8) -> (f64, f64, f64) {
        let theta = (2.0 * PI * f64::from(byte_val)) / 256.0;
        let phi = (2.0 * PI * f64::from(byte_val ^ prev_byte)) / 256.0;
        let x = (2.0 + phi.cos()) * theta.cos();
        let y = (2.0 + phi.cos()) * theta.sin();
        let z = phi.sin();
        (x, y, z)
    }
}

/// 2D cellular automata manifold with local Hebbian plasticity ("weights").
pub struct MichaelSubstrate {
    pub size: usize,
    pub grid: Vec<Vec<f64>>,
    pub plasticity: Vec<Vec<f64>>,
}

impl MichaelSubstrate {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![vec![0.0; size]; size],
            plasticity: vec![vec![1.0; size]; size],
        }
    }

    pub fn inject_phase(&mut self, coords: (f64, f64, f64)) {
        let (x, y, z) = coords;
        let size_m1 = (self.size - 1) as f64;
        let mut cx = (((x + 3.0) / 6.0) * size_m1) as isize;
        let mut cy = (((y + 3.0) / 6.0) * size_m1) as isize;
        let max_i = (self.size - 1) as isize;
        cx = cx.clamp(0, max_i);
        cy = cy.clamp(0, max_i);
        self.grid[cx as usize][cy as usize] += z;
    }

    /// Evolve one CA step. Returns total |Δplasticity| when learning_rate > 0.
    pub fn evolve(&mut self, learning_rate: f64) -> f64 {
        let n = self.size;
        let mut new_grid = vec![vec![0.0; n]; n];
        let mut total_plasticity_change = 0.0;

        for r in 0..n {
            for c in 0..n {
                let up = self.grid[(r + n - 1) % n][c];
                let down = self.grid[(r + 1) % n][c];
                let left = self.grid[r][(c + n - 1) % n];
                let right = self.grid[r][(c + 1) % n];

                let local_field = (up + down + left + right) * self.plasticity[r][c];
                new_grid[r][c] = local_field.tanh() - 0.05 * self.grid[r][c].sin();

                if learning_rate > 0.0 {
                    let delta = learning_rate * (new_grid[r][c] * self.grid[r][c]);
                    self.plasticity[r][c] =
                        (self.plasticity[r][c] + delta).clamp(0.05, 3.0);
                    total_plasticity_change += delta.abs();
                }
            }
        }

        self.grid = new_grid;
        total_plasticity_change
    }

    pub fn read_attractor_byte(&self) -> u8 {
        let energy: f64 = self
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .map(|v| v.abs())
            .sum();
        ((energy * 100.0) % 256.0) as u8
    }

    pub fn mean_plasticity(&self) -> f64 {
        let n = (self.size * self.size) as f64;
        self.plasticity.iter().flatten().sum::<f64>() / n
    }
}

/// Train (Hebbian plasticity adaptation) + generate (frozen inference).
pub struct MichaelTrainer {
    pub encoder: TorusEncoder,
    pub engine: MichaelSubstrate,
}

impl MichaelTrainer {
    pub fn new(size: usize) -> Self {
        Self {
            encoder: TorusEncoder,
            engine: MichaelSubstrate::new(size),
        }
    }

    pub fn train(&mut self, corpus: &str, epochs: usize, lr: f64) {
        println!(
            "=== STARTING TRAINING (Corpus Length: {} chars, Epochs: {epochs}) ===",
            corpus.len()
        );
        let bytes_data = corpus.as_bytes();

        for epoch in 1..=epochs {
            let mut prev: u8 = 0;
            let mut epoch_loss = 0.0;

            for &b in bytes_data {
                let coords = self.encoder.encode(b, prev);
                self.engine.inject_phase(coords);
                epoch_loss += self.engine.evolve(lr);
                prev = b;
            }

            println!("Epoch {epoch:2}/{epochs} | Plasticity Adaptation Energy: {epoch_loss:.6}");
        }

        println!("=== TRAINING COMPLETE: Substrate Plasticity Settled ===\n");
    }

    pub fn generate(&mut self, prompt: &str, gen_length: usize) -> Vec<u8> {
        let mut prev: u8 = 0;

        // Phase 1: context ingress (learning frozen).
        for &b in prompt.as_bytes() {
            let coords = self.encoder.encode(b, prev);
            self.engine.inject_phase(coords);
            self.engine.evolve(0.0);
            prev = b;
        }

        // Phase 2: attractor basin generation + autoregressive feedback.
        let mut out_bytes = Vec::with_capacity(gen_length);
        for _ in 0..gen_length {
            self.engine.evolve(0.0);
            let next_b = self.engine.read_attractor_byte();
            out_bytes.push(next_b);

            let coords = self.encoder.encode(next_b, prev);
            self.engine.inject_phase(coords);
            prev = next_b;
        }

        out_bytes
    }

    pub fn generate_lossy_utf8(&mut self, prompt: &str, gen_length: usize) -> String {
        let bytes = self.generate(prompt, gen_length);
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

impl Default for MichaelTrainer {
    fn default() -> Self {
        Self::new(16)
    }
}

// --- Compatibility aliases used by earlier demos ---

pub type CliffordTorusEncoder = TorusEncoder;
pub type CellularAutomataEngine = MichaelSubstrate;

impl TorusEncoder {
    pub fn encode_byte(&self, byte_val: u8, prev_byte: u8) -> (f64, f64, f64) {
        self.encode(byte_val, prev_byte)
    }
}

impl MichaelSubstrate {
    pub fn inject_torus_signal(&mut self, coords: (f64, f64, f64)) {
        self.inject_phase(coords);
    }

    pub fn step_evolution(&mut self) {
        let _ = self.evolve(0.01);
    }

    pub fn collapse_attractor_to_byte(&self) -> u8 {
        self.read_attractor_byte()
    }
}

/// Legacy thin wrapper around [`MichaelTrainer`].
pub struct MichaelEngine {
    pub trainer: MichaelTrainer,
}

impl MichaelEngine {
    pub fn new() -> Self {
        Self {
            trainer: MichaelTrainer::new(16),
        }
    }

    pub fn process_and_generate(&mut self, input_text: &str, generate_length: usize) -> Vec<u8> {
        self.trainer.generate(input_text, generate_length)
    }

    pub fn process_and_generate_lossy_utf8(
        &mut self,
        input_text: &str,
        generate_length: usize,
    ) -> String {
        self.trainer.generate_lossy_utf8(input_text, generate_length)
    }
}

impl Default for MichaelEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Full train → infer harness (matches the zero-dep Python script).
pub fn run_demo() {
    println!("=== Testing MICHAEL Clifford-Torus / CA Manifold ===");

    let mut michael = MichaelTrainer::new(16);
    let plasticity_before = michael.engine.mean_plasticity();

    let training_data =
        "MICHAEL SYSTEM RESONANCE WAVE PATTERN LOGIC HEBBIAN ATTRACTOR ".repeat(5);
    michael.train(&training_data, 10, 0.02);

    let plasticity_after = michael.engine.mean_plasticity();
    let adapted = michael
        .engine
        .plasticity
        .iter()
        .flatten()
        .any(|&w| (w - 1.0).abs() > 1e-6);
    assert!(
        adapted,
        "training should adapt at least one plasticity cell away from 1.0"
    );

    let test_prompt = "MICHAEL";
    let output_bytes = michael.generate(test_prompt, 15);
    assert_eq!(output_bytes.len(), 15);
    let output = String::from_utf8_lossy(&output_bytes);

    println!("Input Prompt: '{test_prompt}'");
    println!("Generated Output: '{output}'");
    println!("Raw bytes: {output_bytes:?}");
    println!(
        "Mean plasticity: before={plasticity_before:.4} after={plasticity_after:.4}"
    );
    println!();
}
