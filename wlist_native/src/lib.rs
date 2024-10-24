use std::fmt::Debug;
use std::path::PathBuf;

pub mod common;
pub mod web;
pub mod core;

pub(crate) async fn initialize(data_directory: impl Into<PathBuf> + Debug, cache_directory: impl Into<PathBuf> + Debug) -> anyhow::Result<()> {
    common::workspace::initialize(data_directory, cache_directory)?;
    common::database::initialize()?;
    Ok(())
}
