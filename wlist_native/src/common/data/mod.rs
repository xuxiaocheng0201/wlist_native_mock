use serde::{Deserialize, Serialize};

pub mod storages;
pub mod files;
pub mod trashes;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    ASCEND,
    DESCEND,
}

impl Direction {
    pub fn revert(&self) -> Self {
        match self {
            Direction::ASCEND => Direction::DESCEND,
            Direction::DESCEND => Direction::ASCEND,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Language {
    EnUs,
    ZhCn,
}
