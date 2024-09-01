use std::sync::Arc;

use chrono::{DateTime, Utc};
use derive_more::Constructor;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use validators::phonenumber::country::Id;

use crate::common::data::storages::information::{StorageDetailsInformation, StorageInformation, StorageListInformation};
use crate::common::data::storages::options::{FilterFlags, ListStorageOptions};
use crate::common::data::storages::StorageType;
use crate::common::exceptions::{IncorrectArgumentError, InvalidStorageConfigError, StorageNotFoundError};
use crate::core::client::context::define_func;

static NOW: Lazy<DateTime<Utc>> = Lazy::new(Utc::now);

// TODO: implement

define_func!(storages_list(login_context, options: ListStorageOptions) ->StorageListInformation = {
    let storage = StorageInformation {
        id: 1,
        name: Arc::new("tester".to_string()),
        read_only: false,
        storage_type: StorageType::Lanzou,
        available: true,
        create_time: *NOW,
        update_time: *NOW,
        root_directory_id: -1,
    };
    if options.filter.flags().contains(FilterFlags::Writable) {
        Ok(StorageListInformation {
            total: 1,
            filtered: 1,
            storages: if options.offset == 0 && options.limit > 0 { vec![storage] } else { vec![] },
        })
    } else {
        Ok(StorageListInformation {
            total: 1,
            filtered: 0,
            storages: vec![],
        })
    }
});
define_func!(storages_get(login_context, id: i64, _check: bool) -> StorageDetailsInformation = {
    if id != 1 { return Err(StorageNotFoundError.into()); }
    let storage = StorageInformation {
        id: 1,
        name: Arc::new("tester".to_string()),
        read_only: false,
        storage_type: StorageType::Lanzou,
        available: true,
        create_time: *NOW,
        update_time: *NOW,
        root_directory_id: -1,
    };
    Ok(StorageDetailsInformation {
        basic: storage,
        size: Some(130),
        indexed_size: 130,
        total_size: None,
        upload_flow: None,
        download_flow: None,
        max_size_per_file: 100 << 20,
    })
});
define_func!(storages_remove(login_context, id: i64) -> () = {
    if id != 1 { return Err(StorageNotFoundError.into()); }
    unimplemented!()
});
define_func!(storages_rename(login_context, id: i64, _name: String) -> () = {
    if id != 1 { return Err(StorageNotFoundError.into()); }
    unimplemented!()
});
define_func!(storages_set_readonly(login_context, id: i64, _readonly: bool) -> () = {
    if id != 1 { return Err(StorageNotFoundError.into()); }
    unimplemented!()
});


#[derive(Debug, Constructor, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct LanzouConfigurationInner {
    pub phone_number: Arc<String>,
    pub password: Arc<String>,
    pub root_directory_id: i64,
}

impl LanzouConfigurationInner {
    pub fn check(&self) -> Result<(), InvalidStorageConfigError> {
        let mut errors = HashMap::new();

        if let Err(e) = validators::phonenumber::parse(Some(Id::CN), self.phone_number.as_str()) {
            errors.insert("phone_number".into(), e.to_string());
        }
        if self.password.len() < 6 {
            errors.insert("password".into(), "must be at least 6 characters".to_string());
        }
        if self.root_directory_id < -1 {
            errors.insert("root_directory_id".into(), "must be greater than or equal to -1".to_string());
        }

        if errors.is_empty() { Ok(()) } else { Err(InvalidStorageConfigError::new(errors)) }
    }
}

define_func!(storages_lanzou_add(login_context, name: String, config: LanzouConfigurationInner) -> StorageInformation = {
    config.check()?;
    if name.len() < 1 { return Err(IncorrectArgumentError::new("storage name is empty".into()).into()); }
    if name.len() > 32767 { return Err(IncorrectArgumentError::new("storage name is too long".into()).into()); }
    unimplemented!()
});
define_func!(storages_lanzou_update(login_context, id: i64, config: LanzouConfigurationInner) -> () = {
    config.check()?;
    if id != 1 { return Err(StorageNotFoundError.into()); }
    unimplemented!()
});
define_func!(storages_lanzou_chcek(_context, config: LanzouConfigurationInner) -> () = {
    config.check().map_err(Into::into)
});
