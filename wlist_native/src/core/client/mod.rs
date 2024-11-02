#![allow(unused_variables)]

use std::fmt::Debug;
use std::marker::PhantomData;

use anyhow::Result;
use tokio::net::ToSocketAddrs;

mod context {
    use anyhow::Result;
    use once_cell::sync::Lazy;

    #[derive(Default, Clone)]
    pub struct ClientContext(());

    impl ClientContext {
        pub async fn check_is_login(&self) -> Result<()> {
            unimplemented!()
        }
    }

    pub static NATIVE_CONTEXT: Lazy<ClientContext> = Lazy::new(|| ClientContext::default());

    macro_rules! define_func {
        ($func: ident(login_context, $($request: ident: $req: ty),*) -> $res: ty = $($body: tt)+) => {
            define_func!($func(context, $($request: $req),*) -> $res = {
                context.check_is_login().await?;
                $($body)+
            });
        };
        ($func: ident($context: ident, $($request: ident: $req: ty),*) -> $res: ty = $($body: tt)+) => {
            pub async fn $func(_client: &mut Option<&mut $crate::core::client::WlistClient<'_>>, $($request: $req),*) -> ::anyhow::Result<$res> {
                let $context = &$crate::core::client::context::NATIVE_CONTEXT;
                $($body)+
            }
        };
    }
    pub(crate) use define_func;
}

pub mod users;
pub mod storages;
pub mod files;
pub mod refresh;
pub mod download;
pub mod upload;
pub mod trash;

#[derive(Debug, Clone)]
pub struct WlistClientManager<A: ToSocketAddrs + Debug + Clone + Send + Sync> {
    _addr: A,
}

pub struct WlistClient<'a> {
    _lifetime: PhantomData<&'a ()>,
}

impl<A: ToSocketAddrs + Debug + Clone + Send + Sync> WlistClientManager<A> {
    #[inline]
    pub async fn new(_addr: A) -> Result<Self> {
        Err(anyhow::anyhow!("WlistClientManager is not implemented in mock."))
    }

    #[inline]
    pub async fn get<'a>(&'a self) -> Result<WlistClient> {
        unimplemented!("WlistClientManager is not implemented in mock.")
    }
}
