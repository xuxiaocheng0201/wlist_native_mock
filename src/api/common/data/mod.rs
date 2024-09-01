pub mod storages;
pub mod files;
pub mod trashes;

#[flutter_rust_bridge::frb(non_opaque)]
/// The order direction.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::Direction)]
pub enum FDirection {
    /// Ascending order.
    ASCEND,
    /// Descending order.
    DESCEND,
}
