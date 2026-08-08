//! MICHAEL-BIO: DNA toehold-mediated strand displacement (DSD) wetware engine.
//! Analog mass-action kinetics — no digital matmul, no numpy.

/// Biological DNA toehold-mediated strand displacement gate (non-linear bio-neural).
pub struct DnaStrandDisplacementGate {
    /// Toehold binding constant (M⁻¹ s⁻¹).
    pub k_on: f64,
    /// Branch migration constant (s⁻¹).
    pub k_b: f64,
    /// [Input DNA] molar.
    pub input_strand: f64,
    /// [Fuel DNA Gate] molar.
    pub gate_complex: f64,
    /// [Output DNA] molar.
    pub displaced_output: f64,
    /// [Inert Waste] molar.
    pub waste_product: f64,
}

impl DnaStrandDisplacementGate {
    pub fn new(toehold_binding_rate: f64, branch_migration_rate: f64) -> Self {
        Self {
            k_on: toehold_binding_rate,
            k_b: branch_migration_rate,
            input_strand: 0.0,
            gate_complex: 1.0e-6,
            displaced_output: 0.0,
            waste_product: 0.0,
        }
    }

    /// Injects a volumetric concentration of input DNA strands.
    pub fn inject_input_concentration(&mut self, concentration_molar: f64) {
        self.input_strand += concentration_molar;
    }

    /// Solves mass-action kinetic ODEs for DNA displacement over `time_seconds`.
    pub fn simulate_kinetics(&mut self, time_seconds: f64, steps: usize) {
        let dt = time_seconds / steps as f64;
        for _ in 0..steps {
            // Rate = k_on * [Input] * [Gate]
            let reaction_rate = self.k_on * self.input_strand * self.gate_complex;
            // Non-linear enzymatic saturation limit.
            let effective_rate = reaction_rate.min(self.k_b * self.gate_complex);

            let delta_conc = effective_rate * dt;
            self.input_strand = (self.input_strand - delta_conc).max(0.0);
            self.gate_complex = (self.gate_complex - delta_conc).max(0.0);
            self.displaced_output += delta_conc;
            self.waste_product += delta_conc;
        }
    }
}

impl Default for DnaStrandDisplacementGate {
    fn default() -> Self {
        Self::new(1e6, 1e4)
    }
}

/// Cascaded DNA strand-displacement inference chip.
pub struct MichaelWetwareEngine {
    pub nodes: Vec<DnaStrandDisplacementGate>,
}

impl MichaelWetwareEngine {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            nodes: (0..num_nodes)
                .map(|_| DnaStrandDisplacementGate::default())
                .collect(),
        }
    }

    pub fn process_prompt_chemical(&mut self, prompt: &str) -> Vec<f64> {
        println!(
            "\n[MICHAEL-BIO] Transducing Prompt '{prompt}' into DNA Microfluidic Ingress..."
        );

        // 1. Convert prompt string to molar concentrations.
        for (idx, &b) in prompt.as_bytes().iter().enumerate() {
            let target_node = idx % self.nodes.len();
            let molar_input = (f64::from(b) / 255.0) * 1.0e-6; // micromolar scale
            self.nodes[target_node].inject_input_concentration(molar_input);
        }

        // 2. Run biochemical reaction kinetics across the chip.
        println!(
            "[MICHAEL-BIO] Initiating Toehold Displacement Kinetics in Wetware Chamber..."
        );
        let mut output_concentrations = Vec::with_capacity(self.nodes.len());
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.simulate_kinetics(0.5, 50);
            output_concentrations.push(node.displaced_output);
            println!(
                "  └── Node #{i} Output DNA Concentration: {:.4} µM",
                node.displaced_output * 1e6
            );
        }

        output_concentrations
    }

    /// Decodes microfluidic fluorescence outflow concentrations into text.
    pub fn decode_concentration_to_text(&self, concentrations: &[f64]) -> String {
        concentrations
            .iter()
            .map(|&conc| {
                let normalized = conc / 1.0e-6;
                let char_code = ((normalized * 200.0) as i64).rem_euclid(95) as u8 + 32;
                char::from(char_code)
            })
            .collect()
    }
}

impl Default for MichaelWetwareEngine {
    fn default() -> Self {
        Self::new(4)
    }
}

/// Demo harness for the synthetic bio-chemical wetware system.
pub fn run_demo() {
    println!("=== Testing MICHAEL-BIO DNA Strand Displacement Wetware ===");

    let mut wetware = MichaelWetwareEngine::new(4);
    let prompt = "MICHAEL_GENESIS";

    let output_concentrations = wetware.process_prompt_chemical(prompt);
    assert_eq!(output_concentrations.len(), 4);
    assert!(
        output_concentrations.iter().any(|&c| c > 0.0),
        "displacement must produce output DNA"
    );

    let generated_text = wetware.decode_concentration_to_text(&output_concentrations);
    assert_eq!(generated_text.chars().count(), 4);

    println!("\n[MICHAEL-BIO] Chemical Equilibrium Reached.");
    println!("[MICHAEL-BIO] Decoded Nanopore Spectrum Output: '{generated_text}'");
    println!();
}
