pub mod tokens;
pub mod confirmations;
pub mod information;
pub mod progresses;
pub mod options;

#[flutter_rust_bridge::frb(non_opaque)]
/// Unique location of a file/directory.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::FileLocation)]
pub struct FFileLocation {
    /// The id of the storage.
    pub storage: i64,
    /// The file/directory id.
    pub file_id: i64,
    /// True if the location identify a directory.
    pub is_directory: bool,
}
