use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::Result;
use once_cell::sync::Lazy;
use thiserror::Error;

static DATA_DIRECTORY: Lazy<RwLock<Option<PathBuf>>> = Lazy::new(|| RwLock::new(None));
static CACHE_DIRECTORY: Lazy<RwLock<Option<PathBuf>>> = Lazy::new(|| RwLock::new(None));

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("Data directory not set.")]
    DataNotSet,
    #[error("Cache directory not set.")]
    CacheNotSet,

    #[error("Data directory does not exist. {0:?}")]
    DataNotExist(PathBuf),
    #[error("Cache directory does not exist. {0:?}")]
    CacheNotExist(PathBuf),
}

pub fn get_data_directory() -> Result<PathBuf, WorkspaceError> {
    DATA_DIRECTORY.read().unwrap().clone().ok_or(WorkspaceError::DataNotSet)
}

pub fn get_cache_directory() -> Result<PathBuf, WorkspaceError> {
    CACHE_DIRECTORY.read().unwrap().clone().ok_or(WorkspaceError::CacheNotSet)
}

pub fn initialize(data_directory: impl Into<PathBuf> + Debug, cache_directory: impl Into<PathBuf> + Debug) -> Result<()> {
    let data_directory = data_directory.into().canonicalize()?;
    let cache_directory = cache_directory.into().canonicalize()?;
    if !data_directory.is_dir() { return Err(WorkspaceError::DataNotExist(data_directory).into()); }
    if !cache_directory.is_dir() { return Err(WorkspaceError::CacheNotExist(cache_directory).into()); }
    DATA_DIRECTORY.write().unwrap().replace(data_directory);
    CACHE_DIRECTORY.write().unwrap().replace(cache_directory);
    Ok(())
}
