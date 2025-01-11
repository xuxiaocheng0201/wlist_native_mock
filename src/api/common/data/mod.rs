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

#[flutter_rust_bridge::frb(non_opaque)]
/// The supported language.
#[derive(Copy, Clone, o2o::o2o)]
#[map_owned(wlist_native::common::data::Language| _ => unreachable!())]
#[non_exhaustive]
pub enum FLanguage {
    /// en-us
    EnUs,
    /// zh-cn
    ZhCn,
}
