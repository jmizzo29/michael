//! Constraint trait and guardrail enforcers applied to every tensor result.

use crate::tensor::Tensor;
use std::fmt;

/// A post-op check that may reject or repair a tensor before it leaves the pipeline.
pub trait Constraint {
    fn name(&self) -> &'static str;
    fn enforce(&self, tensor: &mut Tensor) -> Result<(), ConstraintError>;
}

/// Ordered pipeline of constraints. Every op result must pass through this.
#[derive(Default)]
pub struct ConstraintPipeline {
    guards: Vec<Box<dyn Constraint>>,
}

impl ConstraintPipeline {
    pub fn new() -> Self {
        Self { guards: Vec::new() }
    }

    pub fn with_defaults() -> Self {
        let mut p = Self::new();
        p.push(NoNan);
        p.push(MaxNorm::new(1.0e6));
        p.push(AllocationBound::new(16 * 1024 * 1024)); // 16M elements
        p
    }

    pub fn push<C: Constraint + 'static>(&mut self, c: C) {
        self.guards.push(Box::new(c));
    }

    /// Run all guards on `tensor`. On success, returns the same tensor.
    pub fn apply(&self, mut tensor: Tensor) -> Result<Tensor, ConstraintError> {
        for g in &self.guards {
            g.enforce(&mut tensor).map_err(|e| ConstraintError::GuardFailed {
                guard: g.name(),
                reason: e.to_string(),
            })?;
        }
        Ok(tensor)
    }
}

/// Reject any NaN or infinite element.
pub struct NoNan;

impl Constraint for NoNan {
    fn name(&self) -> &'static str {
        "no_nan"
    }

    fn enforce(&self, tensor: &mut Tensor) -> Result<(), ConstraintError> {
        for (i, &v) in tensor.data().iter().enumerate() {
            if v.is_nan() {
                return Err(ConstraintError::Nan { index: i });
            }
            if v.is_infinite() {
                return Err(ConstraintError::Infinite { index: i, value: v });
            }
        }
        Ok(())
    }
}

/// Cap L2 norm by scaling down when above `max_norm`.
pub struct MaxNorm {
    max_norm: f32,
}

impl MaxNorm {
    pub fn new(max_norm: f32) -> Self {
        Self { max_norm }
    }
}

impl Constraint for MaxNorm {
    fn name(&self) -> &'static str {
        "max_norm"
    }

    fn enforce(&self, tensor: &mut Tensor) -> Result<(), ConstraintError> {
        if !(self.max_norm.is_finite() && self.max_norm > 0.0) {
            return Err(ConstraintError::InvalidConfig(
                "max_norm must be finite and > 0",
            ));
        }
        let n = tensor.l2_norm();
        if n > self.max_norm {
            let scale = self.max_norm / n;
            tensor.scale_inplace(scale);
        }
        Ok(())
    }
}

/// Bound total element count (allocation guardrail).
pub struct AllocationBound {
    max_elements: usize,
}

impl AllocationBound {
    pub fn new(max_elements: usize) -> Self {
        Self { max_elements }
    }
}

impl Constraint for AllocationBound {
    fn name(&self) -> &'static str {
        "allocation_bound"
    }

    fn enforce(&self, tensor: &mut Tensor) -> Result<(), ConstraintError> {
        if tensor.len() > self.max_elements {
            return Err(ConstraintError::AllocationExceeded {
                got: tensor.len(),
                max: self.max_elements,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ConstraintError {
    Nan { index: usize },
    Infinite { index: usize, value: f32 },
    AllocationExceeded { got: usize, max: usize },
    InvalidConfig(&'static str),
    GuardFailed { guard: &'static str, reason: String },
}

impl fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nan { index } => write!(f, "NaN at index {index}"),
            Self::Infinite { index, value } => {
                write!(f, "non-finite value {value} at index {index}")
            }
            Self::AllocationExceeded { got, max } => {
                write!(f, "allocation {got} elements exceeds bound {max}")
            }
            Self::InvalidConfig(msg) => write!(f, "invalid constraint config: {msg}"),
            Self::GuardFailed { guard, reason } => {
                write!(f, "constraint `{guard}` failed: {reason}")
            }
        }
    }
}

impl std::error::Error for ConstraintError {}
