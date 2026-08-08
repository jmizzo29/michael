//! Core tensor: shape, strides, and contiguous `f32` storage.

use std::fmt;

/// Dense, row-major tensor backed by a contiguous `Vec<f32>`.
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    shape: Vec<usize>,
    strides: Vec<usize>,
    data: Vec<f32>,
}

impl Tensor {
    /// Build a tensor from shape and flat data. Data length must equal product of shape.
    pub fn new(shape: Vec<usize>, data: Vec<f32>) -> Result<Self, TensorError> {
        let expected = shape.iter().try_fold(1usize, |acc, &d| {
            acc.checked_mul(d).ok_or(TensorError::ShapeOverflow)
        })?;
        if data.len() != expected {
            return Err(TensorError::LengthMismatch {
                expected,
                got: data.len(),
            });
        }
        let strides = row_major_strides(&shape);
        Ok(Self {
            shape,
            strides,
            data,
        })
    }

    /// Zeros with the given shape.
    #[allow(dead_code)]
    pub fn zeros(shape: Vec<usize>) -> Result<Self, TensorError> {
        let n = shape.iter().try_fold(1usize, |acc, &d| {
            acc.checked_mul(d).ok_or(TensorError::ShapeOverflow)
        })?;
        Self::new(shape, vec![0.0; n])
    }

    /// Fill every element with `value`.
    #[allow(dead_code)]
    pub fn filled(shape: Vec<usize>, value: f32) -> Result<Self, TensorError> {
        let n = shape.iter().try_fold(1usize, |acc, &d| {
            acc.checked_mul(d).ok_or(TensorError::ShapeOverflow)
        })?;
        Self::new(shape, vec![value; n])
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn strides(&self) -> &[usize] {
        &self.strides
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    #[allow(dead_code)]
    pub fn data_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// L2 norm of the flattened tensor.
    pub fn l2_norm(&self) -> f32 {
        self.data
            .iter()
            .map(|x| x * x)
            .sum::<f32>()
            .sqrt()
    }

    /// Scale every element in place.
    pub fn scale_inplace(&mut self, factor: f32) {
        for x in &mut self.data {
            *x *= factor;
        }
    }
}

/// Row-major strides for `shape`.
pub fn row_major_strides(shape: &[usize]) -> Vec<usize> {
    let mut strides = vec![1usize; shape.len()];
    for i in (0..shape.len().saturating_sub(1)).rev() {
        strides[i] = strides[i + 1].saturating_mul(shape[i + 1]);
    }
    strides
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensorError {
    LengthMismatch { expected: usize, got: usize },
    ShapeOverflow,
}

impl fmt::Display for TensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthMismatch { expected, got } => {
                write!(f, "data length {got} does not match shape product {expected}")
            }
            Self::ShapeOverflow => write!(f, "shape product overflowed usize"),
        }
    }
}

impl std::error::Error for TensorError {}
