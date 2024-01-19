use crate::serialization::Pickleable;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass(module = "l2_r0prover")]
#[derive(Clone, Serialize, Deserialize)]
pub struct SuccinctReceipt {
    succinct_receipt: Option<risc0_zkvm::SuccinctReceipt>,
}

impl SuccinctReceipt {
    pub fn new(succinct_receipt: risc0_zkvm::SuccinctReceipt) -> Self {
        Self {
            succinct_receipt: Some(succinct_receipt),
        }
    }

    pub fn get_succinct_receipt_ref(&self) -> &risc0_zkvm::SuccinctReceipt {
        &self.succinct_receipt.as_ref().unwrap()
    }
}

impl Pickleable for SuccinctReceipt {}

#[pymethods]
impl SuccinctReceipt {
    #[new]
    fn new_init() -> Self {
        Self {
            succinct_receipt: None,
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
