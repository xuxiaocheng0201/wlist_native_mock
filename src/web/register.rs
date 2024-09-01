pub mod as_guest {
    use std::borrow::Cow;
    use std::fmt::Debug;

    use dashmap::Entry;

    use crate::common::exceptions::IncorrectArgumentError;
    use crate::web::ACCOUNT;

    pub async fn register_as_guest<'d, 'p>(device_id: impl Into<Cow<'d, str>> + Debug, password: impl Into<Cow<'p, str>> + Debug) -> anyhow::Result<Option<String>> {
        let device_id = device_id.into();
        let password = password.into();
        if device_id.len() < 16 { return Err(IncorrectArgumentError::new("device_id is too short".into()).into()); }
        if device_id.len() > 256 { return Err(IncorrectArgumentError::new("device_id is too large".into()).into()); }
        if password.len() < 6 { return Err(IncorrectArgumentError::new("password is too simple".into()).into()); }
        if password.len() > 128 { return Err(IncorrectArgumentError::new("password is too long".into()).into()); }
        let user_id = device_id.split_at(device_id.len() / 2).0.to_string();
        let entry = ACCOUNT.entry(user_id);
        if let Entry::Occupied(_) = &entry { return Ok(None); }
        let user_id = entry.or_insert(password.to_string()).key().to_string();
        Ok(Some(user_id))
    }
}

pub mod unregister {
    use std::borrow::Cow;
    use std::fmt::Debug;

    use crate::common::exceptions::{PasswordMismatchedError, TokenExpiredError};
    use crate::web::{ACCOUNT, LOGIN_STATUS};

    pub async fn unregister<'p>(password: impl Into<Cow<'p, str>> + Debug) -> anyhow::Result<()> {
        let password = password.into();
        if password.len() < 6 { return Err(PasswordMismatchedError.into()); }
        if password.len() > 128 { return Err(PasswordMismatchedError.into()); }
        let user_id = LOGIN_STATUS.read().await.clone().ok_or(TokenExpiredError)?;
        ACCOUNT.remove_if(&user_id, |_k, v| v == &password)
            .map(|_| ()).ok_or(PasswordMismatchedError.into())
    }
}
