use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::data::files::confirmations::DownloadConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::trashes::information::TrashInformation;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct FileInformation {
    pub id: i64,
    pub parent_id: i64,
    pub name: Arc<String>,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}

impl FileInformation {
    pub fn get_location(&self, storage_id: i64) -> FileLocation {
        FileLocation {
            storage: storage_id,
            file_id: self.id,
            is_directory: self.is_directory,
        }
    }

    pub fn get_parent_location(&self, storage_id: i64) -> FileLocation {
        FileLocation {
            storage: storage_id,
            file_id: self.parent_id,
            is_directory: true,
        }
    }

    pub fn as_trash(&self, trash_time: Option<DateTime<Utc>>) -> TrashInformation {
        TrashInformation {
            id: self.id,
            name: self.name.clone(),
            is_directory: self.is_directory,
            size: self.size,
            create_time: self.create_time,
            update_time: self.update_time,
            trash_time,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct FileDetailsInformation {
    pub basic: FileInformation,
    pub md5: Option<Arc<String>>,
    pub path: Vec<String>,
    pub thumbnail: Option<DownloadConfirmation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct FileListInformation {
    pub total: u64,
    pub filtered: u64,
    pub files: Vec<FileInformation>,
}


#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DownloadChunkInformation {
    pub range: bool,
    pub start: u64,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DownloadInformation {
    pub chunks: Vec<DownloadChunkInformation>,
    pub expire: DateTime<Utc>,
}


#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UploadChunkInformation {
    pub start: u64,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UploadInformation {
    pub chunks: Vec<UploadChunkInformation>,
    pub expire: DateTime<Utc>,
}


#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ShareInformation {
    pub id: Arc<String>,
    pub password: Option<Arc<String>>,
    pub expire: Option<DateTime<Utc>>,
}
