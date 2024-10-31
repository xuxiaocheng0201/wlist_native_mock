use std::fmt::Debug;
use std::path::PathBuf;

pub mod data;
pub mod exceptions;
pub mod workspace;
pub mod database;
pub mod versions;

pub async fn initialize(data_directory: impl Into<PathBuf> + Debug, cache_directory: impl Into<PathBuf> + Debug) -> anyhow::Result<()> {
    workspace::initialize(data_directory, cache_directory)?;
    database::initialize()?;
    Ok(())
}
