use crate::api::common::o2o;
use crate::api::common::data::files::confirmations::FDownloadConfirmation;

#[flutter_rust_bridge::frb(non_opaque)]
/// The information of a file/directory.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::FileInformation)]
pub struct FFileInformation {
    /// The file/directory id.
    pub id: i64,
    /// The parent directory id. (For root directory, it equals to `id`.)
    pub parent_id: i64,
    /// The file/directory name.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub name: String,
    /// True if this is a directory.
    pub is_directory: bool,
    /// The file/directory size. (null means unknown.)
    pub size: Option<u64>,
    /// The file/directory create time. (null means unknown.)
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The file/directory update time. (null means unknown.)
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[flutter_rust_bridge::frb(mirror(FileDetailsInformation), non_opaque)]
/// The detail information of a file/directory.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::FileDetailsInformation)]
pub struct FFileDetailsInformation {
    /// The file/directory information.
    #[map(o2o::map(~))]
    pub basic: FFileInformation,
    /// The file/directory md5. (null means unknown.)
    ///
    /// This is a lowercase string with a length of 32.
    /// For directory, it's always null.
    #[from(o2o::from_option_arc(~))]
    #[into(o2o::into_option_arc(~))]
    pub md5: Option<String>,
    /// The full path. (Not contain the storage name and the file/directory name.)
    pub path: Vec<String>,
    /// The thumbnail download confirmation.
    ///
    /// Notice you should call [download_cancel](crate::api::common::files::download::download_cancel) to cancel the download if you don't need.
    #[map(o2o::map_option(~))]
    pub thumbnail: Option<FDownloadConfirmation>,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The information list of a file list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::FileListInformation)]
pub struct FFileListInformation {
    /// The total number of files/directories in the directory.
    pub total: u64,
    /// The number of files/directories after [filtering](crate::api::common::data::files::options::FFilesFilter).
    pub filtered: u64,
    /// The information list.
    #[map(o2o::map_vec(~))]
    pub files: Vec<FFileInformation>,
}


#[flutter_rust_bridge::frb(non_opaque)]
/// The information of each download chunk.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::DownloadChunkInformation)]
pub struct FDownloadChunkInformation {
    /// Support downloads this chunk in parts or not.
    pub range: bool,
    /// The start byte index of the entire file.
    pub start: u64,
    /// This chunk size.
    pub size: u64,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The information of download chunks.
///
/// Ensured that the chunk bounds won't overlap.
/// And the chunks cover the entire file.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::DownloadInformation)]
pub struct FDownloadInformation {
    /// The download chunks. **The chunk id is the index of the list.**
    #[map(o2o::map_vec(~))]
    pub chunks: Vec<FDownloadChunkInformation>,
    /// The expiry time.
    pub expire: chrono::DateTime<chrono::Utc>,
}


#[flutter_rust_bridge::frb(non_opaque)]
/// The information of each upload chunk.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::UploadChunkInformation)]
pub struct FUploadChunkInformation {
    /// The start byte index of the entire file.
    pub start: u64,
    /// This chunk size.
    pub size: u64,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The information of upload chunks.
///
/// Ensured that the chunk bounds won't overlap.
/// But the chunks may not cover the entire file.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::UploadInformation)]
pub struct FUploadInformation {
    /// The upload chunks. **The chunk id is the index of the list.**
    #[map(o2o::map_vec(~))]
    pub chunks: Vec<FUploadChunkInformation>,
    /// The expiry time.
    pub expire: chrono::DateTime<chrono::Utc>,
}


#[flutter_rust_bridge::frb(mirror(ShareInformation), non_opaque)]
/// The shared information of the files/directories.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::information::ShareInformation)]
pub struct FShareInformation {
    /// The sharing id.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub id: String,
    /// The password of the sharing. (null means the sharing doesn't need password.)
    #[from(o2o::from_option_arc(~))]
    #[into(o2o::into_option_arc(~))]
    pub password: Option<String>,
    /// The expiry time. (null means the sharing is already expired.)
    pub expire: Option<chrono::DateTime<chrono::Utc>>,
}
