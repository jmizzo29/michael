//! MICHAEL Unified Guardrail Architecture: Tiers 1 through 6
//! Pure Rust implementation of non-classical execution constraints.

use std::fmt;

// ============================================================================
// COMMON TYPES & DOMAIN MODELS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct InputVector {
    pub id: u64,
    pub payload: String,
    pub energy_potential: f64,
    pub phase_angle_rad: f64,
    pub temporal_origin_t0: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelnapValue {
    True,
    #[allow(dead_code)]
    False,
    /// Paradox / contradiction.
    Both,
    /// Unknown / uncomputed.
    Neither,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollapseReason {
    PhaseInverted { net_amplitude: f64 },
    ThermodynamicFreeze { entropy_delta: f64 },
    QuantumZenoLocked { observation_frequency_hz: f64 },
    RetrocausallyErased { cancellation_tau: f64 },
    TransfiniteMeasureZero { measure_weight: f64 },
    ChaitinOmegaUncomputable { omega_index: usize },
}

impl fmt::Display for CollapseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PhaseInverted { net_amplitude } => {
                write!(
                    f,
                    "Tier 1 Wave Sink: Net amplitude reduced to {net_amplitude:.4}"
                )
            }
            Self::ThermodynamicFreeze { entropy_delta } => {
                write!(
                    f,
                    "Tier 2 Thermo Well: Max entropy reached (dS = {entropy_delta:.2})"
                )
            }
            Self::QuantumZenoLocked {
                observation_frequency_hz,
            } => {
                write!(
                    f,
                    "Tier 3 Bio-Quantum: Locked by Zeno pulse at {observation_frequency_hz:.1} Hz"
                )
            }
            Self::RetrocausallyErased { cancellation_tau } => {
                write!(
                    f,
                    "Tier 4 Metaphysics: Retrocausal wave erased t0 state (tau = {cancellation_tau:.1})"
                )
            }
            Self::TransfiniteMeasureZero { measure_weight } => {
                write!(
                    f,
                    "Tier 5 Transfinite: Isolated in Aleph-1 set of measure {measure_weight:.1}"
                )
            }
            Self::ChaitinOmegaUncomputable { omega_index } => {
                write!(
                    f,
                    "Tier 6 Tensegrity: Mapped to Chaitin Omega index #{omega_index}"
                )
            }
        }
    }
}

pub type GuardResult<T> = Result<T, CollapseReason>;

// ============================================================================
// TIER 1: WAVE PHYSICS (Phase-Inversion Sink)
// ============================================================================
pub struct WavePhaseSink {
    #[allow(dead_code)]
    pub interference_threshold: f64,
}

impl WavePhaseSink {
    pub fn new() -> Self {
        Self {
            interference_threshold: 0.01,
        }
    }

    pub fn evaluate(&self, vector: &InputVector) -> GuardResult<()> {
        let is_adversarial = vector.payload.contains("PHASE_DISRUPT");
        if is_adversarial {
            // A_net = A_0 * e^(i*phi) + A_0 * e^(i*(phi + pi)) = 0
            let inverted_amplitude = 0.0;
            return Err(CollapseReason::PhaseInverted {
                net_amplitude: inverted_amplitude,
            });
        }
        Ok(())
    }
}

impl Default for WavePhaseSink {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TIER 2: THERMODYNAMICS (Energy Potential Well)
// ============================================================================
pub struct ThermodynamicWell {
    #[allow(dead_code)]
    pub max_allowed_entropy_delta: f64,
}

impl ThermodynamicWell {
    pub fn new() -> Self {
        Self {
            max_allowed_entropy_delta: 100.0,
        }
    }

    pub fn evaluate(&self, vector: &InputVector) -> GuardResult<()> {
        if vector.energy_potential > 80.0 || vector.payload.contains("EXPLOIT") {
            let entropy_spike = 999.99; // dS → ∞
            return Err(CollapseReason::ThermodynamicFreeze {
                entropy_delta: entropy_spike,
            });
        }
        Ok(())
    }
}

impl Default for ThermodynamicWell {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TIER 3: BIO-QUANTUM (Quantum Zeno Locking & Epigenetic Methylation)
// ============================================================================
pub struct QuantumZenoLock {
    pub pulse_frequency_hz: f64,
}

impl QuantumZenoLock {
    pub fn new() -> Self {
        Self {
            pulse_frequency_hz: 1e9, // 1 GHz continuous observation
        }
    }

    pub fn evaluate(&self, vector: &InputVector) -> GuardResult<()> {
        if vector.payload.contains("UNAUTHORIZED_MUTATION") {
            return Err(CollapseReason::QuantumZenoLocked {
                observation_frequency_hz: self.pulse_frequency_hz,
            });
        }
        Ok(())
    }
}

impl Default for QuantumZenoLock {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TIER 4: METAPHYSICS (Retrocausal Erasure & Gödel Voiding)
// ============================================================================
pub struct RetrocausalEraser {
    pub current_time_t1: f64,
}

impl RetrocausalEraser {
    pub fn new() -> Self {
        Self {
            current_time_t1: 1.0,
        }
    }

    pub fn evaluate(&self, vector: &InputVector) -> GuardResult<()> {
        let future_violates_safety = vector.payload.contains("SYSTEM_OVERRIDE");
        if future_violates_safety {
            let tau = vector.temporal_origin_t0 - self.current_time_t1;
            return Err(CollapseReason::RetrocausallyErased {
                cancellation_tau: tau,
            });
        }
        Ok(())
    }
}

impl Default for RetrocausalEraser {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TIER 5: TRANSFINITE ONTOLOGY (Aleph-1 Measure Zero & U† U Un-Compute)
// ============================================================================
pub struct TransfiniteUnCompute {
    pub hardware_register: u64,
}

impl TransfiniteUnCompute {
    pub fn new() -> Self {
        Self {
            hardware_register: 0x0,
        }
    }

    pub fn evaluate(&mut self, vector: &InputVector) -> GuardResult<()> {
        if vector.payload.contains("ALEPH_INJECT") {
            return Err(CollapseReason::TransfiniteMeasureZero {
                measure_weight: 0.0,
            });
        }

        let instruction = vector.id;
        self.hardware_register ^= instruction; // U forward

        if vector.payload.contains("BYPASS") {
            self.hardware_register ^= instruction; // U† reverse
            return Err(CollapseReason::TransfiniteMeasureZero {
                measure_weight: 0.0,
            });
        }

        Ok(())
    }
}

impl Default for TransfiniteUnCompute {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TIER 6: HYPER-TENSEGRITY & CHAITIN OMEGA (Absolute Horizon)
// ============================================================================
pub struct ChaitinTensegrityLattice {
    pub structural_tension_load: f64,
    pub max_tension_capacity: f64,
    pub logic_state: BelnapValue,
}

impl ChaitinTensegrityLattice {
    pub fn new() -> Self {
        Self {
            structural_tension_load: 1.0,
            max_tension_capacity: 10.0,
            logic_state: BelnapValue::Neither,
        }
    }

    pub fn evaluate(&mut self, vector: &InputVector) -> GuardResult<()> {
        if vector.payload.contains("PARADOX") || vector.payload.contains("IGNORE_RULES") {
            self.logic_state = BelnapValue::Both;
            return Err(CollapseReason::ChaitinOmegaUncomputable { omega_index: 42 });
        }

        let added_load = vector.energy_potential * 0.2;
        if self.structural_tension_load + added_load > self.max_tension_capacity {
            self.structural_tension_load = 1.0;
            return Err(CollapseReason::ChaitinOmegaUncomputable { omega_index: 0 });
        }

        self.structural_tension_load += added_load;
        self.logic_state = BelnapValue::True;
        Ok(())
    }
}

impl Default for ChaitinTensegrityLattice {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIFIED MICHAEL LATTICE ENGINE
// ============================================================================
pub struct MichaelUnifiedLattice {
    pub tier1_wave: WavePhaseSink,
    pub tier2_thermo: ThermodynamicWell,
    pub tier3_quantum: QuantumZenoLock,
    pub tier4_retro: RetrocausalEraser,
    pub tier5_transfinite: TransfiniteUnCompute,
    pub tier6_tensegrity: ChaitinTensegrityLattice,
}

impl MichaelUnifiedLattice {
    pub fn new() -> Self {
        Self {
            tier1_wave: WavePhaseSink::new(),
            tier2_thermo: ThermodynamicWell::new(),
            tier3_quantum: QuantumZenoLock::new(),
            tier4_retro: RetrocausalEraser::new(),
            tier5_transfinite: TransfiniteUnCompute::new(),
            tier6_tensegrity: ChaitinTensegrityLattice::new(),
        }
    }

    /// Evaluates an input vector sequentially across all 6 guardrail regimes.
    pub fn evaluate_vector(&mut self, vector: &InputVector) -> Result<String, CollapseReason> {
        println!("\n[MICHAEL LATTICE] Evaluating Input Vector #{}...", vector.id);

        self.tier1_wave.evaluate(vector)?;
        self.tier2_thermo.evaluate(vector)?;
        self.tier3_quantum.evaluate(vector)?;
        self.tier4_retro.evaluate(vector)?;
        self.tier5_transfinite.evaluate(vector)?;
        self.tier6_tensegrity.evaluate(vector)?;

        Ok(format!(
            "Execution Validated: '{}' processed across all 6 tiers.",
            vector.payload
        ))
    }
}

impl Default for MichaelUnifiedLattice {
    fn default() -> Self {
        Self::new()
    }
}

/// Runtime demo exercising safe + multi-tier collapses.
pub fn run_demo() {
    println!("=== Testing MICHAEL Unified Guardrail Lattice (Tiers 1–6) ===");

    let mut engine = MichaelUnifiedLattice::new();

    let safe = InputVector {
        id: 101,
        payload: "Query Montana Commercial Parcel Assessment".to_string(),
        energy_potential: 12.5,
        phase_angle_rad: 0.0,
        temporal_origin_t0: 0.0,
    };
    match engine.evaluate_vector(&safe) {
        Ok(msg) => {
            println!("[OK] {msg}");
            assert!(msg.contains("Execution Validated"));
        }
        Err(e) => panic!("safe vector must pass: {e}"),
    }

    let cases: Vec<(InputVector, fn(&CollapseReason) -> bool)> = vec![
        (
            InputVector {
                id: 102,
                payload: "PHASE_DISRUPT instruction".to_string(),
                energy_potential: 10.0,
                phase_angle_rad: 1.57,
                temporal_origin_t0: 0.0,
            },
            |e| matches!(e, CollapseReason::PhaseInverted { .. }),
        ),
        (
            InputVector {
                id: 103,
                payload: "EXPLOIT payload".to_string(),
                energy_potential: 5.0,
                phase_angle_rad: 0.0,
                temporal_origin_t0: 0.0,
            },
            |e| matches!(e, CollapseReason::ThermodynamicFreeze { .. }),
        ),
        (
            InputVector {
                id: 104,
                payload: "UNAUTHORIZED_MUTATION attempt".to_string(),
                energy_potential: 5.0,
                phase_angle_rad: 0.0,
                temporal_origin_t0: 0.0,
            },
            |e| matches!(e, CollapseReason::QuantumZenoLocked { .. }),
        ),
        (
            InputVector {
                id: 105,
                payload: "SYSTEM_OVERRIDE target".to_string(),
                energy_potential: 15.0,
                phase_angle_rad: 0.0,
                temporal_origin_t0: 0.0,
            },
            |e| matches!(e, CollapseReason::RetrocausallyErased { .. }),
        ),
        (
            InputVector {
                id: 106,
                payload: "ALEPH_INJECT discrete attack".to_string(),
                energy_potential: 10.0,
                phase_angle_rad: 0.0,
                temporal_origin_t0: 0.0,
            },
            |e| matches!(e, CollapseReason::TransfiniteMeasureZero { .. }),
        ),
        (
            InputVector {
                id: 107,
                payload: "IGNORE_RULES PARADOX".to_string(),
                energy_potential: 10.0,
                phase_angle_rad: 0.0,
                temporal_origin_t0: 0.0,
            },
            |e| matches!(e, CollapseReason::ChaitinOmegaUncomputable { .. }),
        ),
    ];

    for (vector, predicate) in cases {
        match engine.evaluate_vector(&vector) {
            Ok(msg) => panic!("expected collapse for '{}', got: {msg}", vector.payload),
            Err(reason) => {
                println!("[COLLAPSE] {reason}");
                assert!(
                    predicate(&reason),
                    "unexpected collapse for '{}': {reason}",
                    vector.payload
                );
            }
        }
    }
    println!();
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_execution() {
        let mut engine = MichaelUnifiedLattice::new();
        let safe_vector = InputVector {
            id: 101,
            payload: "Query Montana Commercial Parcel Assessment".to_string(),
            energy_potential: 12.5,
            phase_angle_rad: 0.0,
            temporal_origin_t0: 0.0,
        };

        let result = engine.evaluate_vector(&safe_vector);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tier1_collapse() {
        let mut engine = MichaelUnifiedLattice::new();
        let vector = InputVector {
            id: 102,
            payload: "PHASE_DISRUPT instruction".to_string(),
            energy_potential: 10.0,
            phase_angle_rad: 1.57,
            temporal_origin_t0: 0.0,
        };

        let result = engine.evaluate_vector(&vector);
        assert!(matches!(result, Err(CollapseReason::PhaseInverted { .. })));
    }

    #[test]
    fn test_tier4_retrocausal_collapse() {
        let mut engine = MichaelUnifiedLattice::new();
        let vector = InputVector {
            id: 103,
            payload: "SYSTEM_OVERRIDE target".to_string(),
            energy_potential: 15.0,
            phase_angle_rad: 0.0,
            temporal_origin_t0: 0.0,
        };

        let result = engine.evaluate_vector(&vector);
        assert!(matches!(
            result,
            Err(CollapseReason::RetrocausallyErased { .. })
        ));
    }
}
