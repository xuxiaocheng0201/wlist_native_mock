use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TrashInformation {
    pub id: i64,
    pub name: Arc<String>,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
    pub trash_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct TrashDetailsInformation {
    pub basic: TrashInformation,
    pub md5: Option<Arc<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TrashListInformation {
    pub total_file: u64,
    pub total_directory: u64,
    pub files: Vec<TrashInformation>,
}
