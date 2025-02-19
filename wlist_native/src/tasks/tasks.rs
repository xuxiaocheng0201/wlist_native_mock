use std::sync::Arc;

use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum TaskState {
    Running,
    Pausing,
    Pending,
    Failed(Arc<String>),
    Cancelled,
    Complete,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TaskBase {
    pub id: i64,
    pub state: TaskState,
    pub create_time: DateTime<Utc>,
    pub origin: Option<i64>,
}

impl TaskBase {
    pub fn new(origin: Option<i64>) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Deref, DerefMut, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct RefreshTask {
    #[deref] #[deref_mut]
    pub base: TaskBase,
    pub storage: i64,
    pub directory: i64,
}

#[derive(Debug, Deref, DerefMut, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct DownloadTask {
    #[deref] #[deref_mut]
    pub base: TaskBase,
    pub storage: i64,
    pub file_id: i64,
    pub is_directory: bool,
    pub path: String,
    // TODO
}

#[derive(Debug, Deref, DerefMut, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UploadTask {
    #[deref] #[deref_mut]
    pub base: TaskBase,
    pub storage: i64,
    pub parent: i64,
    pub path: String,
    // TODO
}
