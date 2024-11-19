use crate::api::common::exceptions::UniverseError;

pub mod common;
pub mod web;
pub mod core;
pub mod tasks;

/// Initialize the core server.
///
/// Note that you **must** call this method before calling all the others.
/// You should ensure these directories exist and have permissions to read/write them.
///
/// The path can be absolute or relative.
/// data_directory contains the data files, such as the database.
/// cache_directory contains the support files, such as the dynamic libs.
pub async fn initialize(data_directory: String, cache_directory: String) -> Result<(), UniverseError> {
    wlist_native::common::initialize(data_directory, cache_directory).await.map_err(Into::into)
}
