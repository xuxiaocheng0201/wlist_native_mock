use crate::api::common::data::FDirection;
use crate::api::common::data::files::options::{FFilesFilter, FSearchPattern};
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// Sort the trashed file list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::options::TrashesOrder)]
pub enum FTrashesOrder {
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
    /// Sort by the file/directory trash time. (unknown is ahead of known)
    TrashTime,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Options when listing trashed files.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::options::ListTrashOptions)]
pub struct FListTrashOptions {
    /// The filter to determine which type of trashed file to list.
    #[map(o2o::map(~))]
    pub filter: FFilesFilter,
    /// The order in which to list the trashed files.
    ///
    /// The front entry has a higher priority.
    /// Notice that items with the same priority will be listed in a random order.
    #[from(o2o::from_index_map(~))]
    #[into(o2o::into_index_map(~))]
    pub orders: Vec<(FTrashesOrder, FDirection)>,
    /// The offset of the first item to list.
    pub offset: u64,
    /// The maximum number of items to list.
    pub limit: u32,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Options when searching trashed files/directories.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::options::SearchTrashesOptions)]
pub struct FSearchTrashesOptions {
    /// The keyword to search.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub keyword: String,
    /// The pattern to search.
    #[map(o2o::map(~))]
    pub pattern: FSearchPattern,
}
