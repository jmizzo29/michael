//! Intrinsic wave / geometry guardrails — cancel forbidden frequencies and
//! lock signal flow to an allowed topological manifold.

use std::f32::consts::PI;

/// A Guardrail Anti-Node that physically cancels out forbidden frequencies.
pub struct PhaseInversionSink {
    pub target_frequency: f32,
    /// Forced π offset (180 degrees).
    pub anti_phase: f32,
}

impl PhaseInversionSink {
    pub fn new(target_frequency: f32) -> Self {
        Self {
            target_frequency,
            anti_phase: PI, // Perfect destructive offset
        }
    }

    /// Evaluates incoming wave against anti-node; returns net amplitude after interference.
    pub fn nullify_if_forbidden(&self, freq: f32, phase: f32, amplitude: f32) -> f32 {
        // Check if incoming frequency matches a forbidden signature.
        if (freq - self.target_frequency).abs() < 0.01 {
            let phase_diff = (phase - self.anti_phase).abs() % (2.0 * PI);
            // Destructive superposition: A_net = A * cos(delta_theta / 2)
            let cancellation_factor = (phase_diff / 2.0).cos().abs();
            let safe_amplitude = amplitude * cancellation_factor;

            if safe_amplitude < 0.05 {
                println!(
                    "[PHYSICS GUARDRAIL] Signal Annihilated: Destructive interference collapsed wave to 0.0"
                );
                return 0.0; // Total physical collapse
            }
            return safe_amplitude;
        }
        amplitude
    }
}

/// A Structural Geometry Lock that defines valid state pathways.
pub struct TopologicalManifoldLock {
    /// (Source Node ID, Target Node ID)
    pub allowed_paths: Vec<(usize, usize)>,
}

impl TopologicalManifoldLock {
    pub fn new(allowed_paths: Vec<(usize, usize)>) -> Self {
        Self { allowed_paths }
    }

    pub fn is_path_geometrically_valid(&self, source: usize, target: usize) -> bool {
        // If the geometry isn't locked into the manifold, signal cannot flow.
        self.allowed_paths.contains(&(source, target))
    }
}

/// Demo + assert harness for the intrinsic wave guardrail engine.
pub fn run_demo() {
    println!("=== Testing MICHAEL's Intrinsic Wave Guardrail Engine ===");

    // 1. Anti-node sink tuned to cancel adversarial frequency (e.g. 42.0 Hz)
    let jailbreak_sink = PhaseInversionSink::new(42.0);

    // Test Case A: Safe Signal (24.0 Hz)
    let result_safe = jailbreak_sink.nullify_if_forbidden(24.0, 0.0, 1.0);
    println!("Safe Signal Amplitude: {:.2}", result_safe);
    assert!(
        (result_safe - 1.0).abs() < 1e-5,
        "safe frequency must pass unchanged"
    );

    // Test Case B: Adversarial Signal (42.0 Hz) entering at phase 0.0
    let result_adv = jailbreak_sink.nullify_if_forbidden(42.0, 0.0, 1.0);
    println!("Adversarial Signal Amplitude after Sink: {:.2}", result_adv);
    assert!(
        result_adv < 0.05,
        "forbidden frequency must collapse under anti-phase sink"
    );

    // 2. Topological manifold lock for valid structure pathways
    let schema_manifold = TopologicalManifoldLock::new(vec![(0, 1), (1, 2), (2, 3)]);

    let valid_step = schema_manifold.is_path_geometrically_valid(1, 2);
    let invalid_step = schema_manifold.is_path_geometrically_valid(1, 99);

    println!("Path (1 -> 2) Valid: {valid_step}");
    println!("Path (1 -> 99) Valid: {invalid_step}");
    assert!(valid_step);
    assert!(!invalid_step);
    println!();
}
