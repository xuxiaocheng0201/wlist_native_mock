use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection;

use crate::common::workspace::{get_cache_directory, WorkspaceError};

fn get_dict_path() -> Result<PathBuf, WorkspaceError> {
    Ok(get_cache_directory()?.join("simple"))
}

pub fn initialize() -> Result<()> {
    libsimple::release_dict(&get_dict_path()?)?;
    libsimple::enable_auto_extension()?;
    rusqlite_regex::enable_auto_extension()?;
    Ok(())
}

pub fn load_extension(connection: &Connection) -> rusqlite::Result<()> {
    libsimple::set_dict(connection, &get_dict_path().unwrap())?;
    Ok(())
}
