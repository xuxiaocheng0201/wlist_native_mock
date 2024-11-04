pub mod login {
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub async fn login<'i, 'p>(user_id: impl Into<Cow<'i, str>> + Debug, password: impl Into<Cow<'p, str>> + Debug) -> anyhow::Result<()> {
        unimplemented!()
    }
}

pub mod logout {
    pub async fn logout() -> anyhow::Result<()> {
        unimplemented!()
    }
}

pub mod is_logged {
    pub async fn is_logged() -> anyhow::Result<bool> {
        unimplemented!()
    }
}
