use either::Either;

use crate::common::data::files::confirmations::RefreshConfirmation;
use crate::common::data::files::information::{FileDetailsInformation, FileInformation, FileListInformation};
use crate::common::data::files::options::{Duplicate, ListFileOptions};
use crate::common::data::files::FileLocation;
use crate::core::client::define_func;

define_func!(files_list(directory: FileLocation, options: ListFileOptions) -> Either<FileListInformation, RefreshConfirmation>);
define_func!(files_get(location: FileLocation, check: bool) -> FileDetailsInformation);

define_func!(files_copy(source: FileLocation, target: FileLocation, name: String, duplicate: Duplicate) -> FileInformation);
define_func!(files_move(source: FileLocation, target: FileLocation, duplicate: Duplicate) -> FileInformation);
define_func!(files_rename(location: FileLocation, name: String, duplicate: Duplicate) -> FileInformation);
