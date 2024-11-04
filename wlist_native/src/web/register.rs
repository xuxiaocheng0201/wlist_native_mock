pub mod as_guest {
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub async fn register_as_guest<'d, 'p>(device_id: impl Into<Cow<'d, str>> + Debug, password: impl Into<Cow<'p, str>> + Debug) -> anyhow::Result<Option<String>> {
        unimplemented!()
    }
}

pub mod unregister {
    use std::borrow::Cow;
    use std::fmt::Debug;

    pub async fn unregister<'p>(password: impl Into<Cow<'p, str>> + Debug) -> anyhow::Result<()> {
        unimplemented!()
    }
}
