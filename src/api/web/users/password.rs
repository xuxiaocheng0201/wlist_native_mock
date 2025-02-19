use crate::api::common::exceptions::UniverseError;
use crate::utils::convert_option_bool;
/// Reset the password. (require login)
/// If reset successfully, it will return true.
///
/// old: The old login password. Used to verify the user.
///
/// new: 6 <= len <= 128
pub async fn reset_password(old: String, new: String) -> Result<bool, UniverseError> {
    wlist_native::web::users::password::reset_password(old, new).await.map(convert_option_bool).map_err(Into::into)
}

/// Try reset the password with mail. (require login)
/// This sends a verification code to the email and returns an ident, you should pass the ident to [reset_password_with_mail].
pub async fn reset_password_with_mail_untrusted() -> Result<String, UniverseError> {
    wlist_native::web::users::password::reset_password_with_mail_untrusted().await.map_err(Into::into)
}

/// Reset the password with mail.
///
/// ident: This ident returned by [reset_password_with_mail_untrusted].
///
/// code: The verification code sent to the mail.
///
/// password: 6 <= len <= 128
pub async fn reset_password_with_mail<'a>(ident: String, code: String, password: String) -> Result<bool, UniverseError> {
    wlist_native::web::users::password::reset_password_with_mail(ident, code, password).await.map(convert_option_bool).map_err(Into::into)
}
