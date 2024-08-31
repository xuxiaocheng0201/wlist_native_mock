use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RefreshProgress {
    pub loaded_files: u64,
    pub loaded_directories: u64,
    pub total_files: u64,
    pub total_directories: u64,
}
