use std::sync::Arc;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::common::data::storages::information::{StorageDetailsInformation, StorageInformation, StorageListInformation};
use crate::common::data::storages::options::ListStorageOptions;
use crate::core::client::context::define_func;

define_func!(storages_list(login_context, options: ListStorageOptions) -> StorageListInformation = {
    unimplemented!()
});
define_func!(storages_get(login_context, id: i64, _check: bool) -> StorageDetailsInformation = {
    unimplemented!()
});
define_func!(storages_remove(login_context, id: i64) -> () = {
    unimplemented!()
});
define_func!(storages_rename(login_context, id: i64, name: String) -> () = {
    unimplemented!()
});
define_func!(storages_set_readonly(login_context, id: i64, readonly: bool) -> () = {
    unimplemented!()
});

#[derive(Debug, Constructor, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LanzouConfigurationInner {
    pub phone_number: Arc<String>,
    pub password: Arc<String>,
    pub root_directory_id: i64,
}

define_func!(storages_lanzou_add(login_context, name: String, config: LanzouConfigurationInner) -> StorageInformation = {
    unimplemented!()
});
define_func!(storages_lanzou_update(login_context, id: i64, config: LanzouConfigurationInner) -> () = {
    unimplemented!()
});
define_func!(storages_lanzou_check(_context, config: LanzouConfigurationInner) -> () = {
    unimplemented!()
});
