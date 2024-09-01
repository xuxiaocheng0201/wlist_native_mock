use serde::Deserialize;

use crate::common::exceptions::TokenExpiredError;
use crate::web::LOGIN_STATUS;

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum VersionState {
    Latest,
    Updatable,
    Unavailable,
}

pub async fn check_version() -> anyhow::Result<VersionState> {
    LOGIN_STATUS.read().await.as_ref().ok_or(TokenExpiredError)?;
    Ok(VersionState::Latest)
}
