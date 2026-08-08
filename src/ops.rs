//! Raw arithmetic and matrix ops. Every result exits through the constraint pipeline.

use crate::constraints::{ConstraintError, ConstraintPipeline};
use crate::tensor::{Tensor, TensorError};
use std::fmt;

#[derive(Debug)]
pub enum OpError {
    Tensor(TensorError),
    Constraint(ConstraintError),
    ShapeMismatch { left: Vec<usize>, right: Vec<usize> },
    MatMulRank,
    InnerDimMismatch { a_k: usize, b_k: usize },
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tensor(e) => write!(f, "{e}"),
            Self::Constraint(e) => write!(f, "{e}"),
            Self::ShapeMismatch { left, right } => {
                write!(f, "shape mismatch {:?} vs {:?}", left, right)
            }
            Self::MatMulRank => write!(f, "matmul requires 2D tensors"),
            Self::InnerDimMismatch { a_k, b_k } => {
                write!(f, "matmul inner dims {a_k} != {b_k}")
            }
        }
    }
}

impl std::error::Error for OpError {}

impl From<TensorError> for OpError {
    fn from(e: TensorError) -> Self {
        Self::Tensor(e)
    }
}

impl From<ConstraintError> for OpError {
    fn from(e: ConstraintError) -> Self {
        Self::Constraint(e)
    }
}

/// Operator surface that always seals results with `pipeline`.
pub struct Ops<'a> {
    pipeline: &'a ConstraintPipeline,
}

impl<'a> Ops<'a> {
    pub fn new(pipeline: &'a ConstraintPipeline) -> Self {
        Self { pipeline }
    }

    fn seal(&self, tensor: Tensor) -> Result<Tensor, OpError> {
        Ok(self.pipeline.apply(tensor)?)
    }

    /// Element-wise add (same shape).
    pub fn add(&self, a: &Tensor, b: &Tensor) -> Result<Tensor, OpError> {
        if a.shape() != b.shape() {
            return Err(OpError::ShapeMismatch {
                left: a.shape().to_vec(),
                right: b.shape().to_vec(),
            });
        }
        let data: Vec<f32> = a
            .data()
            .iter()
            .zip(b.data().iter())
            .map(|(x, y)| x + y)
            .collect();
        let out = Tensor::new(a.shape().to_vec(), data)?;
        self.seal(out)
    }

    /// Element-wise subtract.
    pub fn sub(&self, a: &Tensor, b: &Tensor) -> Result<Tensor, OpError> {
        if a.shape() != b.shape() {
            return Err(OpError::ShapeMismatch {
                left: a.shape().to_vec(),
                right: b.shape().to_vec(),
            });
        }
        let data: Vec<f32> = a
            .data()
            .iter()
            .zip(b.data().iter())
            .map(|(x, y)| x - y)
            .collect();
        let out = Tensor::new(a.shape().to_vec(), data)?;
        self.seal(out)
    }

    /// Element-wise multiply.
    pub fn mul(&self, a: &Tensor, b: &Tensor) -> Result<Tensor, OpError> {
        if a.shape() != b.shape() {
            return Err(OpError::ShapeMismatch {
                left: a.shape().to_vec(),
                right: b.shape().to_vec(),
            });
        }
        let data: Vec<f32> = a
            .data()
            .iter()
            .zip(b.data().iter())
            .map(|(x, y)| x * y)
            .collect();
        let out = Tensor::new(a.shape().to_vec(), data)?;
        self.seal(out)
    }

    /// Scalar multiply.
    pub fn scale(&self, a: &Tensor, factor: f32) -> Result<Tensor, OpError> {
        let data: Vec<f32> = a.data().iter().map(|x| x * factor).collect();
        let out = Tensor::new(a.shape().to_vec(), data)?;
        self.seal(out)
    }

    /// Dense matrix multiply for 2D tensors: `[m,k] @ [k,n] -> [m,n]`.
    pub fn matmul(&self, a: &Tensor, b: &Tensor) -> Result<Tensor, OpError> {
        if a.ndim() != 2 || b.ndim() != 2 {
            return Err(OpError::MatMulRank);
        }
        let m = a.shape()[0];
        let k = a.shape()[1];
        let k2 = b.shape()[0];
        let n = b.shape()[1];
        if k != k2 {
            return Err(OpError::InnerDimMismatch { a_k: k, b_k: k2 });
        }

        let mut data = vec![0.0f32; m * n];
        let a_d = a.data();
        let b_d = b.data();
        for i in 0..m {
            for j in 0..n {
                let mut acc = 0.0f32;
                for t in 0..k {
                    acc += a_d[i * k + t] * b_d[t * n + j];
                }
                data[i * n + j] = acc;
            }
        }
        let out = Tensor::new(vec![m, n], data)?;
        self.seal(out)
    }

    /// Construct a tensor and seal it through the pipeline (construction path).
    pub fn tensor(&self, shape: Vec<usize>, data: Vec<f32>) -> Result<Tensor, OpError> {
        let t = Tensor::new(shape, data)?;
        self.seal(t)
    }
}
