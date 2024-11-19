use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum TaskState {
    Running,
    Pending,
    Failed(String),
    Complete,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TaskBase {
    pub id: i64,
    pub state: TaskState,
    pub create_time: DateTime<Utc>,
    pub origin: Option<i64>,
}

#[derive(Debug, Deref, DerefMut, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct RefreshTask {
    #[deref] #[deref_mut]
    pub base: TaskBase,
    pub storage: i64,
    pub directory: i64,
}
