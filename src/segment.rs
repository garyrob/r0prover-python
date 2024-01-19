use crate::serialization::Pickleable;
use anyhow::Result;
use pyo3::prelude::*;
use risc0_zkvm::VerifierContext;
use serde::{Deserialize, Serialize};

#[pyclass(module = "l2_r0prover")]
#[derive(Serialize, Deserialize, Clone)]
pub struct Segment {
    segment: Option<risc0_zkvm::Segment>,
}

impl Segment {
    pub fn new(segment: risc0_zkvm::Segment) -> Self {
        Self {
            segment: Some(segment),
        }
    }

    pub fn prove(&self, verifier_context: &VerifierContext) -> Result<SegmentReceipt> {
        Ok(SegmentReceipt::new(
            self.segment.as_ref().unwrap().prove(verifier_context)?,
        ))
    }
}

impl Pickleable for Segment {}

#[pymethods]
impl Segment {
    #[new]
    fn new_init() -> Self {
        Self { segment: None }
    }

    fn __getstate__(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.to_bytes(py)
    }

    fn __setstate__(&mut self, py: Python<'_>, state: PyObject) -> PyResult<()> {
        *self = Self::from_bytes(state, py)?;
        Ok(())
    }
}

#[pyclass(module = "l2_r0prover")]
#[derive(Serialize, Deserialize, Clone)]
pub struct SegmentReceipt {
    segment_receipt: Option<risc0_zkvm::SegmentReceipt>,
}

impl SegmentReceipt {
    pub fn new(segment_receipt: risc0_zkvm::SegmentReceipt) -> Self {
        Self {
            segment_receipt: Some(segment_receipt),
        }
    }

    pub fn get_segment_receipt_ref(&self) -> &risc0_zkvm::SegmentReceipt {
        &self.segment_receipt.as_ref().unwrap()
    }
}

impl Pickleable for SegmentReceipt {}

#[pymethods]
impl SegmentReceipt {
    #[new]
    fn new_init() -> Self {
        Self {
            segment_receipt: None,
        }
    }

    fn __getstate__(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.to_bytes(py)
    }

    fn __setstate__(&mut self, py: Python<'_>, state: PyObject) -> PyResult<()> {
        *self = Self::from_bytes(state, py)?;
        Ok(())
    }
}
