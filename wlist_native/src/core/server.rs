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
    use once_cell::sync::Lazy;
    use tokio::sync::RwLock;

    use crate::core::helper::utils::generate_string;

    static PASSWORD: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(generate_string(8)));

    pub(crate) async fn login(username: &str, password: &str) -> Result<bool> {
        if username != "admin" { return Ok(false); }
        Ok(*PASSWORD.read().await == password)
    }

    pub async fn reset_admin_password() -> Result<String> {
        let password = generate_string(8);
        *PASSWORD.write().await = password.clone();
        Ok(password)
    }
}
