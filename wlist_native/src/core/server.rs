use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::ToSocketAddrs;

#[derive(Debug)]
pub struct WlistServer(());

impl WlistServer {
    pub fn local_addr(&self) -> SocketAddr {
        unimplemented!()
    }

    pub async fn start(addr: impl ToSocketAddrs) -> Result<Self> {
        unimplemented!()
    }

    pub async fn stop(self) -> Result<()> {
        unimplemented!()
    }
}

pub mod users {
    use anyhow::Result;

    pub async fn reset_admin_password() -> Result<String> {
        unimplemented!()
    }
}
