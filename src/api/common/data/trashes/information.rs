use crate::api::common::o2o;

#[flutter_rust_bridge::frb(non_opaque)]
/// The information of a trashed file/directory.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::information::TrashInformation)]
pub struct FTrashInformation {
    /// The file/directory id.
    pub id: i64,
    /// The file/directory name.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub name: String,
    /// True if this is a directory.
    pub is_directory: bool,
    /// The file/directory size. (null means unknown.)
    pub size: Option<u64>,
    /// The file/directory create time. (null means unknown.)
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The file/directory update time. (null means unknown.)
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The file/directory trash time. (null means unknown.)
    pub trash_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The detail information of a trashed file/directory.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::information::TrashDetailsInformation)]
pub struct FTrashDetailsInformation {
    /// The file/directory information.
    #[map(o2o::map(~))]
    pub basic: FTrashInformation,
    /// The file/directory md5. (null means unknown.)
    ///
    /// This is a lowercase string with a length of 32.
    /// For directory, it's always null.
    #[from(o2o::from_option_arc(~))]
    #[into(o2o::into_option_arc(~))]
    pub md5: Option<String>,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The information list of a trashed file list.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::trashes::information::TrashListInformation)]
pub struct FTrashListInformation {
    /// The total number of the trashed files/directories.
    pub total: u64,
    /// The number of files/directories after [filtering](crate::api::common::data::files::options::FFilesFilter).
    pub filtered: u64,
    /// The information list.
    #[map(o2o::map_vec(~))]
    pub files: Vec<FTrashInformation>,
}
