use wlist_native::common::data::storages::StorageType;

use crate::api::common::exceptions::UniverseError;
use crate::api::common::o2o;

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
    pub fn is_share(&self) -> bool {
        Into::<StorageType>::into(*self).is_share()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// True means the storage is private. (User's personal account.)
    pub fn is_private(&self) -> bool {
        Into::<StorageType>::into(*self).is_private()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// All allowed suffixes by the storage.
    /// Empty set means all suffixes are valid.
    ///
    /// See the source code in [check_filename](wlist_native::common::data::storages::StorageType::check_filename) for details and example.
    pub fn allowed_suffixes(&self) -> std::collections::HashSet<String> {
        o2o::from_hash_set(Into::<StorageType>::into(*self).allowed_suffixes().clone())
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// All disallowed suffixed by the storage.
    /// Empty set means all suffixes are valid.
    ///
    /// See the source code in [check_filename](wlist_native::common::data::storages::StorageType::check_filename) for details and example.
    pub fn disallowed_suffixes(&self) -> std::collections::HashSet<String> {
        o2o::from_hash_set(Into::<StorageType>::into(*self).disallowed_suffixes().clone())
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// True means the storage allows no suffix.
    ///
    /// See the source code in [check_filename](wlist_native::common::data::storages::StorageType::check_filename) for details and example.
    pub fn allow_no_suffix(&self) -> bool {
        Into::<StorageType>::into(*self).allow_no_suffix()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// All allowed characters by the storage.
    /// Empty set means all characters are valid.
    ///
    /// See the source code in [check_filename](wlist_native::common::data::storages::StorageType::check_filename) for details and example.
    pub fn allowed_characters(&self) -> std::collections::HashSet<char> {
        o2o::from_hash_set(Into::<StorageType>::into(*self).allowed_characters().clone())
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// All disallowed characters by the storage.
    /// Empty set means all characters are valid.
    ///
    /// See the source code in [check_filename](wlist_native::common::data::storages::StorageType::check_filename) for details and example.
    pub fn disallowed_characters(&self) -> std::collections::HashSet<char> {
        o2o::from_hash_set(Into::<StorageType>::into(*self).disallowed_characters().clone())
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    /// Max filename length allowed by the storage.
    ///
    /// The min filename length is always 1.
    ///
    /// 0 < TheReturnedValue < 32768
    ///
    /// See the source code in [check_filename](wlist_native::common::data::storages::StorageType::check_filename) for details and example.
    pub fn max_filename_length(&self) -> usize {
        Into::<StorageType>::into(*self).max_filename_length()
    }

    /// A fast test for check filename that is not dependent on existed storage.
    /// If the filename is invalid, an error will be returned.
    ///
    /// This function is similar to [`upload_check_name`](crate::api::core::client::upload::upload_check_name),
    /// but only check this three exceptions: [NameTooLongError], [InvalidFilenameError], [IllegalSuffixError].
    ///
    /// >[NameTooLongError]: UniverseError::NameTooLongError
    /// >[InvalidFilenameError]: UniverseError::InvalidFilenameError
    /// >[IllegalSuffixError]: UniverseError::IllegalSuffixError
    pub fn check_filename(&self, name: String, is_directory: bool) -> Result<(), UniverseError> {
        Into::<StorageType>::into(*self).check_filename(&o2o::into_arc(name), is_directory).map_err(Into::into)
    }
}
