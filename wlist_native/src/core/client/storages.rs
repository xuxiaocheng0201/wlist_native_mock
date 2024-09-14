use std::cmp::Ordering;
use std::sync::atomic::AtomicI64;
use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use chrono::Utc;
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use derive_more::Constructor;
use hashbrown::HashMap;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use validators::phonenumber::country::Id;

use crate::common::data::Direction;
use crate::common::data::storages::information::{StorageDetailsInformation, StorageInformation, StorageListInformation};
use crate::common::data::storages::options::{FilterFlags, ListStorageOptions, StoragesOrder};
use crate::common::data::storages::StorageType;
use crate::common::exceptions::{IncorrectArgumentError, InvalidStorageConfigError, StorageNotFoundError, StorageTypeMismatchedError};
use crate::core::client::context::define_func;
use crate::core::storages::lanzou::LanzouStorage;

static ATOMIC_ID: AtomicI64 = AtomicI64::new(1);
static STORAGES: Lazy<DashMap<i64, (LanzouConfigurationInner, StorageInformation, LanzouStorage)>> = Lazy::new(DashMap::new);

pub(crate) fn get_storage(id: i64) -> Result<Ref<'static, i64, (LanzouConfigurationInner, StorageInformation, LanzouStorage)>> {
    STORAGES.get(&id).ok_or(StorageNotFoundError.into())
}

pub(crate) async fn get_detail_information(id: i64) -> Result<StorageDetailsInformation> {
    let storage = get_storage(id)?.1.clone();
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
}

define_func!(storages_list(login_context, options: ListStorageOptions) -> StorageListInformation = {
    let iter = STORAGES.iter()
        .map(|entry| entry.value().1.clone());
    let total = iter.clone().count() as u64;
    let vec = iter.filter(|s| {
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
    }).collect::<Vec<_>>();
    let filtered = vec.len() as u64;
    struct Ordered<'a> {
        order: &'a IndexMap<StoragesOrder, Direction>,
        encoded_name_cache: once_cell::sync::OnceCell<Bytes>,
        storage: StorageDetailsInformation,
    }
    impl<'a> Ordered<'a> {
        fn encoded_name(&self) -> &[u8] {
            self.encoded_name_cache.get_or_init(||
                encoding_rs::GBK.encode(&self.storage.basic.name).0.into_owned().into()
            )
        }
    }
    impl<'a> PartialEq<Self> for Ordered<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.storage.eq(&other.storage)
        }
    }
    impl<'a> Eq for Ordered<'a> { }
    impl<'a> PartialOrd<Self> for Ordered<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let a = &self.storage.basic;
            let b = &other.storage.basic;
            for (k, v) in self.order {
                let order = match k {
                    StoragesOrder::Id => a.id.cmp(&b.id),
                    StoragesOrder::Name => self.encoded_name().cmp(&other.encoded_name()),
                    StoragesOrder::Shared => a.storage_type.is_share().cmp(&b.storage_type.is_share()).reverse(),
                    StoragesOrder::Readonly => a.read_only.cmp(&b.read_only).reverse(),
                    StoragesOrder::Size => self.storage.size.cmp(&other.storage.size),
                    StoragesOrder::IndexedSize => self.storage.indexed_size.cmp(&other.storage.indexed_size),
                    StoragesOrder::TotalSize => self.storage.total_size.cmp(&other.storage.total_size),
                    StoragesOrder::SpareSize => self.storage.spare_size().cmp(&other.storage.spare_size()),
                    StoragesOrder::CreateTime => a.create_time.cmp(&b.create_time),
                    StoragesOrder::UpdateTime => a.update_time.cmp(&b.update_time),
                };
                let order = match v { Direction::ASCEND => order, Direction::DESCEND => order.reverse(), };
                if order.is_ne() { return Some(order); }
            }
            None
        }
    }
    impl<'a> Ord for Ordered<'a> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
        }
    }
    let mut v = Vec::with_capacity(vec.len());
    for s in vec {
        let detail = get_detail_information(s.id).await?;
        v.push(Ordered {
            order: &options.orders,
            encoded_name_cache: Default::default(),
            storage: detail,
        });
    }
    v.sort_unstable();
    let storages = v.into_iter()
        .map(|s| s.storage.basic)
        .skip(options.offset as usize)
        .take(options.limit as usize)
        .collect::<Vec<_>>();
    Ok(StorageListInformation {
        total, filtered, storages,
    })
});
define_func!(storages_get(login_context, id: i64, _check: bool) -> StorageDetailsInformation = {
    get_detail_information(id).await
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
    let id = ATOMIC_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
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
define_func!(storages_lanzou_check(_context, config: LanzouConfigurationInner) -> () = {
    config.check().map_err(Into::into)
});
