use either::Either;

use wlist_native::common::data::files::confirmations::RefreshConfirmation;
use wlist_native::common::data::files::information::FileListInformation;

use crate::api::common::data::files::confirmations::FRefreshConfirmation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::files::information::{FFileDetailsInformation, FFileInformation, FFileListInformation};
use crate::api::common::data::files::options::{FDuplicate, FListFileOptions};
use crate::api::core::client::define_func;

#[flutter_rust_bridge::frb(non_opaque)]
/// The [files list](files_list) result.
pub enum FFilesListResult {
    /// The list result.
    List(FFileListInformation),
    /// The refresh confirmation.
    Refresh(FRefreshConfirmation),
}

impl From<Either<FileListInformation, RefreshConfirmation>> for FFilesListResult {
    fn from(value: Either<FileListInformation, RefreshConfirmation>) -> Self {
        match value {
            Either::Left(list) => FFilesListResult::List(list.into()),
            Either::Right(refresh) => FFilesListResult::Refresh(refresh.into()),
        }
    }
}

define_func!(
    /// List the files in directory.
    ///
    /// directory: `.isDirectory == true`.
    files_list(directory: FFileLocation, options: FListFileOptions) -> FFilesListResult = wlist_native::core::client::files::files_list
);
define_func!(
    /// Get detail information of a file/directory.
    ///
    /// check: True indicates the server should recheck the information.
    files_get(location: FFileLocation, check: bool) -> FFileDetailsInformation = wlist_native::core::client::files::files_get
);

define_func!(
    /// Copy the source file/directory to the target directory
    ///
    ///
    /// target: `.isDirectory == true`.
    ///
    /// name: 0 < len < 32768
    ///
    /// # [ComplexOperationError]
    /// ## For `source.isDirectory == true` small pieces:
    /// Recursively access the files/directories in the source directory,
    /// and [copy](files_copy) them to the target directory.
    /// ## For `source.isDirectory == false` small pieces:
    /// [Download] the source file and [upload] to the target directory.
    /// Use streaming transmission as much as possible.
    ///
    /// >[ComplexOperationError]: crate::api::common::exceptions::UniverseError::ComplexOperationError
    /// >[download]: crate::api::core::client::download::download_request
    /// >[upload]: crate::api::core::client::upload::upload_request
    files_copy(source: FFileLocation, target: FFileLocation, name: String, duplicate: FDuplicate) -> FFileInformation = wlist_native::core::client::files::files_copy
);
define_func!(
    /// Move the source file/directory to the target directory.
    ///
    /// If the target directory is the parent directory of the source, the source won't be moved.
    /// If the target directory is in the source directory, it will return [FileInLockError].
    ///
    /// target: `.isDirectory == true`.
    ///
    /// # [ComplexOperationError] small pieces:
    /// [Copy](files_copy) the source to the target.
    /// Then, [trash] and [delete] the source directory.
    ///
    /// >[FileInLockError]: crate::api::common::exceptions::UniverseError::FileInLockError
    /// >[ComplexOperationError]: crate::api::common::exceptions::UniverseError::ComplexOperationError
    /// >[trash]: crate::api::core::client::trash::trash_trash
    /// >[delete]: crate::api::core::client::trash::trash_delete
    files_move(source: FFileLocation, target: FFileLocation, duplicate: FDuplicate) -> FFileInformation = wlist_native::core::client::files::files_move
);
define_func!(
    /// Rename the source file/directory.
    ///
    /// name: 0 < len < 32768
    ///
    /// If the name is same as the current name, the source won't be renamed.
    ///
    /// # [ComplexOperationError]
    /// ## For `location.isDirectory == true` small pieces:
    /// [Create] a directory in the parent directory of the source with the new name.
    /// Access the files/directories in the source directory and [copy](files_copy) them to the new directory.
    /// Then, [trash] and [delete] the source directory.
    /// ## For `location.isDirectory == false` small pieces:
    /// [Download] the file and [upload] as new file with the new name.
    /// Use streaming transmission as much as possible.
    /// Then, [trash] and [delete] the source file.
    ///
    /// >[ComplexOperationError]: crate::api::common::exceptions::UniverseError::ComplexOperationError
    /// >[create]: crate::api::core::client::upload::upload_mkdir
    /// >[download]: crate::api::core::client::download::download_request
    /// >[upload]: crate::api::core::client::upload::upload_request
    /// >[trash]: crate::api::core::client::trash::trash_trash
    /// >[delete]: crate::api::core::client::trash::trash_delete
    files_rename(location: FFileLocation, name: String, duplicate: FDuplicate) -> FFileInformation = wlist_native::core::client::files::files_rename
);
