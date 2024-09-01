use crate::api::common::data::storages::FStorageType;
use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// The information of a storage.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageInformation)]
pub struct FStorageInformation {
    /// The storage id.
    pub id: i64,
    /// The storage name.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub name: String,
    /// True if the storage is read-only.
    pub read_only: bool,
    /// The storage type.
    #[map(o2o::map(~))]
    pub storage_type: FStorageType,
    /// True if the storage is available.
    pub available: bool,
    /// The storage create time.
    pub create_time: chrono::DateTime<chrono::Utc>,
    /// The storage update time.
    pub update_time: chrono::DateTime<chrono::Utc>,
    /// The root directory id of the storage.
    pub root_directory_id: i64,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The detail information of a storage.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageDetailsInformation)]
pub struct FStorageDetailsInformation {
    /// The storage information
    #[map(o2o::map(~))]
    pub basic: FStorageInformation,
    /// The storage size already used. (null means unknown.)
    pub size: Option<u64>,
    /// The storage size already indexed.
    pub indexed_size: u64,
    /// The storage total size, both used and not used. (null means unknown.)
    pub total_size: Option<u64>,
    /// The usage of upload flow. (null means unknown/infinity.)
    #[map(o2o::map_option(~))]
    pub upload_flow: Option<FStorageFlow>,
    /// The usage of download flow. (null means unknown/infinity.)
    #[map(o2o::map_option(~))]
    pub download_flow: Option<FStorageFlow>,
    /// The maximum size of a file.
    pub max_size_per_file: u64,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The information list of a storage list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageListInformation)]
pub struct FStorageListInformation {
    /// The total number of storages.
    pub total: u64,
    /// The number of storages after [filtering](crate::api::common::data::storages::options::FStoragesFilter).
    pub filtered: u64,
    /// The information list.
    #[map(o2o::map_vec(~))]
    pub storages: Vec<FStorageInformation>,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The usage of flow.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::storages::information::StorageFlow)]
pub struct FStorageFlow {
    /// The start time of the flow start to calculate.
    pub start: chrono::DateTime<chrono::Utc>,
    /// The next time the used flow to reset.
    pub refresh: chrono::DateTime<chrono::Utc>,
    /// The rest flow.
    pub flow: u64,
}
