use crate::api::common::data::storages::information::{FStorageDetailsInformation, FStorageInformation, FStorageListInformation};
use crate::api::common::data::storages::options::FListStorageOptions;
use crate::api::common::o2o;
use crate::api::core::client::define_func;

define_func!(
    /// List the storages.
    storages_list(options: FListStorageOptions) -> FStorageListInformation = wlist_native::core::client::storages::storages_list
);
define_func!(
    /// Get the detail information of a storage.
    ///
    /// check: True indicates the server should recheck whether the storage is available.
    ///
    /// May returns [StorageNotFoundError](crate::api::common::exceptions::UniverseError::StorageNotFoundError).
    storages_get(id: i64, check: bool) -> FStorageDetailsInformation = wlist_native::core::client::storages::storages_get
);
define_func!(
    /// Remove a storage.
    ///
    /// May returns [StorageNotFoundError](crate::api::common::exceptions::UniverseError::StorageNotFoundError) and
    /// [StorageInLockError](crate::api::common::exceptions::UniverseError::StorageInLockError).
    storages_remove(id: i64) -> () = wlist_native::core::client::storages::storages_remove
);
define_func!(
    /// Rename a storage.
    ///
    /// name: 0 < len < 32768
    ///
    /// May returns [StorageNotFoundError](crate::api::common::exceptions::UniverseError::StorageNotFoundError) and
    /// [DuplicateStorageError](crate::api::common::exceptions::UniverseError::DuplicateStorageError).
    storages_rename(id: i64, name: String) -> () = wlist_native::core::client::storages::storages_rename
);
define_func!(
    /// Set the readonly of a storage.
    ///
    /// May returns [StorageNotFoundError](crate::api::common::exceptions::UniverseError::StorageNotFoundError),
    /// [StorageTypeMismatchedError](crate::api::common::exceptions::UniverseError::StorageTypeMismatchedError) and
    /// [StorageInLockError](crate::api::common::exceptions::UniverseError::StorageInLockError).
    storages_set_readonly(id: i64, readonly: bool) -> () = wlist_native::core::client::storages::storages_set_readonly
);

#[flutter_rust_bridge::frb(non_opaque)]
/// The [Lanzou](crate::api::common::data::storages::FStorageType::Lanzou) type storage config.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::core::client::storages::LanzouConfigurationInner)]
pub struct FLanzouConfiguration {
    /// Login phone number.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub phone_number: String,
    /// Login password. (6 <= len)
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub password: String,
    /// The root directory id. (Default should be -1.)
    pub root_directory_id: i64,
}

define_func!(
    /// Add a [Lanzou](crate::api::common::data::storages::FStorageType::Lanzou) type storage.
    ///
    /// name: 0 < len < 32768
    ///
    /// May returns [DuplicateStorageError](crate::api::common::exceptions::UniverseError::DuplicateStorageError),
    /// [IncorrectStorageAccountError](crate::api::common::exceptions::UniverseError::IncorrectStorageAccountError),
    /// [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError) and
    /// [IncorrectArgumentError](crate::api::common::exceptions::UniverseError::IncorrectArgumentError).
    storages_lanzou_add(name: String, config: FLanzouConfiguration) -> FStorageInformation = wlist_native::core::client::storages::storages_lanzou_add
);
define_func!(
    /// Update the configuration of the [Lanzou](crate::api::common::data::storages::FStorageType::Lanzou) type storage.
    ///
    /// May returns [IncorrectStorageAccountError](crate::api::common::exceptions::UniverseError::IncorrectStorageAccountError),
    /// [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError),
    /// [StorageNotFoundError](crate::api::common::exceptions::UniverseError::StorageNotFoundError),
    /// [StorageTypeMismatchedError](crate::api::common::exceptions::UniverseError::StorageTypeMismatchedError) and
    /// [StorageInLockError](crate::api::common::exceptions::UniverseError::StorageInLockError)
    storages_lanzou_update(id: i64, config: FLanzouConfiguration) -> () = wlist_native::core::client::storages::storages_lanzou_update
);
define_func!(
    /// Check the [Lanzou](crate::api::common::data::storages::FStorageType::Lanzou) type [configuration](FLanzouConfiguration).
    ///
    /// Notice that this method won't check whether the account is correct/available (won't log in).
    /// May returns [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError).
    storages_lanzou_chcek(config: FLanzouConfiguration) -> () = wlist_native::core::client::storages::storages_lanzou_chcek
);
