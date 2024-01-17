use anyhow::Result;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct ExitCode {
    exit_code: risc0_zkvm::ExitCode,
}

impl ExitCode {
    pub fn new(exit_code: risc0_zkvm::ExitCode) -> Self {
        Self { exit_code }
    }
}

#[pymethods]
impl ExitCode {
    pub fn is_system_split(&self) -> PyResult<bool> {
        Ok(matches!(self.exit_code, risc0_zkvm::ExitCode::SystemSplit))
    }

    pub fn is_session_limit(&self) -> PyResult<bool> {
        Ok(matches!(self.exit_code, risc0_zkvm::ExitCode::SessionLimit))
    }

    pub fn is_paused(&self) -> PyResult<bool> {
        Ok(matches!(self.exit_code, risc0_zkvm::ExitCode::Paused(_)))
    }

    pub fn get_paused_code(&self) -> PyResult<u32> {
        match self.exit_code {
            risc0_zkvm::ExitCode::Paused(v) => Ok(v),
            _ => Err(PyValueError::new_err("The exit code is not for pausing.")),
        }
    }

    pub fn is_halted(&self) -> PyResult<bool> {
        Ok(matches!(self.exit_code, risc0_zkvm::ExitCode::Halted(_)))
    }

    pub fn get_halted_code(&self) -> PyResult<u32> {
        match self.exit_code {
            risc0_zkvm::ExitCode::Halted(v) => Ok(v),
            _ => Err(PyValueError::new_err("The exit code is not for halting.")),
        }
    }

    pub fn is_fault(&self) -> PyResult<bool> {
        Ok(matches!(self.exit_code, risc0_zkvm::ExitCode::Fault))
    }
}

#[pyclass]
pub struct SessionInfo {
    journal: Vec<u8>,
    exit_code: ExitCode,
}

impl SessionInfo {
    pub fn new(session: &risc0_zkvm::Session) -> Result<Self> {
        let journal = match &session.journal {
            Some(v) => v.bytes.clone(),
            None => vec![],
        };
        Ok(Self {
            journal,
            exit_code: ExitCode::new(session.exit_code),
        })
    }
}

#[pymethods]
impl SessionInfo {
    pub fn get_journal(&self) -> PyResult<Vec<u8>> {
        Ok(self.journal.clone())
    }

    pub fn get_exit_code(&self) -> PyResult<ExitCode> {
        Ok(self.exit_code.clone())
    }
}
