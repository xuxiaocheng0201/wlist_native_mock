use std::sync::Arc;

use derive_more::Display;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::data::Direction;

#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FilesFilter {
    OnlyDirectories,
    OnlyFiles,
    Both,
}

#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FilesOrder {
    Id,
    Name,
    Suffix,
    Directory,
    Size,
    CreateTime,
    UpdateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ListFileOptions {
    pub filter: FilesFilter,
    pub orders: IndexMap<FilesOrder, Direction>,
    pub offset: u64,
    pub limit: u32,
}

#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Duplicate {
    Error,
    Replace,
    Rename,
    RenameIfDifferent,
}


#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SearchPattern {
    FullMatch,
    Regex,
    Pinyin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SearchFilesOptions {
    pub keyword: Arc<String>,
    pub pattern: SearchPattern,
    pub recursive: bool,
}
