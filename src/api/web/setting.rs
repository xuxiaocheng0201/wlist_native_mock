use crate::api::common::data::FLanguage;
use crate::api::common::exceptions::UniverseError;

/// Check the current version state.
pub async fn set_language(language: FLanguage) -> Result<(), UniverseError> {
    wlist_native::web::setting::set_language(language.into()).await.map_err(Into::into)
}

#[flutter_rust_bridge::frb(non_opaque)]
#[derive(o2o::o2o)]
#[map_owned(wlist_native::web::setting::VersionState)]
pub enum FVersionState {
    /// The current version is the latest and needn't upgrade.
    Latest,
    /// The current version is not the latest but still can be used.
    Updatable,
    /// The current version shouldn't be used and must upgrade.
    Unavailable,
}

/// Check the current version state.
pub async fn check_version() -> Result<FVersionState, UniverseError> {
    wlist_native::web::setting::check_version().await.map(Into::into).map_err(Into::into)
}
