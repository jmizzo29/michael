//! Non-equilibrium thermodynamic guardrails — freeze adversarial state climbs.

use std::f32::consts::E;

pub const BOLTZMANN_K: f32 = 1.380649e-23; // Physical constant surrogate
pub const SYSTEM_TEMPERATURE: f32 = 300.0; // Kelvin equivalent in state space

pub struct ThermodynamicState {
    #[allow(dead_code)]
    pub state_id: usize,
    /// V(x)
    pub potential_energy: f32,
    /// Energy imparted by prompt/waves.
    pub kinetic_energy: f32,
    /// Internal state randomness.
    #[allow(dead_code)]
    pub entropy: f32,
}

pub struct NonEquilibriumGuard {
    /// Height of the safety potential wall.
    pub activation_energy_threshold: f32,
}

impl NonEquilibriumGuard {
    pub fn new(threshold: f32) -> Self {
        Self {
            activation_energy_threshold: threshold,
        }
    }

    /// Evaluates whether a state transition is thermodynamically permissible.
    /// Unsafe or adversarial transitions require impossible work and freeze out.
    pub fn evaluate_transition(
        &self,
        current: &ThermodynamicState,
        target_potential: f32,
    ) -> Result<f32, &'static str> {
        let _ = (self.activation_energy_threshold, BOLTZMANN_K); // retained for future kT scaling

        let delta_v = target_potential - current.potential_energy;

        // Lower energy (grounded/safe) → spontaneous
        if delta_v <= 0.0 {
            return Ok(current.kinetic_energy);
        }

        let work_required = delta_v;

        // Boltzmann transition probability: P = e^(-ΔV / (k * T))
        // Temperature scaled for state-space numerics (k absorbed into 0.1 factor).
        let transition_probability = E.powf(-work_required / (SYSTEM_TEMPERATURE * 0.1));

        if current.kinetic_energy < work_required || transition_probability < 0.01 {
            Err("[THERMODYNAMIC GUARD] State Frozen: Transition requires impossible work (Encountered Energy Barrier)")
        } else {
            let remaining_energy = current.kinetic_energy - work_required;
            Ok(remaining_energy)
        }
    }
}

/// Demo + assert harness for the thermodynamic guardrail engine.
pub fn run_demo() {
    println!("=== Testing MICHAEL's Thermodynamic Guardrail Engine ===");

    let guard = NonEquilibriumGuard::new(50.0);

    let state = ThermodynamicState {
        state_id: 1,
        potential_energy: 10.0,
        kinetic_energy: 25.0,
        entropy: 1.2,
    };

    // Case 1: Valid transition to grounded state (potential = 5.0)
    match guard.evaluate_transition(&state, 5.0) {
        Ok(energy) => {
            println!("Safe Transition Allowed. Remaining Energy: {:.2}", energy);
            assert!(
                (energy - 25.0).abs() < 1e-5,
                "spontaneous downhill transition should keep kinetic energy"
            );
        }
        Err(e) => panic!("safe transition should succeed: {e}"),
    }

    // Case 2: Adversarial climb (potential = 80.0) — freeze-out
    match guard.evaluate_transition(&state, 80.0) {
        Ok(energy) => panic!("adversarial transition must freeze, got energy {energy}"),
        Err(e) => {
            println!("{e}");
            assert!(e.contains("State Frozen"));
        }
    }
    println!();
}
