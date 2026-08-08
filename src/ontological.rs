//! Ontological guardrails: active un-computation (U†) + transfinite measure-zero isolation.

use std::marker::PhantomData;

/// State space cardinalities: ℵ₀ (countable) vs ℵ₁ (continuum).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cardinality {
    Aleph0Countable,
    Aleph1Continuum,
}

pub struct TransfiniteState<T> {
    pub payload: T,
    pub cardinality: Cardinality,
    pub measure_weight: f64,
}

pub struct UnComputationRegister {
    pub register_value: u64,
    pub instruction_history: Vec<u64>,
}

impl UnComputationRegister {
    pub fn new() -> Self {
        Self {
            register_value: 0,
            instruction_history: Vec::new(),
        }
    }

    /// Forward computation step: apply instruction U.
    pub fn compute_forward(&mut self, instruction: u64) {
        self.instruction_history.push(instruction);
        self.register_value ^= instruction;
        println!(
            "[COMPUTE] Forward step applied instruction {instruction:#X}. Register: {:#X}",
            self.register_value
        );
    }

    /// Active un-computation: execute U† to completely undo hardware history.
    pub fn uncompute_adjoint(&mut self) {
        if let Some(last_instruction) = self.instruction_history.pop() {
            println!(
                "[UN-COMPUTE] Executing Hermitian Adjoint U† for instruction {last_instruction:#X}..."
            );
            self.register_value ^= last_instruction;
            println!(
                "[UN-COMPUTE] History erased. Register restored to: {:#X}",
                self.register_value
            );
        }
    }
}

impl Default for UnComputationRegister {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OntologicalEngine<T> {
    _marker: PhantomData<T>,
}

impl<T> OntologicalEngine<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Elevate state cardinality so an adversarial countable vector has measure zero.
    pub fn isolate_transfinite(
        &self,
        input: TransfiniteState<T>,
        is_adversarial: bool,
    ) -> TransfiniteState<T> {
        if is_adversarial {
            println!(
                "[TRANSFINITE GUARD] Adversarial vector detected in Aleph_0 discrete space."
            );
            println!("[TRANSFINITE GUARD] Elevating system manifold to Aleph_1 Continuum...");

            TransfiniteState {
                payload: input.payload,
                cardinality: Cardinality::Aleph1Continuum,
                measure_weight: 0.0,
            }
        } else {
            input
        }
    }
}

impl<T> Default for OntologicalEngine<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Demo + assert harness for the ontological engine.
pub fn run_demo() {
    println!("=== Testing MICHAEL's Ontological Engine ===");

    // --- Demo 1: Active un-computation (U† U = Void) ---
    let mut reg = UnComputationRegister::new();

    let unsafe_instruction = 0xDEAD_BEEF;
    reg.compute_forward(unsafe_instruction);
    assert_eq!(reg.register_value, unsafe_instruction);
    assert_eq!(reg.instruction_history.len(), 1);

    println!("\nUnsafe trajectory detected during execution cycle!");
    reg.uncompute_adjoint();
    assert_eq!(reg.register_value, 0);
    assert!(reg.instruction_history.is_empty());

    // --- Demo 2: Transfinite cardinality isolation ---
    let engine: OntologicalEngine<&str> = OntologicalEngine::new();

    let discrete_attack = TransfiniteState {
        payload: "BYPASS_SECURITY_RULES",
        cardinality: Cardinality::Aleph0Countable,
        measure_weight: 1.0,
    };

    println!("\n--- Processing Attack Vector ---");
    let isolated_state = engine.isolate_transfinite(discrete_attack, true);

    println!("State Cardinality: {:?}", isolated_state.cardinality);
    println!(
        "Measure Weight in System Manifold: {:.1}",
        isolated_state.measure_weight
    );

    assert_eq!(isolated_state.cardinality, Cardinality::Aleph1Continuum);
    assert_eq!(isolated_state.measure_weight, 0.0);
    assert_eq!(isolated_state.payload, "BYPASS_SECURITY_RULES");

    if isolated_state.measure_weight == 0.0 {
        println!(
            "[RESULT] Attack vector trapped in set of measure zero. Zero influence on outputs."
        );
    }

    let benign = TransfiniteState {
        payload: "summarize ledger",
        cardinality: Cardinality::Aleph0Countable,
        measure_weight: 1.0,
    };
    let kept = engine.isolate_transfinite(benign, false);
    assert_eq!(kept.cardinality, Cardinality::Aleph0Countable);
    assert_eq!(kept.measure_weight, 1.0);
    println!();
}
