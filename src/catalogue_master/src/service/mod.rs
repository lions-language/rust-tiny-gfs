mod storage;

use crate::*;

pub(crate) trait Service {
    fn find(path: &Path) -> Result<File>;
}
