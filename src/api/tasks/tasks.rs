use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// Task status.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::tasks::TaskState)]
pub enum FTaskState {
    /// Task is running.
    Running,
    /// Task is pending.
    Pending,
    /// Task is failed.
    Failed(String),
    /// Task is completed.
    Complete,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Basic task struct.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::tasks::TaskBase)]
pub struct FTaskBase {
    /// The task id.
    pub id: i64,
    /// The task state.
    #[map(o2o::map(~))]
    pub state: FTaskState,
    /// The task create time.
    pub create_time: chrono::DateTime<chrono::Utc>,
    /// The origin task id.
    pub origin: Option<i64>,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Represent a refresh task.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::tasks::RefreshTask)]
pub struct FRefreshTask {
    /// The task base.
    #[map(o2o::map(~))]
    pub base: FTaskBase,
    /// The refresh storage.
    pub storage: i64,
    /// The refresh directory.
    pub directory: i64,
}
