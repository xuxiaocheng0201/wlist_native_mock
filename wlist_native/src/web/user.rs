pub mod nickname {
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub async fn get_nickname() -> anyhow::Result<String> {
        unimplemented!()
    }

    pub async fn set_nickname<'n>(nickname: impl Into<Cow<'n, str>> + Debug) -> anyhow::Result<()> {
        unimplemented!()
    }
}

pub mod password {
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub async fn reset_password<'o, 'n>(old: impl Into<Cow<'o, str>> + Debug, new: impl Into<Cow<'n, str>> + Debug) -> anyhow::Result<()> {
        unimplemented!()
    }
}
