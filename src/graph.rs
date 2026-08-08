//! Dynamic Sparse Graph — arena-allocated nodes, threshold-gated edges.
//!
//! Replaces dense matrix-multiply assumptions: signal only traverses edges
//! that fire (`signal * weight > threshold`). Inactive sub-graphs are never
//! enqueued, so they incur no visit work and no extra queue slots.

use std::collections::HashMap;

pub type NodeId = usize;

/// Guardrail applied to node activations or edge routing.
pub trait GraphGuardrail {
    fn validate(&self, activation: f32) -> Result<f32, &'static str>;
}

/// A node in the arena (`Vec<Node>` indexed by [`NodeId`]).
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub bias: f32,
    pub current_activation: f32,
    /// Hard upper bound on activation after ReLU.
    pub max_cap: f32,
    /// How many times this node was entered during propagation.
    pub visit_count: u32,
}

impl Node {
    /// Hard guardrails baked into every node:
    /// NaN/Inf rejection → ReLU(`signal + bias`) → clamp to `max_cap`.
    pub fn apply_guardrails(&self, signal: f32) -> f32 {
        if signal.is_nan() || signal.is_infinite() {
            return 0.0;
        }
        let shifted = signal + self.bias;
        if shifted.is_nan() || shifted.is_infinite() {
            return 0.0;
        }
        let relu = shifted.max(0.0);
        if !self.max_cap.is_finite() || self.max_cap < 0.0 {
            return 0.0;
        }
        relu.min(self.max_cap)
    }
}

/// Weighted connection; fires only when `signal * weight > threshold`.
pub struct Edge {
    #[allow(dead_code)]
    pub source: NodeId,
    pub target: NodeId,
    pub weight: f32,
    pub threshold: f32,
}

impl GraphGuardrail for Edge {
    fn validate(&self, signal: f32) -> Result<f32, &'static str> {
        if signal.is_nan() || signal.is_infinite() {
            return Err("non-finite signal");
        }
        let routed = signal * self.weight;
        if routed.is_nan() || routed.is_infinite() {
            return Err("non-finite routed signal");
        }
        if routed > self.threshold {
            Ok(routed)
        } else {
            Err("below routing threshold")
        }
    }
}

/// Sparse graph engine: arena of nodes + adjacency map by source [`NodeId`].
pub struct MichaelGraph {
    /// Arena: `nodes[id]` is the node with that [`NodeId`].
    pub nodes: Vec<Node>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

impl MichaelGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    /// Allocate a node in the arena; returns its index-based [`NodeId`].
    pub fn add_node(&mut self, label: &str, bias: f32, max_cap: f32) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Node {
            id,
            label: label.to_string(),
            bias,
            current_activation: 0.0,
            max_cap,
            visit_count: 0,
        });
        id
    }

    pub fn add_edge(&mut self, source: NodeId, target: NodeId, weight: f32, threshold: f32) {
        debug_assert!(source < self.nodes.len(), "source NodeId out of arena");
        debug_assert!(target < self.nodes.len(), "target NodeId out of arena");
        self.edges.entry(source).or_default().push(Edge {
            source,
            target,
            weight,
            threshold,
        });
    }

    pub fn clear_activations(&mut self) {
        for node in &mut self.nodes {
            node.current_activation = 0.0;
            node.visit_count = 0;
        }
    }

    /// Dynamic forward propagation with per-node hard guardrails and
    /// threshold-gated sparse routing. Only firing edges are pushed onto the
    /// work stack — inactive sub-graphs are never visited or allocated for.
    pub fn propagate(&mut self, entry_node: NodeId, input_signal: f32) -> PropagateStats {
        let mut stats = PropagateStats::default();
        if entry_node >= self.nodes.len() {
            return stats;
        }

        // Work stack grows only for edges that fire.
        let mut stack: Vec<(NodeId, f32)> = Vec::with_capacity(4);
        stack.push((entry_node, input_signal));

        while let Some((current_id, signal)) = stack.pop() {
            stats.nodes_processed += 1;

            if current_id >= self.nodes.len() {
                continue;
            }

            let guarded = {
                let node = &self.nodes[current_id];
                node.apply_guardrails(signal)
            };

            {
                let node = &mut self.nodes[current_id];
                node.current_activation = guarded;
                node.visit_count = node.visit_count.saturating_add(1);
            }

            if guarded <= 0.0 {
                stats.signals_collapsed += 1;
                continue;
            }

            // Snapshot edge count; read each edge by index so we never hold a
            // long-lived borrow across a `stack.push` (and skip non-firing edges).
            let edge_len = self
                .edges
                .get(&current_id)
                .map(|v| v.len())
                .unwrap_or(0);

            for i in 0..edge_len {
                let (target, weight, threshold) = {
                    let edge = &self.edges[&current_id][i];
                    (edge.target, edge.weight, edge.threshold)
                };
                // Reconstruct edge view for GraphGuardrail::validate.
                let edge = Edge {
                    source: current_id,
                    target,
                    weight,
                    threshold,
                };
                stats.edges_examined += 1;
                match edge.validate(guarded) {
                    Ok(routed) => {
                        stats.edges_fired += 1;
                        stack.push((target, routed));
                    }
                    Err(_) => {
                        stats.edges_gated += 1;
                        // Inactive: no push → no visit → no extra stack memory.
                    }
                }
            }

            stats.peak_stack = stats.peak_stack.max(stack.len());
        }

        stats
    }
}

impl Default for MichaelGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Observability for sparse skip / allocation behavior.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PropagateStats {
    pub nodes_processed: usize,
    pub edges_examined: usize,
    pub edges_fired: usize,
    pub edges_gated: usize,
    pub signals_collapsed: usize,
    pub peak_stack: usize,
}
