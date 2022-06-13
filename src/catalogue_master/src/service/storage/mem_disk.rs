use std::fs::File;
use std::path::Path;

use crate::{Error, Result};

pub struct MemDisk {
    file: File,
}

impl MemDisk {
    fn load(&mut self) {}

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            file: File::open(path)?,
        })
    }
}
