use crate::api::common::data::files::information::FFileListInformation;
use crate::api::common::data::files::options::{FListFileOptions, FSearchFilesOptions};
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::trashes::information::FTrashListInformation;
use crate::api::common::data::trashes::options::{FListTrashOptions, FSearchTrashesOptions};
use crate::api::core::client::define_func;

define_func!(
    /// Search files in the directory.
    ///
    /// This won't check whether the directory is existed or not.
    /// If the directory is not existed, this will return a empty list.
    ///
    /// directory: `.isDirectory == true`.
    search_file(directory: FFileLocation, options: FSearchFilesOptions, list_options: FListFileOptions) -> FFileListInformation = wlist_native::core::client::search::search_file
);

define_func!(
    /// Search files in the trashed file lists.
    search_trash(storage: i64, options: FSearchTrashesOptions, list_options: FListTrashOptions) -> FTrashListInformation = wlist_native::core::client::search::search_trash
);
