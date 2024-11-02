use either::Either;

use crate::common::data::files::confirmations::RefreshConfirmation;
use crate::common::data::files::information::{FileDetailsInformation, FileInformation, FileListInformation};
use crate::common::data::files::options::{Duplicate, ListFileOptions};
use crate::common::data::files::FileLocation;
use crate::core::client::context::define_func;

define_func!(files_list(login_context, directory: FileLocation, options: ListFileOptions) -> Either<FileListInformation, RefreshConfirmation> = {
    unimplemented!()
});
define_func!(files_get(login_context, location: FileLocation, check: bool) -> FileDetailsInformation = {
    unimplemented!()
});

define_func!(files_copy(login_context, source: FileLocation, target: FileLocation, name: String, duplicate: Duplicate) -> FileInformation = {
    unimplemented!()
});
define_func!(files_move(login_context, source: FileLocation, target: FileLocation, duplicate: Duplicate) -> FileInformation = {
    unimplemented!()
});
define_func!(files_rename(login_context, location: FileLocation, name: String, duplicate: Duplicate) -> FileInformation = {
    unimplemented!()
});
