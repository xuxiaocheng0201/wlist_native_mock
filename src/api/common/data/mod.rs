use wlist_native::common::data::Direction;

pub mod storages;
pub mod files;
pub mod trashes;

#[flutter_rust_bridge::frb(non_opaque)]
/// The order direction.
#[derive(Copy, Clone, o2o::o2o)]
#[map_owned(Direction)]
pub enum FDirection {
    /// Ascending order.
    ASCEND,
    /// Descending order.
    DESCEND,
}

impl FDirection {
    #[flutter_rust_bridge::frb(sync, getter)]
    /// The revert direction.
    pub fn revert(&self) -> Self {
        Into::<Direction>::into(*self).revert().into()
    }
}
