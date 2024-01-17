mod image;
mod segment;
mod session;

use crate::image::Image;
use crate::segment::{Segment, SegmentReceipt};
use crate::session::SessionInfo;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use risc0_zkvm::{ExecutorEnv, ExecutorImpl, FileSegmentRef, SimpleSegmentRef, VerifierContext};

#[pyfunction]
fn load_image_from_elf(elf: &PyBytes) -> PyResult<Image> {
    Ok(Image::from_elf(elf.as_bytes())?)
}

#[pyfunction]
fn execute_with_input(image: &Image, input: &PyBytes) -> PyResult<(Vec<Segment>, SessionInfo)> {
    let env = ExecutorEnv::builder()
        .write_slice(input.as_bytes())
        .build()?;

    let mut exec = ExecutorImpl::new(env, image.get_image())?;

    let time = std::time::Instant::now();
    let session = exec.run()?;

    let mut segments = vec![];
    for segment_ref in session.segments.iter() {
        segments.push(Segment::new(segment_ref.resolve()?));
    }

    let session_info = SessionInfo::new(&session)?;
    Ok((segments, session_info))
}

#[pyfunction]
fn prove_segment(segment: &Segment) -> PyResult<SegmentReceipt> {
    let verifier_context = VerifierContext::default();
    let res = segment.prove(&verifier_context)?;
    Ok(res)
}

#[pymodule]
fn l2_r0prover(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_image_from_elf, m)?)?;
    m.add_function(wrap_pyfunction!(execute_with_input, m)?)?;
    m.add_function(wrap_pyfunction!(prove_segment, m)?)?;
    Ok(())
}
