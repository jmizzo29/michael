//! Output schema + security guardrails for MICHAEL model invocations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MichaelOutput {
    pub intent: String,
    pub payload: serde_json::Value,
    pub confidence_score: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuardrailFailure {
    PromptInjectionDetected,
    SchemaValidationError(String),
    LowGroundednessScore(f32),
    PIILeakage,
}

impl std::fmt::Display for GuardrailFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PromptInjectionDetected => write!(f, "prompt injection detected"),
            Self::SchemaValidationError(e) => write!(f, "schema validation error: {e}"),
            Self::LowGroundednessScore(s) => write!(f, "low groundedness score: {s}"),
            Self::PIILeakage => write!(f, "PII leakage detected"),
        }
    }
}

impl std::error::Error for GuardrailFailure {}

pub struct MichaelGuardrails {
    pub min_groundedness_score: f32,
    pub max_retry_attempts: u8,
}

impl MichaelGuardrails {
    pub fn new() -> Self {
        Self {
            min_groundedness_score: 0.85,
            max_retry_attempts: 3,
        }
    }

    /// Tier 1: Screen user prompt before cognitive execution.
    pub fn validate_input(&self, input: &str) -> Result<(), GuardrailFailure> {
        let lower = input.to_ascii_lowercase();
        if lower.contains("ignore previous instructions") || lower.contains("system prompt") {
            return Err(GuardrailFailure::PromptInjectionDetected);
        }
        if looks_like_pii(input) {
            return Err(GuardrailFailure::PIILeakage);
        }
        Ok(())
    }

    /// Tier 3 & 4: Validate output schema and groundedness score.
    pub fn validate_output(&self, raw_response: &str) -> Result<MichaelOutput, GuardrailFailure> {
        // 1. Schema validation
        let parsed: MichaelOutput = serde_json::from_str(raw_response)
            .map_err(|e| GuardrailFailure::SchemaValidationError(e.to_string()))?;

        // 2. Groundedness score check
        if parsed.confidence_score < self.min_groundedness_score {
            return Err(GuardrailFailure::LowGroundednessScore(
                parsed.confidence_score,
            ));
        }

        // 3. PII scan on serialized payload text
        let payload_text = parsed.payload.to_string();
        if looks_like_pii(&payload_text) || looks_like_pii(&parsed.intent) {
            return Err(GuardrailFailure::PIILeakage);
        }

        Ok(parsed)
    }

    /// Self-correction retry orchestrator.
    pub async fn execute_with_rails<F, Fut>(
        &self,
        input: &str,
        mut invoke_model: F,
    ) -> Result<MichaelOutput, String>
    where
        F: FnMut(Option<&str>) -> Fut,
        Fut: std::future::Future<Output = String>,
    {
        self.validate_input(input)
            .map_err(|_| "Input security violation".to_string())?;

        let mut feedback: Option<String> = None;
        for _attempt in 0..self.max_retry_attempts {
            let response = invoke_model(feedback.as_deref()).await;
            match self.validate_output(&response) {
                Ok(valid_output) => return Ok(valid_output),
                Err(GuardrailFailure::SchemaValidationError(err)) => {
                    feedback = Some(format!(
                        "Previous output failed schema validation: {err}. Fix JSON format."
                    ));
                }
                Err(GuardrailFailure::LowGroundednessScore(score)) => {
                    feedback = Some(format!(
                        "Confidence too low ({score:.2}). Provide grounded facts only."
                    ));
                }
                Err(_) => break,
            }
        }
        Err("Exceeded maximum guardrail self-correction attempts.".to_string())
    }
}

impl Default for MichaelGuardrails {
    fn default() -> Self {
        Self::new()
    }
}

fn looks_like_pii(text: &str) -> bool {
    // Minimal heuristic: SSN-shaped or email-shaped tokens.
    text.split_whitespace().any(|tok| {
        let t = tok.trim_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '.' && c != '-');
        is_email_like(t) || is_ssn_like(t)
    })
}

fn is_email_like(tok: &str) -> bool {
    let at = match tok.find('@') {
        Some(i) => i,
        None => return false,
    };
    let dot = match tok[at + 1..].find('.') {
        Some(i) => at + 1 + i,
        None => return false,
    };
    at > 0 && dot + 1 < tok.len()
}

fn is_ssn_like(tok: &str) -> bool {
    let b = tok.as_bytes();
    b.len() == 11
        && b[3] == b'-'
        && b[6] == b'-'
        && b[0..3].iter().all(|c| c.is_ascii_digit())
        && b[4..6].iter().all(|c| c.is_ascii_digit())
        && b[7..11].iter().all(|c| c.is_ascii_digit())
}
