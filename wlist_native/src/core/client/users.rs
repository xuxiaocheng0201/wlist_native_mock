use crate::core::client::define_func;

define_func!(users_login(username: String, passport: String) -> ());
define_func!(users_logout() -> ());
