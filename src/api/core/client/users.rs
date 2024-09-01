use crate::api::core::client::define_func;

define_func!(
    /// Login the core client.
    ///
    ///
    /// username: should always be "admin".
    ///
    /// password: the string returned by [reset_admin_password](crate::api::core::server::users::reset_admin_password).
    users_login(username: String, password: String) -> () = wlist_native::core::client::users::users_login
);
define_func!(
    /// Logout the core client.
    users_logout() -> () = wlist_native::core::client::users::users_logout
);
