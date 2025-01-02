use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::data::Direction;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StoragesFilter {
    Readonly,
    Writable,
    Shared,
    Private,
    ReadonlyPrivate,
    Owned,
    All,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StoragesOrder {
    Id,
    Name,
    Shared,
    Readonly,
    Size,
    IndexedSize,
    TrashSize,
    TrashIndexedSize,
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
