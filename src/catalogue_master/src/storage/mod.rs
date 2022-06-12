mod mem_disk;

use crate::*;

pub trait Storage {
    fn find(path: &Path) -> Result<File>;
}
