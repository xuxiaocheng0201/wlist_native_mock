pub mod account {
    use crate::api::common::exceptions::UniverseError;

    /// Log in to the web.
    pub async fn login(user_id: String, password: String) -> Result<(), UniverseError> {
        wlist_native::web::account::login::login(user_id, password).await.map_err(Into::into)
    }

    /// Log out from the web.
    pub async fn logout() -> Result<(), UniverseError> {
        wlist_native::web::account::logout::logout().await.map_err(Into::into)
    }
}

pub mod register {
    use crate::api::common::exceptions::UniverseError;

    /// Register as guest, returns user_id.
    ///
    /// device_id: 16 <= len <= 256
    /// password: 6 <= len <= 128
    pub async fn register_as_guest(device_id: String, password: String) -> Result<Option<String>, UniverseError> {
        wlist_native::web::register::as_guest::register_as_guest(device_id, password).await.map_err(Into::into)
    }

    /// Unregister, requires password to verify.
    pub async fn unregister(password: String) -> Result<(), UniverseError> {
        wlist_native::web::register::unregister::unregister(password).await.map_err(Into::into)
    }
}

pub mod user {
    use crate::api::common::exceptions::UniverseError;

    /// Get the nickname of the current user.
    pub async fn get_nickname() -> Result<String, UniverseError> {
        wlist_native::web::user::nickname::get_nickname().await.map_err(Into::into)
    }

    /// Set the nickname of the current user.
    pub async fn set_nickname(nickname: String) -> Result<(), UniverseError> {
        wlist_native::web::user::nickname::set_nickname(nickname).await.map_err(Into::into)
    }

    /// Reset the password of the current user.
    pub async fn reset_password(old: String, new: String) -> Result<(), UniverseError> {
        wlist_native::web::user::password::reset_password(old, new).await.map_err(Into::into)
    }
}

pub mod version {
    use crate::api::common::exceptions::UniverseError;

    #[flutter_rust_bridge::frb(non_opaque)]
    #[derive(o2o::o2o)]
    #[map_owned(wlist_native::web::version::VersionState)]
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
        wlist_native::web::version::check_version().await.map(Into::into).map_err(Into::into)
    }
}
