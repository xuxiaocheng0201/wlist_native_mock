use std::sync::Arc;

use derive_more::Display;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::data::Direction;
use crate::common::data::files::options::{FilesFilter, SearchPattern};

#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TrashesOrder {
    Id,
    Name,
    Suffix,
    Directory,
    Size,
    CreateTime,
    UpdateTime,
    TrashTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ListTrashOptions {
    pub filter: FilesFilter,
    pub orders: IndexMap<TrashesOrder, Direction>,
    pub offset: u64,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SearchTrashesOptions {
    pub keyword: Arc<String>,
    pub pattern: SearchPattern,
}
