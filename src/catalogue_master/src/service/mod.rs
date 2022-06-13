mod storage;

use crate::*;

pub trait Service {
    fn find(path: &Path) -> Result<File>;
}
