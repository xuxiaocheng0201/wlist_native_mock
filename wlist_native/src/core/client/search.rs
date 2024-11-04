use crate::common::data::files::FileLocation;
use crate::common::data::files::information::FileListInformation;
use crate::common::data::files::options::{ListFileOptions, SearchFilesOptions};
use crate::common::data::trashes::information::TrashListInformation;
use crate::common::data::trashes::options::{ListTrashOptions, SearchTrashesOptions};
use crate::core::client::define_func;

define_func!(search_file(directory: FileLocation, options: SearchFilesOptions, list_options: ListFileOptions) -> FileListInformation);
define_func!(search_trash(storage: i64, options: SearchTrashesOptions, list_options: ListTrashOptions) -> TrashListInformation);
