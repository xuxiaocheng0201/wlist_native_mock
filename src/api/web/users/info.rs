use crate::api::common::exceptions::UniverseError;
use crate::api::web::users::FUserRole;

#[flutter_rust_bridge::frb(non_opaque)]
/// The [get user info](get_user_info) result.
pub struct FUserInfoPatch {
    /// The nickname of the current user.
    pub nickname: String,
    /// The role of the current user.
    pub role: FUserRole,
}

/// Get the patch information of the current user. (require login)
///
/// Notice this not return [FUser](crate::api::web::users::FUser),
/// because the rest fields won't be modified.
pub async fn get_user_info() -> Result<FUserInfoPatch, UniverseError> {
    wlist_native::web::users::info::get_user_info().await
        .map(|(nickname, role)| FUserInfoPatch { nickname, role: role.into(), })
        .map_err(Into::into)
}

/// Set the nickname of the current user. (require login)
///
/// nickname: 1 <= len <= 128
pub async fn set_nickname(nickname: String) -> Result<(), UniverseError> {
    wlist_native::web::users::info::set_nickname(nickname).await.map_err(Into::into)
}
