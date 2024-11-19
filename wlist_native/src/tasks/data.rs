use serde::{Deserialize, Serialize};

use crate::tasks::Task;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TasksFilter {
    All,
    Refresh,

}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TaskStateFilter {
    All,
    /// `state` == 0
    Running,
    /// `state` == 1
    Pending,
    /// `state` == 2
    Failed,
    /// `state` == 3
    Complete,

    /// `state` == 0 OR `state` == 1
    Working,
    /// `state` == 2 OR `state` == 3
    Finished,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct TaskListInformation {
    pub total_pending: u64,
    pub total_running: u64,
    pub total_complete: u64,
    pub total_failed: u64,
    pub tasks: Vec<Task>,
}
