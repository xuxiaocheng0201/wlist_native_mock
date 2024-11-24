use serde::{Deserialize, Serialize};

use crate::tasks::Task;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TasksFilter {
    All,
    Refresh,
    Download,
    Upload,

}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TaskStateFilter {
    All,
    Running,
    Pausing,
    Pending,
    Failed,
    Cancelled,
    Complete,

    Working,
    Finished,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TaskListInformation {
    pub total_running: u64,
    pub total_pausing: u64,
    pub total_pending: u64,
    pub total_complete: u64,
    pub total_cancelled: u64,
    pub total_failed: u64,
    pub tasks: Vec<Task>,
}
