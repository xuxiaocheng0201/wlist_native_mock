use crate::api::common::exceptions::UniverseError;

/// Try register with mail.
/// This sends a verification code to the email and returns an ident, you should pass the ident to [register_with_mail].
///
/// If the returned value is `None`, the mail is not supported.
pub async fn register_with_mail_untrusted<'a>(mail: String) -> Result<Option<String>, UniverseError> {
    wlist_native::web::users::register::register_with_mail_untrusted(mail).await.map_err(Into::into)
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The [register with mail](register_with_mail) result.
pub enum FRegisterEmailResult {
    /// The code is mismatched, you can try again.
    VerifyFailed,
    /// This mail is registered.
    Duplicate,
    /// The returned user id.
    Success(String),
}

/// Register with mail.
/// Then you should call [login](super::account::login) with the returned user id to log in.
///
/// ident: This ident returned by [register_with_mail_untrusted].
///
/// code: The verification code sent to the mail.
///
/// password: 6 <= len <= 128
pub async fn register_with_mail<'a>(ident: String, code: String, password: String) -> Result<FRegisterEmailResult, UniverseError> {
    wlist_native::web::users::register::register_with_mail(ident, code, password).await.map(|result| match result {
        Err(false) => FRegisterEmailResult::VerifyFailed,
        Err(true) => FRegisterEmailResult::Duplicate,
        Ok(id) => FRegisterEmailResult::Success(id),
    }).map_err(Into::into)
}

/// Unregister with mail. (require login)
/// After unregistered, it logged out automatically.
///
/// password: The login password. Used to verify the user.
pub async fn unregister<'p>(password: String) -> Result<(), UniverseError> {
    wlist_native::web::users::register::unregister(password).await.map_err(Into::into)
}
