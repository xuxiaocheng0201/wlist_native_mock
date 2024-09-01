pub mod login {
    use std::borrow::Cow;
    use std::fmt::Debug;

    use crate::common::exceptions::PasswordMismatchedError;
    use crate::web::{ACCOUNT, LOGIN_STATUS};

    pub async fn login<'i, 'p>(user_id: impl Into<Cow<'i, str>> + Debug, password: impl Into<Cow<'p, str>> + Debug) -> anyhow::Result<()> {
        let user_id = user_id.into();
        let password = password.into();
        if password.len() < 6 || 128 < password.len() { return Err(PasswordMismatchedError.into()); }
        match ACCOUNT.get::<str>(&user_id) {
            Some(entry) if entry.value() == &password => {
                LOGIN_STATUS.write().await.replace(user_id.to_string());
                Ok(())
            },
            _ => Err(PasswordMismatchedError.into()),
        }
    }
}

pub mod logout {
    use crate::web::LOGIN_STATUS;

    pub async fn logout() -> anyhow::Result<()> {
        LOGIN_STATUS.write().await.take();
        Ok(())
    }
}
