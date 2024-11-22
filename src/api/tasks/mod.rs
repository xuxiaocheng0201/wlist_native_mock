//! Represent tasks.
//! Notice this module only provides a database layer.

use crate::api::common::o2o;
use crate::api::tasks::tasks::FTaskBase;

pub mod data;
pub mod tasks;
pub mod manager;

#[flutter_rust_bridge::frb(non_opaque)]
/// A union task struct.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::Task)]
pub enum FTask {
    /// Represents a refresh task.
    Refresh(#[map(o2o::map(~))] tasks::FRefreshTask),
    /// Represents a download task.
    Download(#[map(o2o::map(~))] tasks::FDownloadTask),
    /// Represents a upload task.
    Upload(#[map(o2o::map(~))] tasks::FUploadTask),

}

impl FTask {
    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn base(&self) -> &FTaskBase {
        match self {
            FTask::Refresh(task) => &task.base,
            FTask::Download(task) => &task.base,
            FTask::Upload(task) => &task.base,
        }
    }
}
