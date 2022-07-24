pub(crate) struct FileMgr {}

impl FileMgr {
    pub(crate) fn create_file(&mut self) {
        // 1. file is exists (read from metadata)
        // - exist:
        //  - read finish marker
        //      - finish: return error
        //      - init state / writing state: return success
        // - not exist:
        //  - alloc balance chunks
        //  - create metadata
        unimplemented!();
    }

    pub(crate) fn new() -> Self {
        Self {}
    }
}
