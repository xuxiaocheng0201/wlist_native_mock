use crate::common::data::files::FileLocation;
use crate::common::data::files::information::FileListInformation;
use crate::common::data::files::options::{ListFileOptions, SearchFilesOptions};
use crate::common::data::trashes::information::TrashListInformation;
use crate::common::data::trashes::options::{ListTrashOptions, SearchTrashesOptions};
use crate::core::client::context::define_func;

define_func!(search_file(login_context, directory: FileLocation, options: SearchFilesOptions, list_options: ListFileOptions) -> FileListInformation = {
    unimplemented!()
});
define_func!(search_trash(login_context, storage: i64, options: SearchTrashesOptions, list_options: ListTrashOptions) -> TrashListInformation = {
    unimplemented!()
});
