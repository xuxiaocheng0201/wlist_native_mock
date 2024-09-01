use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static ACCOUNT: Lazy<DashMap<String, String>> = Lazy::new(|| DashMap::new());
static LOGIN_STATUS: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

pub mod account;
pub mod register;
pub mod user;
pub mod version;
