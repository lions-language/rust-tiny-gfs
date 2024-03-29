pub(crate) mod catalogue;
pub mod chunk_handler;

pub(crate) use catalogue::*;
pub use chunk_handler::*;

impl HeartbeatResponse {
    pub fn new_ok() -> Self {
        Self {
            code: 0,
            msg: "success".into(),
        }
    }
}

impl RegisterResponse {
    pub fn new_ok(chunk_id: String) -> Self {
        Self {
            chunk_id: chunk_id,
            code: 0,
            msg: "success".into(),
        }
    }
}

impl CreateFileResponse {
    pub fn new_ok() -> Self {
        Self {
            code: 0,
            msg: "success".into(),
        }
    }
}
