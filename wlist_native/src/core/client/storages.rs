use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use derive_more::Constructor;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use validators::phonenumber::country::Id;

use crate::common::data::storages::information::{StorageDetailsInformation, StorageInformation, StorageListInformation};
use crate::common::data::storages::options::{FilterFlags, ListStorageOptions};
use crate::common::data::storages::StorageType;
use crate::common::exceptions::{IncorrectArgumentError, InvalidStorageConfigError, StorageNotFoundError, StorageTypeMismatchedError};
use crate::core::client::context::define_func;
use crate::core::storages::lanzou::LanzouStorage;

static ATOMIC_ID: AtomicI64 = AtomicI64::new(1);
static STORAGES: Lazy<DashMap<i64, (LanzouConfigurationInner, StorageInformation, LanzouStorage)>> = Lazy::new(DashMap::new);

pub(crate) fn get_storage(id: i64) -> Result<Ref<i64, (LanzouConfigurationInner, StorageInformation, LanzouStorage)>> {
    STORAGES.get(&id).ok_or(StorageNotFoundError.into())
}

define_func!(storages_list(login_context, options: ListStorageOptions) -> StorageListInformation = {
    let iter = STORAGES.iter()
        .map(|entry| entry.value().1.clone());
    let total = iter.clone().count() as u64;
    let iter = iter.filter(|s| {
        let flag = options.filter.flags();
        if !flag.contains(FilterFlags::Shared) {
            if s.storage_type.is_share() {
                return false;
            }
        }
        if !flag.contains(FilterFlags::Writable) {
            if s.storage_type.is_private() && !s.read_only {
                return false;
            }
        }
        if !flag.contains(FilterFlags::ReadonlyPrivate) {
            if s.storage_type.is_private() && s.read_only {
                return false;
            }
        }
        true
    });
    let filtered = iter.clone().count() as u64;
    let storages = iter.skip(options.offset as usize)
        .take(options.limit as usize)
        .collect::<Vec<_>>();
    Ok(StorageListInformation {
        total, filtered, storages,
    })
});
define_func!(storages_get(login_context, id: i64, _check: bool) -> StorageDetailsInformation = {
    let storage = STORAGES.get(&id).ok_or(StorageNotFoundError)?.1.clone();
    // TODO: mock more detail implement.
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
    STORAGES.remove(&id).ok_or(StorageNotFoundError)?;
    Ok(())
});
define_func!(storages_rename(login_context, id: i64, name: String) -> () = {
    if name.len() < 1 { return Err(IncorrectArgumentError::new("storage name is empty".into()).into()); }
    if name.len() > 32767 { return Err(IncorrectArgumentError::new("storage name is too long".into()).into()); }
    let mut entry = STORAGES.get_mut(&id).ok_or(StorageNotFoundError)?;
    entry.1.name = Arc::new(name);
    Ok(())
});
define_func!(storages_set_readonly(login_context, id: i64, readonly: bool) -> () = {
    let mut entry = STORAGES.get_mut(&id).ok_or(StorageNotFoundError)?;
    if !readonly && entry.1.storage_type.is_share() {
        return Err(StorageTypeMismatchedError.into());
    }
    entry.1.read_only = readonly;
    Ok(())
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
    let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    let info = StorageInformation {
        id, name: Arc::new(name),
        read_only: false,
        storage_type: StorageType::Lanzou,
        available: true,
        create_time: Utc::now(),
        update_time: Utc::now(),
        root_directory_id: config.root_directory_id,
    };
    let storage = LanzouStorage::new(config.root_directory_id);
    STORAGES.insert(id, (config, info.clone(), storage));
    Ok(info)
});
define_func!(storages_lanzou_update(login_context, id: i64, config: LanzouConfigurationInner) -> () = {
    config.check()?;
    let mut storage = STORAGES.get_mut(&id).ok_or(StorageNotFoundError)?;
    if storage.1.storage_type != StorageType::Lanzou {
        return Err(StorageTypeMismatchedError.into());
    }
    storage.0 = config;
    Ok(())
});
define_func!(storages_lanzou_chcek(_context, config: LanzouConfigurationInner) -> () = {
    config.check().map_err(Into::into)
});
