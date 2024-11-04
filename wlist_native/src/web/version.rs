use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum VersionState {
    Latest,
    Updatable,
    Unavailable,
}

pub async fn check_version() -> anyhow::Result<VersionState> {
    unimplemented!()
}
