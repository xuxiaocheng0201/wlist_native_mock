use either::Either;

use wlist_native::common::data::files::confirmations::RefreshConfirmation;
use wlist_native::common::data::trashes::information::TrashListInformation;

use crate::api::common::data::files::confirmations::FRefreshConfirmation;
use crate::api::common::data::files::information::FFileInformation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::trashes::information::{FTrashDetailsInformation, FTrashInformation, FTrashListInformation};
use crate::api::common::data::trashes::options::FListTrashOptions;
use crate::api::core::client::define_func;

#[flutter_rust_bridge::frb(non_opaque)]
/// The [files list](files_list) result.
pub enum FTrashListResult {
    /// The list result.
    List(FTrashListInformation),
    /// The refresh confirmation.
    Refresh(FRefreshConfirmation),
}

impl From<Either<TrashListInformation, RefreshConfirmation>> for FTrashListResult {
    fn from(value: Either<TrashListInformation, RefreshConfirmation>) -> Self {
        match value {
            Either::Left(list) => FTrashListResult::List(list.into()),
            Either::Right(refresh) => FTrashListResult::Refresh(refresh.into()),
        }
    }
}

define_func!(
    /// List the trashed files.
    trash_list(storage: i64, options: FListTrashOptions) -> FTrashListResult = wlist_native::core::client::trash::trash_list
);
define_func!(
    /// Refresh the trash bin.
    trash_refresh(storage: i64) -> FRefreshConfirmation = wlist_native::core::client::trash::trash_refresh
);
define_func!(
    /// Get detail information of a trashed file/directory.
    ///
    /// check: True indicates the server should recheck the information.
    trash_get(location: FFileLocation, check: bool) -> FTrashDetailsInformation = wlist_native::core::client::trash::trash_get
);
define_func!(
    /// Trash a file/directory.
    ///
    /// # [ComplexOperationError](crate::api::common::exceptions::UniverseError::ComplexOperationError) small pieces:
    /// (This only may be returned when `location.isDirectory == true`.)
    /// Recursively access the files/directories in the directory, and [trash](trash_trash) them.
    trash_trash(location: FFileLocation) -> FTrashInformation = wlist_native::core::client::trash::trash_trash
);
define_func!(
    /// Restore a trashed file/directory.
    ///
    /// parent: the parent directory id the file/directory restored to.
    trash_restore(location: FFileLocation, parent: i64) -> FFileInformation = wlist_native::core::client::trash::trash_restore
);
define_func!(
    /// Completely delete the trashed file/directory.
    trash_delete(location: FFileLocation) -> () = wlist_native::core::client::trash::trash_delete
);
define_func!(
    /// Delete all the trashed files and directories.
    trash_delete_all(storage: i64) -> () = wlist_native::core::client::trash::trash_delete_all
);
