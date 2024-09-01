use crate::api::common::exceptions::UniverseError;

pub mod users;

#[flutter_rust_bridge::frb(opaque)]
/// The core server.
pub struct WlistServer {
    server: wlist_native::core::server::WlistServer,
}

impl WlistServer {
    /// Returns the server's local address.
    pub fn local_addr(&self) -> String {
        self.server.local_addr().to_string()
    }

    /// Open the core server on address.
    ///
    /// addr: can be `localhost:0`
    pub async fn start(addr: String) -> Result<Self, UniverseError> {
        let server = wlist_native::core::server::WlistServer::start(addr).await?;
        Ok(Self { server })
    }

    /// Stop the core server.
    pub async fn stop(self) -> Result<(), UniverseError> {
        self.server.stop().await.map_err(Into::into)
    }
}
