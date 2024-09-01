use tokio::sync::watch::Sender;

use crate::api::common::exceptions::UniverseError;

pub mod users;
pub mod storages;
pub mod files;
pub mod refresh;
pub mod download;
pub mod upload;
pub mod trash;

#[flutter_rust_bridge::frb(opaque)]
/// The core client manager.
///
/// For the `client` parameter: null is calling the internal core server directly.
#[derive(Clone)]
pub struct WlistClientManager {
    manager: wlist_native::core::client::WlistClientManager<String>,
}

impl WlistClientManager {
    /// Create a new core client manager.
    ///
    /// addr: should be the string returned by [local_addr](crate::api::core::server::WlistServer::local_addr).
    pub async fn connect(addr: String) -> Result<Self, UniverseError> {
        let manager = wlist_native::core::client::WlistClientManager::new(addr).await?;
        Ok(Self { manager })
    }
}

macro_rules! define_func {
    ($(#[$doc: meta])* $func: ident($($para: ident: $ty: ty),*) -> $res: ty = $target: expr) => {
        $(#[$doc])*
        pub async fn $func(client: Option<$crate::api::core::client::WlistClientManager>, $($para: $ty),*) -> Result<$res, $crate::api::common::exceptions::UniverseError> {
            let mut client = match &client {
                None => None, Some(manager) => Some(manager.manager.get().await?)
            };
            $target(&mut client.as_mut(), $($para.into()),*).await.map(Into::into).map_err(Into::into)
        }
    };
}
use define_func;

#[flutter_rust_bridge::frb(opaque)]
/// The download/upload pause/resume controller.
///
/// A controller can be passed into many functions.
#[derive(Clone)]
pub struct PauseController {
    sender: Sender<bool>,
}

impl PauseController {
    /// Create a new controller. (default is resumed.)
    pub fn new() -> Self {
        Self { sender: tokio::sync::watch::channel(true).0 }
    }

    /// Pause the task.
    pub fn pause(&self) {
        let _ = self.sender.send(false);
    }

    /// Resume the task.
    pub fn resume(&self) {
        let _ = self.sender.send(true);
    }
}
