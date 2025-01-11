use std::sync::Arc;

use chrono::{DateTime, Utc};
use either::Either;
use serde::{Deserialize, Serialize};

use crate::common::data::storages::information::{StorageDetailsInformation, StorageInformation, StorageListInformation};
use crate::common::data::storages::options::ListStorageOptions;
use crate::core::client::define_func;

define_func!(storages_list(options: ListStorageOptions) -> StorageListInformation);
define_func!(storages_get(id: i64, _check: bool) -> StorageDetailsInformation);
define_func!(storages_remove(id: i64) -> ());
define_func!(storages_rename(id: i64, name: String) -> ());
define_func!(storages_set_readonly(id: i64, readonly: bool) -> ());

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LanzouConfigurationInner {
    pub phone_number: Arc<String>,
    pub password: Arc<String>,
    pub root_directory_id: i64,
}

define_func!(storages_lanzou_add(name: String, config: LanzouConfigurationInner) -> StorageInformation);
define_func!(storages_lanzou_update(id: i64, config: LanzouConfigurationInner) -> ());
define_func!(storages_lanzou_check(config: LanzouConfigurationInner) -> ());

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BaiduConfigurationInner {
    pub authorization_code: Arc<String>,
    pub root_directory_path: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct BaiduLoginTuple {
    pub token: Arc<String>,
    pub expire: DateTime<Utc>,
    pub refresh_token: Arc<String>,
}

define_func!(storages_baidu_add(name: String, config: BaiduConfigurationInner, token: Option<BaiduLoginTuple>) -> StorageInformation);
define_func!(storages_baidu_update(id: i64, config: BaiduConfigurationInner) -> ());
define_func!(storages_baidu_check(config: BaiduConfigurationInner) -> ());
pub fn baidu_login_url() -> String {
    unimplemented!()
}
pub fn baidu_login_callback_url() -> String {
    unimplemented!()
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Pan123ConfigurationInner {
    pub passport: Either<Arc<String>, Arc<String>>,
    pub password: Arc<String>,
    pub root_directory_id: i64,
}

define_func!(storages_pan123_add(name: String, config: Pan123ConfigurationInner) -> StorageInformation);
define_func!(storages_pan123_update(id: i64, config: Pan123ConfigurationInner) -> ());
define_func!(storages_pan123_check(config: Pan123ConfigurationInner) -> ());
