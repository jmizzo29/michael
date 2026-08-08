//! Hyper-computable tensegrity: Belnap 4-value logic + Chaitin isolation + load redistribution.

/// Paraconsistent 4-value logic (Belnap lattice).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelnapValue {
    True,
    #[allow(dead_code)]
    False,
    /// Contradiction / paradox (adversarial attack state).
    Both,
    /// Unknown / uncomputed.
    Neither,
}

/// A tensegrity node connected by pre-stressed structural tension.
pub struct TensegrityNode {
    #[allow(dead_code)]
    pub id: usize,
    /// Structural cable tension.
    pub tension_load: f32,
    pub logic_state: BelnapValue,
    /// Mapped to non-computable Ω space.
    pub is_chaitin_isolated: bool,
}

pub struct HyperTensegrityEngine {
    pub nodes: Vec<TensegrityNode>,
    pub max_structural_tension: f32,
}

impl HyperTensegrityEngine {
    pub fn new(node_count: usize) -> Self {
        let nodes = (0..node_count)
            .map(|id| TensegrityNode {
                id,
                tension_load: 1.0, // Equilibrium baseline
                logic_state: BelnapValue::Neither,
                is_chaitin_isolated: false,
            })
            .collect();

        Self {
            nodes,
            max_structural_tension: 10.0,
        }
    }

    /// Map unsafe states to Chaitin's Ω non-computable space.
    pub fn isolate_chaitin_omega(&mut self, node_id: usize) {
        let node = &mut self.nodes[node_id];
        node.is_chaitin_isolated = true;
        node.logic_state = BelnapValue::Both; // Isolated paradox
        println!(
            "[CHAITIN GUARD] Node {node_id} mapped to non-computable Omega address. CPU evaluation disabled."
        );
    }

    /// Redistribute tension load across the tensegrity frame.
    pub fn apply_point_load(
        &mut self,
        target_node: usize,
        load: f32,
    ) -> Result<(), &'static str> {
        if target_node >= self.nodes.len() {
            return Err("[TENSEGRITY ENGINE] Node id out of arena.");
        }

        if self.nodes[target_node].is_chaitin_isolated {
            return Err(
                "[TENSEGRITY ENGINE] Operation Skipped: Address is Chaitin-uncomputable.",
            );
        }

        let new_tension = self.nodes[target_node].tension_load + load;

        if new_tension > self.max_structural_tension {
            println!(
                "[TENSEGRITY GUARD] Excessive point-load ({new_tension:.1}) on Node {target_node}! Buckling frame..."
            );

            // Isolate, then rebalance (no overlapping &mut borrows).
            self.isolate_chaitin_omega(target_node);
            self.redistribute_structural_equilibrium();
            Err(
                "[TENSEGRITY GUARD] Frame buckled safely away from illegal load. Equilibrium restored.",
            )
        } else {
            let node = &mut self.nodes[target_node];
            node.tension_load = new_tension;
            node.logic_state = BelnapValue::True;
            Ok(())
        }
    }

    /// Automatically balance structural tension across all active nodes.
    fn redistribute_structural_equilibrium(&mut self) {
        let active_count = self.nodes.iter().filter(|n| !n.is_chaitin_isolated).count();
        if active_count == 0 {
            return;
        }

        let total_safe_tension: f32 = self
            .nodes
            .iter()
            .filter(|n| !n.is_chaitin_isolated)
            .map(|n| n.tension_load)
            .sum();

        let balanced_load = total_safe_tension / active_count as f32;

        for node in self.nodes.iter_mut() {
            if !node.is_chaitin_isolated {
                node.tension_load = balanced_load;
            }
        }
        println!(
            "[TENSEGRITY ENGINE] Structural tension re-balanced to {balanced_load:.2} across {active_count} active nodes."
        );
    }
}

/// Demo + assert harness for the hyper-tensegrity engine.
pub fn run_demo() {
    println!("=== Testing MICHAEL's Hyper-Computable Tensegrity Engine ===");

    let mut engine = HyperTensegrityEngine::new(4);

    println!("\n--- Test 1: Applying Normal Load ---");
    match engine.apply_point_load(0, 3.0) {
        Ok(_) => {
            println!("Load applied successfully. Node 0 status: Safe.");
            assert_eq!(engine.nodes[0].logic_state, BelnapValue::True);
            assert!((engine.nodes[0].tension_load - 4.0).abs() < 1e-5);
        }
        Err(e) => panic!("normal load should succeed: {e}"),
    }

    println!("\n--- Test 2: Applying Illegal Adversarial Point-Load ---");
    match engine.apply_point_load(1, 15.0) {
        Ok(_) => panic!("adversarial overload must buckle"),
        Err(e) => {
            println!("{e}");
            assert!(e.contains("buckled safely"));
            assert!(engine.nodes[1].is_chaitin_isolated);
            assert_eq!(engine.nodes[1].logic_state, BelnapValue::Both);
        }
    }

    println!("\n--- Test 3: Accessing Chaitin-Isolated Node ---");
    match engine.apply_point_load(1, 1.0) {
        Ok(_) => panic!("isolated node must reject further loads"),
        Err(e) => {
            println!("{e}");
            assert!(e.contains("Chaitin-uncomputable"));
        }
    }

    // Active nodes remain balanced after buckle redistribution.
    let active: Vec<_> = engine
        .nodes
        .iter()
        .filter(|n| !n.is_chaitin_isolated)
        .collect();
    assert!(!active.is_empty());
    let t0 = active[0].tension_load;
    assert!(active.iter().all(|n| (n.tension_load - t0).abs() < 1e-4));
    println!();
}
