use chrono::{DateTime, Utc};

pub mod register {
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub async fn register_with_mail_untrusted<'a>(mail: impl Into<Cow<'a, str>> + Debug) -> anyhow::Result<Option<String>> {
        unimplemented!()
    }
    pub async fn register_with_mail<'a>(ident: impl Into<Cow<'a, str>> + Debug, code: impl Into<Cow<'a, str>> + Debug, password: impl Into<Cow<'a, str>> + Debug) -> anyhow::Result<anyhow::Result<String, bool>> {
        unimplemented!()
    }

    pub async fn unregister<'p>(password: impl Into<Cow<'p, str>> + Debug) -> anyhow::Result<()> {
        unimplemented!()
    }
}

pub mod account {
    use std::borrow::Cow;
    use std::fmt::Debug;

    use crate::web::users::User;

    pub async fn is_logged_in() -> anyhow::Result<bool> {
        unimplemented!()
    }

    pub async fn login<'a>(id: impl Into<Cow<'a, str>> + Debug, password: impl Into<Cow<'a, str>> + Debug) -> anyhow::Result<Option<User>> {
        unimplemented!()
    }

    pub async fn login_with_mail<'a>(mail: impl Into<Cow<'a, str>> + Debug, password: impl Into<Cow<'a, str>> + Debug) -> anyhow::Result<Option<User>> {
        unimplemented!()
    }

    pub async fn logout() -> anyhow::Result<()> {
        unimplemented!()
    }
}

pub mod info {
    use std::borrow::Cow;
    use std::fmt::Debug;

    use crate::web::users::UserRole;

    pub async fn get_user_info() -> anyhow::Result<(String, UserRole)> {
        unimplemented!()
    }

    pub async fn set_nickname<'a>(nickname: impl Into<Cow<'a, str>> + Debug) -> anyhow::Result<()> {
        unimplemented!()
    }
}

pub mod password {
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub async fn reset_password<'a>(old: impl Into<Cow<'a, str>> + Debug, new: impl Into<Cow<'a, str>> + Debug) -> anyhow::Result<Option<()>> {
        unimplemented!()
    }

    pub async fn reset_password_with_mail_untrusted() -> anyhow::Result<String> {
        unimplemented!()
    }
    pub async fn reset_password_with_mail<'a>(ident: impl Into<Cow<'a, str>> + Debug, code: impl Into<Cow<'a, str>> + Debug, password: impl Into<Cow<'a, str>> + Debug) -> anyhow::Result<Option<()>> {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserRole {
    Admin,
    Alpha,
    Beta,
    Normal,
    Untrusted,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub create_time: DateTime<Utc>,
    pub role: UserRole,
    pub mail: String,
}
