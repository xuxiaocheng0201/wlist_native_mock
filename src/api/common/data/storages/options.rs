use crate::api::common::data::FDirection;
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// Filter the storage list.
///
///
/// 1: If the storage is shared, it always is readonly.
///
/// 2: If the storage is private, it is writable default.
///
/// 3: You can set a private storage to readonly.
///
/// | Storage Type |  Shared  | Private |
/// | ------------ | -------- | ------- |
/// |   Readonly   | always'1 | maybe'3 |
/// |   Writable   |  never   | maybe'2 |
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::options::StoragesFilter)]
pub enum FStoragesFilter {
    /// Readonly, both shared and private. (1, 3)
    Readonly,
    /// Writable, only private. (2)
    Writable,
    /// Shared, only readonly. (1)
    Shared,
    /// Private, both readonly and writable. (2, 3)
    Private,
    /// Only readonly and private. (3)
    ReadonlyPrivate,
    /// Shared and readonly private. (1, 2)
    Owned,
    /// Do not filter. (1, 2, 3)
    All,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Sort the storage list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::options::StoragesOrder)]
pub enum FStoragesOrder {
    /// Sort by the storage id.
    Id,
    /// Sort by the storage name. (sort in GBK encoding)
    Name,
    /// true/false. 'true' is ahead of 'false'.
    Shared,
    /// true/false. 'true' is ahead of 'false'.
    Readonly,
    /// Sort by the storage used size. (unknown is ahead of known)
    Size,
    /// Sort by the storage indexed size.
    IndexedSize,
    /// Sort by the storage total size. (unknown is ahead of known)
    TotalSize,
    /// Sort by the storage spare size. (unknown is ahead of known)
    ///
    /// This is computed by `TotalSize - Size`.
    SpareSize,
    /// Sort by the storage create time.
    CreateTime,
    /// Sort by the storage update time.
    UpdateTime,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// Options when listing storages.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::options::ListStorageOptions)]
pub struct FListStorageOptions {
    /// The filter to determine which type of storages to list.
    #[map(o2o::map(~))]
    pub filter: FStoragesFilter,
    /// The order in which to list the storages.
    ///
    /// The front entry has a higher priority.
    /// Notice that items with the same priority will be listed in a random order.
    #[from(o2o::from_index_map(~))]
    #[into(o2o::into_index_map(~))]
    pub orders: Vec<(FStoragesOrder, FDirection)>,
    /// The offset of the first item to list.
    pub offset: u64,
    /// The maximum number of items to list.
    pub limit: u32,
}
