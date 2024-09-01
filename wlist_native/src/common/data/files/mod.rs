use serde::{Deserialize, Serialize};

pub mod tokens;
pub mod confirmations;
pub mod information;
pub mod progresses;
pub mod options;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct FileLocation {
    pub storage: i64,
    pub file_id: i64,
    pub is_directory: bool,
}
