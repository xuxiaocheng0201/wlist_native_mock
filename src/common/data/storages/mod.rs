use serde::{Deserialize, Serialize};

pub mod information;
pub mod options;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StorageType {
    Lanzou,
    // More types...
}

impl StorageType {
    #[inline]
    pub const fn is_private(&self) -> bool {
        match self {
            StorageType::Lanzou => true,
        }
    }

    #[inline]
    pub const fn is_share(&self) -> bool {
        match self {
            _ => !self.is_private(),
        }
    }

    // TODO  allowedSuffixes disallowedSuffixes disallowedCharacter minFilenameLength maxFilenameLength()
}
