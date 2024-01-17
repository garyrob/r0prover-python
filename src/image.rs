use anyhow::Result;
use pyo3::prelude::*;
use risc0_binfmt::{MemoryImage, Program};
use risc0_zkvm_platform::memory::GUEST_MAX_MEM;
use risc0_zkvm_platform::PAGE_SIZE;

#[pyclass]
pub struct Image {
    memory_image: MemoryImage,
}

impl Image {
    pub fn from_elf(elf: &[u8]) -> Result<Self> {
        let program = Program::load_elf(elf, GUEST_MAX_MEM as u32)?;
        let image = MemoryImage::new(&program, PAGE_SIZE as u32)?;
        Ok(Self {
            memory_image: image,
        })
    }

    pub fn get_image(&self) -> MemoryImage {
        self.memory_image.clone()
    }
}
