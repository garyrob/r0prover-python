use anyhow::Result;
use pyo3::prelude::*;
use risc0_zkvm::VerifierContext;

#[pyclass]
pub struct Segment {
    segment: risc0_zkvm::Segment,
}

impl Segment {
    pub fn new(segment: risc0_zkvm::Segment) -> Self {
        Self { segment }
    }

    pub fn prove(&self, verifier_context: &VerifierContext) -> Result<SegmentReceipt> {
        Ok(SegmentReceipt::new(self.segment.prove(verifier_context)?))
    }
}

#[pyclass]
pub struct SegmentReceipt {
    segment_receipt: risc0_zkvm::SegmentReceipt,
}

impl SegmentReceipt {
    pub fn new(segment_receipt: risc0_zkvm::SegmentReceipt) -> Self {
        Self { segment_receipt }
    }
}
