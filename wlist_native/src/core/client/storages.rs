use std::sync::Arc;

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
