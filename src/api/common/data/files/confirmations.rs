use crate::api::common::data::files::tokens::{FDownloadToken, FRefreshToken, FUploadToken};
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// The confirmation to refresh a directory.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::confirmations::RefreshConfirmation)]
pub struct FRefreshConfirmation {
    /// The count of files in the directory. (null means unknown.)
    pub files: Option<u64>,
    /// The count of directories in the directory. (null means unknown.)
    pub directories: Option<u64>,
    /// The refresh token.
    #[map(o2o::map(~))]
    pub token: FRefreshToken,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The confirmation to download a file.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::confirmations::DownloadConfirmation)]
pub struct FDownloadConfirmation {
    /// The real total download size. (Associate with the `from`/`to` parameters in (download_request)[crate::api::core::client::download::download_request])
    pub size: u64,
    /// The download token.
    #[map(o2o::map(~))]
    pub token: FDownloadToken,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The confirmation to upload a file.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::confirmations::UploadConfirmation)]
pub struct FUploadConfirmation {
    /// Uf true, the file has been successfully uploaded. (Uploaded by Checksum.)
    pub done: bool,
    /// The upload token.
    #[map(o2o::map(~))]
    pub token: FUploadToken,
}
