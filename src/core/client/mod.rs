use std::fmt::Debug;
use std::marker::PhantomData;

use anyhow::Result;
use tokio::net::ToSocketAddrs;

mod context {
    use std::ops::{Deref, DerefMut};
    use std::sync::Arc;

    use anyhow::Result;
    use once_cell::sync::Lazy;
    use tokio::sync::RwLock;

    use crate::common::exceptions::TokenExpiredError;

    #[derive(Default, Clone)]
    pub struct ClientContext(Arc<RwLock<ClientContextInner>>);

    impl ClientContext {
        pub async fn read_inner(&self) -> impl Deref<Target = ClientContextInner> {
            Arc::clone(&self.0).read_owned().await
        }

        pub async fn write_inner(&self) -> impl DerefMut<Target = ClientContextInner> {
            Arc::clone(&self.0).write_owned().await
        }

        pub async fn check_is_login(&self) -> Result<()> {
            if self.read_inner().await.login_status.is_some() {
                Ok(())
            } else {
                Err(TokenExpiredError.into())
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct ClientContextInner {
        pub login_status: Option<()>,
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
#[allow(unused_variables)] // TODO
pub mod files;
#[allow(unused_variables)] // TODO
pub mod refresh;
#[allow(unused_variables)] // TODO
pub mod download;
#[allow(unused_variables)] // TODO
pub mod upload;
#[allow(unused_variables)] // TODO
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
