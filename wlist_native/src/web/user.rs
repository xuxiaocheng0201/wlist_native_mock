pub mod nickname {
    use std::borrow::Cow;
    use std::fmt::Debug;

    use once_cell::sync::Lazy;
    use tokio::sync::RwLock;
    use crate::common::exceptions::{IncorrectArgumentError, TokenExpiredError};
    use crate::web::LOGIN_STATUS;

    static USERNAME: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("User 123456".to_string()));

    pub async fn get_nickname() -> anyhow::Result<String> {
        LOGIN_STATUS.read().await.as_ref().ok_or(TokenExpiredError)?;
        Ok(USERNAME.read().await.to_string())
    }

    pub async fn set_nickname<'n>(nickname: impl Into<Cow<'n, str>> + Debug) -> anyhow::Result<()> {
        let nickname = nickname.into();
        if nickname.len() < 1 { return Err(IncorrectArgumentError::new("nickname is empty".into()).into()); }
        if nickname.len() > 128 { return Err(IncorrectArgumentError::new("nickname is too long".into()).into()); }
        LOGIN_STATUS.read().await.as_ref().ok_or(TokenExpiredError)?;
        *USERNAME.write().await = nickname.to_string();
        Ok(())
    }
}

pub mod password {
    use std::borrow::Cow;
    use std::fmt::Debug;

    use crate::common::exceptions::{IncorrectArgumentError, PasswordMismatchedError, TokenExpiredError};
    use crate::web::{ACCOUNT, LOGIN_STATUS};

    pub async fn reset_password<'o, 'n>(old: impl Into<Cow<'o, str>> + Debug, new: impl Into<Cow<'n, str>> + Debug) -> anyhow::Result<()> {
        let old = old.into();
        let new = new.into();
        if old.len() < 6 || 128 < old.len() { return Err(PasswordMismatchedError.into()); }
        if new.len() < 6 { return Err(IncorrectArgumentError::new("password is too simple".into()).into()); }
        if new.len() > 128 { return Err(IncorrectArgumentError::new("password is too long".into()).into()); }
        let user_id = LOGIN_STATUS.read().await.as_ref().ok_or(TokenExpiredError)?.to_string();
        let mut entry = ACCOUNT.get_mut(&user_id).ok_or(TokenExpiredError)?;
        if entry.value() != &old { return Err(PasswordMismatchedError.into()); }
        *entry.value_mut() = new.to_string();
        Ok(())
    }
}