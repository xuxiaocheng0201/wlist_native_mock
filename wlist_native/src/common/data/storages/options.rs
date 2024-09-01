use bitflags::bitflags;
use derive_more::Display;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::data::Direction;

#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StoragesFilter {
    Readonly,
    Writable,
    Shared,
    Private,
    ReadonlyPrivate,
    Owned,
    All,
}

bitflags! {
    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct FilterFlags: u8 {
        const Shared = 1; // 1
        const Writable = 1 << 1; // 2
        const ReadonlyPrivate = 1 << 2; // 3
    }
}

impl StoragesFilter {
    #[inline]
    pub const fn flags(&self) -> FilterFlags {
        match self {
            StoragesFilter::Readonly => FilterFlags::Shared.union(FilterFlags::ReadonlyPrivate),
            StoragesFilter::Writable => FilterFlags::Writable,
            StoragesFilter::Shared => FilterFlags::Shared,
            StoragesFilter::Private => FilterFlags::Writable.union(FilterFlags::ReadonlyPrivate),
            StoragesFilter::ReadonlyPrivate => FilterFlags::ReadonlyPrivate,
            StoragesFilter::Owned => FilterFlags::Shared.union(FilterFlags::Writable),
            StoragesFilter::All => FilterFlags::Shared.union(FilterFlags::Writable).union(FilterFlags::ReadonlyPrivate),
        }
    }
}

#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StoragesOrder {
    Id,
    Name,
    Shared,
    Readonly,
    Size,
    IndexedSize,
    TotalSize,
    SpareSize,
    CreateTime,
    UpdateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ListStorageOptions {
    pub filter: StoragesFilter,
    pub orders: IndexMap<StoragesOrder, Direction>,
    pub offset: u64,
    pub limit: u32,
}
