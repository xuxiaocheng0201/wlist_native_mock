use wlist_native::common::data::storages::StorageType;

pub mod information;
pub mod options;

#[flutter_rust_bridge::frb(non_opaque)]
/// All supported storage types.
#[derive(Copy, Clone, o2o::o2o)]
#[map_owned(StorageType)]
pub enum FStorageType {
    /// [lanzou](https://up.woozooo.com/).
    Lanzou,

}

impl FStorageType {
    #[flutter_rust_bridge::frb(sync, getter)]
    /// True means the storage is shared. (Other's share link.)
    pub fn is_share(self) -> bool {
        Into::<StorageType>::into(self).is_share()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// True means the storage is private. (User's personal account.)
    pub fn is_private(self) -> bool {
        Into::<StorageType>::into(self).is_private()
    }
}
