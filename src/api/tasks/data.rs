use crate::api::tasks::FTask;
use crate::utils::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// Filter the task list by task type.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::data::TasksFilter)]
pub enum FTasksFilter {
    /// Do not filter.
    All,
    /// Only refresh tasks.
    Refresh,
    /// Only download tasks.
    Download,
    /// Only upload tasks.
    Upload,

}

#[flutter_rust_bridge::frb(non_opaque)]
/// Filter the task list by task state.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::data::TaskStateFilter)]
pub enum FTaskStateFilter {
    /// Do not filter.
    All,
    /// `state` == 0
    Running,
    /// `state` == 1
    Pausing,
    /// `state` == 2
    Pending,
    /// `state` == 3
    Failed,
    /// `state` == 4
    Cancelled,
    /// `state` == 5
    Complete,

    /// `state` == 0 OR `state` == 1 OR `state` == 2
    Working,
    /// `state` == 3 OR `state` == 4 OR `state` == 5
    Finished,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The information list of a task list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::data::TaskListInformation)]
pub struct FTaskListInformation {
    /// The total number of running tasks after applying type filter.
    pub total_running: u64,
    /// The total number of pausing tasks after applying type filter.
    pub total_pausing: u64,
    /// The total number of pending tasks after applying type filter.
    pub total_pending: u64,
    /// The total number of complete tasks after applying type filter.
    pub total_complete: u64,
    /// The total number of cancelled tasks after applying type filter.
    pub total_cancelled: u64,
    /// The total number of failed tasks after applying type filter.
    pub total_failed: u64,
    /// The tasks list.
    #[map(o2o::map_vec(~))]
    pub tasks: Vec<FTask>,
}
