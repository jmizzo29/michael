//! Resonant wave nodes — phase-coupled interference instead of dense matmul.

use std::f32::consts::PI;

pub struct WaveNode {
    #[allow(dead_code)]
    pub id: usize,
    /// Internal oscillation speed (omega).
    pub frequency: f32,
    /// Current phase angle in radians (0 to 2π).
    pub phase: f32,
    pub amplitude: f32,
}

impl WaveNode {
    pub fn new(id: usize, frequency: f32) -> Self {
        Self {
            id,
            frequency,
            phase: 0.0,
            amplitude: 1.0,
        }
    }

    /// Step the node's internal oscillator forward in time.
    pub fn tick(&mut self, dt: f32) {
        self.phase = wrap_phase(self.phase + self.frequency * dt);
    }
}

pub struct ResonantEdge {
    #[allow(dead_code)]
    pub source: usize,
    #[allow(dead_code)]
    pub target: usize,
    /// Impedance: delays wave arrival phase.
    pub phase_shift: f32,
    /// Tracks how long the signal has been constructive.
    pub stability_counter: i32,
}

impl ResonantEdge {
    pub fn new(source: usize, target: usize, phase_shift: f32) -> Self {
        Self {
            source,
            target,
            phase_shift,
            stability_counter: 0,
        }
    }

    /// Compute wave interference at the target node.
    pub fn interfere(&mut self, source_node: &WaveNode, target_node: &mut WaveNode) -> f32 {
        let arrived_phase = wrap_phase(source_node.phase + self.phase_shift);
        let phase_difference = (arrived_phase - target_node.phase).abs();

        // Constructive interference if phase difference is near 0 or 2π.
        let resonance = (phase_difference.cos() + 1.0) / 2.0; // Normalized 0.0 to 1.0

        if resonance > 0.85 {
            // High constructive resonance → reinforce path stability.
            self.stability_counter += 1;
        } else if resonance < 0.2 {
            // Destructive interference → degrade path.
            self.stability_counter -= 1;
        }

        resonance * source_node.amplitude
    }
}

/// Wrap phase into `[0, 2π)`.
fn wrap_phase(phase: f32) -> f32 {
    let two_pi = 2.0 * PI;
    let mut p = phase % two_pi;
    if p < 0.0 {
        p += two_pi;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_advances_and_wraps() {
        let mut n = WaveNode::new(0, PI); // half-turn per unit time
        n.tick(1.0);
        assert!((n.phase - PI).abs() < 1e-5);
        n.tick(1.0);
        assert!(n.phase.abs() < 1e-5 || (n.phase - 2.0 * PI).abs() < 1e-5);
    }
}
