#[flutter_rust_bridge::frb(non_opaque)]
/// The progress of refresh.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::progresses::RefreshProgress)]
pub struct FRefreshProgress {
    /// The count of the files that have completed loading information.
    pub loaded_files: u64,
    /// The count of the directories that have completed loading information.
    pub loaded_directories: u64,
    /// The count of the files that is known in this directory. (This means that this value may increase.)
    pub total_files: u64,
    /// The count of the directories that is known in this directory. (This means that this value may increase.)
    pub total_directories: u64,
}
