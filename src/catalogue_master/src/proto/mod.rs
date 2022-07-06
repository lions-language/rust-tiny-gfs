pub mod chunk_handler;

pub use chunk_handler::*;

impl HeartbeatResponse {
    pub fn new_ok() -> Self {
        Self {
            code: 0,
            msg: "success".into(),
        }
    }
}
