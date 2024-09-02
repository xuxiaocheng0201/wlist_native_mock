use either::Either;
use md5::digest::typenum::op;
use crate::common::data::files::confirmations::RefreshConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::files::information::{FileDetailsInformation, FileInformation, FileListInformation};
use crate::common::data::files::options::{Duplicate, FilesFilter, ListFileOptions};
use crate::common::exceptions::{FileNotFoundError, IncorrectArgumentError};
use crate::core::client::context::define_func;
use crate::core::client::storages::get_storage;

define_func!(files_list(login_context, directory: FileLocation, options: ListFileOptions) -> Either<FileListInformation, RefreshConfirmation> = {
    if !directory.is_directory { return(Err(IncorrectArgumentError::new("listing children in file".into()).into())); }
    let storage = get_storage(directory.storage)?;
    let directory = storage.2.map.get(&directory.file_id).ok_or(FileNotFoundError::new(directory))?;
    let iter = directory.children().iter();
    let total = iter.clone().count() as u64;
    let iter = iter.filter(|s| {
        match &options.filter {
            FilesFilter::OnlyDirectories => s.info.is_directory,
            FilesFilter::OnlyFiles => !s.info.is_directory,
            FilesFilter::Both => true,
        }
    })
    let filtered = iter.clone().count() as u64;
    let files = iter
        .offset(options.offset as usize)
        .take(options.limit as usize)
        .map(|node| node.datail(None))
        .collect::<Vec<_>>();
    let list = FileListInformation { total, filtered, files, };
    // TODO: Mock refresh confirmation.
    Ok(Either::Left(list))
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
