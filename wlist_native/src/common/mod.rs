pub mod data;
pub mod exceptions;
pub mod versions;

pub async fn initialize(
    data_directory: impl Into<std::path::PathBuf> + std::fmt::Debug,
    cache_directory: impl Into<std::path::PathBuf> + std::fmt::Debug
) -> anyhow::Result<()> {
    unimplemented!()
}
