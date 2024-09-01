use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::data::files::FileLocation;
use crate::common::data::files::information::FileInformation;

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

impl TrashInformation {
    pub fn get_location(&self, storage_id: i64) -> FileLocation {
        FileLocation {
            storage: storage_id,
            file_id: self.id,
            is_directory: self.is_directory,
        }
    }

    pub fn as_file(&self, parent_id: i64) -> FileInformation {
        FileInformation {
            id: self.id,
            parent_id,
            name: self.name.clone(),
            is_directory: self.is_directory,
            size: self.size,
            create_time: self.create_time,
            update_time: self.update_time,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct TrashDetailsInformation {
    pub basic: TrashInformation,
    pub md5: Option<Arc<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TrashListInformation {
    pub total: u64,
    pub filtered: u64,
    pub files: Vec<TrashInformation>,
}
