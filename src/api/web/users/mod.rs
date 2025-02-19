use chrono::{DateTime, Utc};

use crate::utils::o2o;

pub mod register;
pub mod account;
pub mod info;
pub mod password;

#[flutter_rust_bridge::frb(non_opaque)]
/// The user role.
#[derive(Copy, Clone, o2o::o2o)]
#[map_owned(wlist_native::web::users::UserRole)]
pub enum FUserRole {
    /// Administrator
    Admin,
    /// Alpha tester
    Alpha,
    /// Beta tester
    Beta,
    /// Normal user
    Normal,
    /// Unregistered user
    Untrusted,
}

#[flutter_rust_bridge::frb(non_opaque)]
/// The user information.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::web::users::User)]
pub struct FUser {
    /// The user id.
    pub id: String,
    /// The nickname of the user.
    pub nickname: String,
    /// The creation time of the user.
    pub create_time: DateTime<Utc>,
    /// The role of the user.
    #[map(o2o::map(~))]
    pub role: FUserRole,
    /// The registered email of the user.
    pub mail: String,
}
