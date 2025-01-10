use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::common::data::Language;

pub async fn set_language(language: Language) -> Result<VersionState> {
    unimplemented!()
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum VersionState {
    Latest,
    Updatable,
    Unavailable,
}

pub async fn check_version() -> Result<VersionState> {
    unimplemented!()
}
