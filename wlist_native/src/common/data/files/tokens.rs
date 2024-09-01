use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::common::data::storages::StorageType;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct RefreshToken {
    pub storage: i64,
    pub token: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DownloadToken {
    pub storage: i64,
    pub r#type: StorageType,
    pub token: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UploadToken {
    pub storage: i64,
    pub r#type: StorageType,
    pub token: Arc<String>,
}
