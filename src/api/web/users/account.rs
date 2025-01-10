use crate::api::common::exceptions::UniverseError;
use crate::api::common::o2o;
use crate::api::web::users::FUser;

/// Check current is logged in. Return true if logged in.
pub async fn is_logged_in() -> Result<bool, UniverseError> {
    wlist_native::web::users::account::is_logged_in().await.map_err(Into::into)
}

/// Do log in.
///
/// id: The user id returned by [register with mail](super::register::register_with_mail).
///
/// password: 6 <= len <= 128
///
/// If the returned value is `None`, the password is mismatched.
pub async fn login<'a>(id: String, password: String) -> Result<Option<FUser>, UniverseError> {
    wlist_native::web::users::account::login(id, password).await.map(o2o::map_option).map_err(Into::into)
}

/// Directly login with mail.
///
/// mail: The mail of the user.
///
/// password: 6 <= len <= 128
///
/// If the returned value is `None`, the password is mismatched.
pub async fn login_with_mail<'a>(mail: String, password: String) -> Result<Option<FUser>, UniverseError> {
    wlist_native::web::users::account::login_with_mail(mail, password).await.map(o2o::map_option).map_err(Into::into)
}

/// Do log out. (require login)
pub async fn logout() -> Result<(), UniverseError> {
    wlist_native::web::users::account::logout().await.map_err(Into::into)
}
