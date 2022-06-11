use std::fs::File;
use std::path::Path;

use bytes::{Buf, BufMut};

use crate::{Error, Result};

/*
 * 1. write: line
 * 2. read: aggr
 * */
pub struct LogFile {
    file: File,
}

impl LogFile {
    pub fn write(&mut self, bytes: &impl Buf) {}

    pub fn read(&mut self, output: &mut impl BufMut) {}

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            file: File::open(path)?,
        })
    }
}
