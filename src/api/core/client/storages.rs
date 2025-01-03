use std::sync::Arc;

use chrono::{DateTime, Utc};
use either::Either;

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
    storages_lanzou_check(config: FLanzouConfiguration) -> () = wlist_native::core::client::storages::storages_lanzou_check
);

#[flutter_rust_bridge::frb(non_opaque)]
/// The [Baidu](crate::api::common::data::storages::FStorageType::Baidu) type storage config.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::core::client::storages::BaiduConfigurationInner)]
pub struct FBaiduConfiguration {
    /// Login authorization code.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub authorization_code: String,
    /// The root directory path. (Default should be empty string.)
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub root_directory_path: String,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The [Baidu](crate::api::common::data::storages::FStorageType::Baidu) type storage login tokens.
/// This is to simplify the login process. Returned by baidu login API.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::core::client::storages::BaiduLoginTuple)]
pub struct FBaiduLoginTuple {
    /// Access token.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub token: String,
    /// Access token expire time.
    pub expire: DateTime<Utc>,
    /// Refresh token.
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub refresh_token: String,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// This is a helper struct for [FBaiduLoginTuple].
pub struct FBaiduLoginTupleOption(pub Option<FBaiduLoginTuple>);

impl From<FBaiduLoginTupleOption> for Option<wlist_native::core::client::storages::BaiduLoginTuple> {
    fn from(value: FBaiduLoginTupleOption) -> Self {
        value.0.map(Into::into)
    }
}

define_func!(
    /// Add a [Baidu](crate::api::common::data::storages::FStorageType::Baidu) type storage.
    ///
    /// name: 0 < len < 32768
    ///
    /// May returns [DuplicateStorageError](crate::api::common::exceptions::UniverseError::DuplicateStorageError),
    /// [IncorrectStorageAccountError](crate::api::common::exceptions::UniverseError::IncorrectStorageAccountError),
    /// [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError) and
    /// [IncorrectArgumentError](crate::api::common::exceptions::UniverseError::IncorrectArgumentError).
    storages_baidu_add(name: String, config: FBaiduConfiguration, token: FBaiduLoginTupleOption) -> FStorageInformation = wlist_native::core::client::storages::storages_baidu_add
);
define_func!(
    /// Update the configuration of the [Baidu](crate::api::common::data::storages::FStorageType::Baidu) type storage.
    ///
    /// May returns [IncorrectStorageAccountError](crate::api::common::exceptions::UniverseError::IncorrectStorageAccountError),
    /// [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError),
    /// [StorageNotFoundError](crate::api::common::exceptions::UniverseError::StorageNotFoundError),
    /// [StorageTypeMismatchedError](crate::api::common::exceptions::UniverseError::StorageTypeMismatchedError) and
    /// [StorageInLockError](crate::api::common::exceptions::UniverseError::StorageInLockError)
    storages_baidu_update(id: i64, config: FBaiduConfiguration) -> () = wlist_native::core::client::storages::storages_baidu_update
);
define_func!(
    /// Check the [Baidu](crate::api::common::data::storages::FStorageType::Baidu) type [configuration](FBaiduConfiguration).
    ///
    /// Notice that this method won't check whether the account is correct/available (won't log in).
    /// May returns [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError).
    storages_baidu_check(config: FBaiduConfiguration) -> () = wlist_native::core::client::storages::storages_baidu_check
);

#[flutter_rust_bridge::frb(non_opaque)]
/// The type of `passport` in [FPan123Configuration] result.
pub enum FPan123ConfigurationPassport {
    /// Login with phone number.
    PhoneNumber(String),
    /// Login with mail address.
    MailAddress(String),
}

impl From<FPan123ConfigurationPassport> for Either<Arc<String>, Arc<String>> {
    fn from(value: FPan123ConfigurationPassport) -> Self {
        match value {
            FPan123ConfigurationPassport::PhoneNumber(passport) => Either::Left(o2o::into_arc(passport)),
            FPan123ConfigurationPassport::MailAddress(passport) => Either::Right(o2o::into_arc(passport)),
        }
    }
}

impl From<Either<Arc<String>, Arc<String>>> for FPan123ConfigurationPassport {
    fn from(value: Either<Arc<String>, Arc<String>>) -> Self {
        match value {
            Either::Left(passport) => FPan123ConfigurationPassport::PhoneNumber(o2o::from_arc(passport)),
            Either::Right(passport) => FPan123ConfigurationPassport::MailAddress(o2o::from_arc(passport)),
        }
    }
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The [Pan123](crate::api::common::data::storages::FStorageType::Pan123) type storage config.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::core::client::storages::Pan123ConfigurationInner)]
pub struct FPan123Configuration {
    /// Login passport.
    #[map(~.into())]
    pub passport: FPan123ConfigurationPassport,
    /// Login password. (6 <= len <= 16)
    #[from(o2o::from_arc(~))]
    #[into(o2o::into_arc(~))]
    pub password: String,
    /// The root directory id. (Default should be 0.)
    pub root_directory_id: i64,
}

define_func!(
    /// Add a [Pan123](crate::api::common::data::storages::FStorageType::Pan123) type storage.
    ///
    /// name: 0 < len < 32768
    ///
    /// May returns [DuplicateStorageError](crate::api::common::exceptions::UniverseError::DuplicateStorageError),
    /// [IncorrectStorageAccountError](crate::api::common::exceptions::UniverseError::IncorrectStorageAccountError),
    /// [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError) and
    /// [IncorrectArgumentError](crate::api::common::exceptions::UniverseError::IncorrectArgumentError).
    storages_pan123_add(name: String, config: FPan123Configuration) -> FStorageInformation = wlist_native::core::client::storages::storages_pan123_add
);
define_func!(
    /// Update the configuration of the [Pan123](crate::api::common::data::storages::FStorageType::Pan123) type storage.
    ///
    /// May returns [IncorrectStorageAccountError](crate::api::common::exceptions::UniverseError::IncorrectStorageAccountError),
    /// [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError),
    /// [StorageNotFoundError](crate::api::common::exceptions::UniverseError::StorageNotFoundError),
    /// [StorageTypeMismatchedError](crate::api::common::exceptions::UniverseError::StorageTypeMismatchedError) and
    /// [StorageInLockError](crate::api::common::exceptions::UniverseError::StorageInLockError)
    storages_pan123_update(id: i64, config: FPan123Configuration) -> () = wlist_native::core::client::storages::storages_pan123_update
);
define_func!(
    /// Check the [Pan123](crate::api::common::data::storages::FStorageType::Pan123) type [configuration](FPan123Configuration).
    ///
    /// Notice that this method won't check whether the account is correct/available (won't log in).
    /// May returns [InvalidStorageConfigError](crate::api::common::exceptions::UniverseError::InvalidStorageConfigError).
    storages_pan123_check(config: FPan123Configuration) -> () = wlist_native::core::client::storages::storages_pan123_check
);
