use crate::api::common::exceptions::UniverseError;

/// Reset the admin password of the core server.
/// The new password will be returned. (length == 8)
pub async fn reset_admin_password() -> Result<String, UniverseError> {
    wlist_native::core::server::users::reset_admin_password().await.map_err(Into::into)
}
