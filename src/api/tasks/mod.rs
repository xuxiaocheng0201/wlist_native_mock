use crate::utils::o2o;

pub mod data;
pub mod tasks;
pub mod manager;
pub mod runner;

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
