pub mod data;
pub mod tasks;
pub mod manager;
pub mod runner;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum Task {
    Refresh(tasks::RefreshTask),
    Download(tasks::DownloadTask),
    Upload(tasks::UploadTask),

}
