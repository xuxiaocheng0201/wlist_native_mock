use crate::common::exceptions::PasswordMismatchedError;
use crate::core::client::context::define_func;
use crate::core::server::users::login;

define_func!(users_login(context, username: String, passport: String) -> () = {
    let ok = login(&username, &passport).await?;
    if ok { context.write_inner().await.login_status = Some(()); }
    if ok { Ok(()) } else { Err(PasswordMismatchedError.into()) }
});

define_func!(users_logout(context,) -> () = {
    context.write_inner().await.login_status = None;
    Ok(())
});
