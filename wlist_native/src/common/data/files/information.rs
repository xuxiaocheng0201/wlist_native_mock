use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::data::files::confirmations::DownloadConfirmation;

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

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct FileDetailsInformation {
    pub basic: FileInformation,
    pub md5: Option<Arc<String>>,
    pub path: Vec<String>,
    pub thumbnail: Option<DownloadConfirmation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct FileListInformation {
    pub total_file: u64,
    pub total_directory: u64,
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
