use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::data::storages::StorageType;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct StorageInformation {
    pub id: i64,
    pub name: Arc<String>,
    pub read_only: bool,
    pub storage_type: StorageType,
    pub available: bool,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub root_directory_id: i64,
}

impl StorageInformation {
    /// The default id is `-1`.
    pub fn new(name: Arc<String>, storage_type: StorageType, root_directory_id: i64) -> Self {
        Self {
            id: -1,
            name,
            read_only: storage_type.is_share(),
            storage_type,
            available: true,
            create_time: Utc::now(),
            update_time: Utc::now(),
            root_directory_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct StorageDetailsInformation {
    pub basic: StorageInformation,
    pub size: Option<u64>,
    pub indexed_size: u64,
    pub total_size: Option<u64>,
    pub upload_flow: Option<StorageFlow>, // None means unknown/infinity.
    pub download_flow: Option<StorageFlow>, // None means unknown/infinity.
    pub max_size_per_file: u64,
}

impl StorageDetailsInformation {
    #[inline]
    pub fn spare_size(&self) -> Option<u64> {
        Some(self.total_size? - self.size?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct StorageListInformation {
    pub total: u64,
    pub filtered: u64,
    pub storages: Vec<StorageInformation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct StorageFlow {
    pub start: DateTime<Utc>,
    pub refresh: DateTime<Utc>,
    pub flow: u64,
}
