use crate::api::common::data::FDirection;
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// Filter the file list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::FilesFilter)]
pub enum FFilesFilter {
    /// Only directories, no files.
    OnlyDirectories,
    /// Only files, no directories.
    OnlyFiles,
    /// Do not filter.
    Both,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Sort the file list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::FilesOrder)]
pub enum FFilesOrder {
    /// Sort by the file/directory id.
    Id,
    /// Sort by the file/directory name. (sort in GBK encoding)
    Name,
    /// Sort by the suffix of name.
    ///
    /// Split by '.', it is unknown if the name does not contain '.'.
    /// (unknown is ahead of known, sort in GBK encoding)
    Suffix,
    /// true/false. 'true' is ahead of 'false'.
    Directory,
    /// Sort by the file/directory size. (unknown is ahead of known)
    Size,
    /// Sort by the file/directory create time. (unknown is ahead of known)
    CreateTime,
    /// Sort by the file/directory update time. (unknown is ahead of known)
    UpdateTime,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Options when listing files.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::ListFileOptions)]
pub struct FListFileOptions {
    /// The filter to determine which type of file to list.
    #[map(o2o::map(~))]
    pub filter: FFilesFilter,
    /// The order in which to list the files.
    ///
    /// The front entry has a higher priority.
    /// Notice that items with the same priority will be listed in a random order.
    #[from(o2o::from_index_map(~))]
    #[into(o2o::into_index_map(~))]
    pub orders: Vec<(FFilesOrder, FDirection)>,
    /// The offset of the first item to list.
    pub offset: u64,
    /// The maximum number of items to list.
    pub limit: u32,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The policy when upload/mkdir/etc. that already exists with the same name.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::Duplicate)]
pub enum FDuplicate {
    /// Return an [error](crate::api::common::exceptions::UniverseError::DuplicateFileError).
    Error,
    /// Replace the old file/directory. (Not atomic, delete before uploading/creating/etc.)
    Replace,
    /// Keep the old file/directory. Add a suffix to the new file/directory name.
    ///
    /// The suffix may be " (1)", " (2)", "（1）", etc.
    /// This may return [NameTooLongError](crate::api::common::exceptions::UniverseError::NameTooLongError) even if the original name is short enough.
    Rename,
    /// Compare files/directories, use [Rename] if different, otherwise ignore the upload/create.
    ///
    /// For file, compare both md5 and size of the file. If any md5/size is unknown, the files are different.
    /// For directory, only two confirmed(have indexes) empty directories are same.
    RenameIfDifferent,
}


#[flutter_rust_bridge::frb(non_opaque)]
/// The pattern when searching.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::SearchPattern)]
pub enum FSearchPattern {
    /// ```sql
    /// WHERE name == :keyword
    /// ```
    FullMatch,
    /// ```sql
    /// WHERE name REGEXP :keyword
    /// ```
    Regex,
    /// Using [an extension of fts5](https://github.com/wangfenjin/simple).
    /// ```sql
    /// WHERE name MATCH jieba_query(:keyword)
    /// ```
    Pinyin,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Options when searching files/directories.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::options::SearchFilesOptions)]
pub struct FSearchFilesOptions {
    /// The keyword to search.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub keyword: String,
    /// The pattern to search.
    #[map(o2o::map(~))]
    pub pattern: FSearchPattern,
    /// True means search in recursive directories.
    pub recursive: bool,
}
