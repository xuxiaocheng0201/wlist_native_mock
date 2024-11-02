use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::ToSocketAddrs;

#[derive(Debug)]
pub struct WlistServer {
    _private_construct: ()
}

impl WlistServer {
    pub fn local_addr(&self) -> SocketAddr {
        unimplemented!("WlistServer is not implemented in mock.")
    }

    pub async fn start(_addr: impl ToSocketAddrs) -> Result<Self> {
        Err(anyhow::anyhow!("WlistServer is not implemented in mock."))
    }

    pub async fn stop(self) -> Result<()> {
        unimplemented!("WlistServer is not implemented in mock.")
    }
}

pub mod users {
    use anyhow::Result;

    pub async fn reset_admin_password() -> Result<String> {
        unimplemented!()
    }
}
