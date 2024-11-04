use std::fmt::Debug;
use std::marker::PhantomData;

use anyhow::Result;
use tokio::net::ToSocketAddrs;

macro_rules! define_func {
    ($func: ident($($request: ident: $req: ty),*) -> $res: ty) => {
        pub async fn $func(client: &mut Option<&mut $crate::core::client::WlistClient<'_>>, $($request: $req),*) -> ::anyhow::Result<$res> {
            unimplemented!()
        }
    };
}
use define_func;

pub mod users;
pub mod storages;
pub mod files;
pub mod refresh;
pub mod download;
pub mod upload;
pub mod trash;
pub mod search;

#[derive(Debug, Clone)]
pub struct WlistClientManager<A: ToSocketAddrs + Debug + Clone + Send + Sync> {
    _addr: A,
}

pub struct WlistClient<'a> {
    _lifetime: PhantomData<&'a ()>,
}

impl<A: ToSocketAddrs + Debug + Clone + Send + Sync> WlistClientManager<A> {
    pub async fn new(addr: A) -> Result<Self> {
        unimplemented!()
    }

    pub async fn get(&self) -> Result<WlistClient> {
        unimplemented!()
    }
}
