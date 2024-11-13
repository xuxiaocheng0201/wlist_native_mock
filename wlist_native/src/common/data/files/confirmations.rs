use serde::{Deserialize, Serialize};

use crate::common::data::files::tokens::{DownloadToken, RefreshToken, UploadToken};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct RefreshConfirmation {
    pub files: u64,
    pub directories: u64,
    pub token: RefreshToken,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DownloadConfirmation {
    pub size: u64,
    pub token: DownloadToken,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UploadConfirmation {
    pub done: bool,
    pub token: UploadToken,
}
