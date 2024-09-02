use anyhow::Result;
use rusqlite::Connection;

// fn get_dict_path() -> Result<PathBuf, WorkspaceError> {
//     Ok(get_cache_directory()?.join("simple"))
// }

pub fn initialize() -> Result<()> {
    // libsimple::release_dict(&get_dict_path()?)?;
    // libsimple::enable_auto_extension()?;
    rusqlite_regex::enable_auto_extension()?;
    Ok(())
}

pub fn load_extension(_connection: &Connection) -> rusqlite::Result<()> {
    // libsimple::set_dict(connection, &get_dict_path().unwrap())?;
    Ok(())
}
