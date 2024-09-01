use std::sync::Arc;

use wlist_native::common::data::storages::StorageType;

#[flutter_rust_bridge::frb(opaque)]
/// The refresh token.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::tokens::RefreshToken)]
pub struct FRefreshToken {
    storage: i64,
    token: Arc<String>,
}

#[flutter_rust_bridge::frb(opaque)]
/// The download token.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::tokens::DownloadToken)]
pub struct FDownloadToken {
    storage: i64,
    r#type: StorageType,
    token: Arc<String>,
}

#[flutter_rust_bridge::frb(opaque)]
/// The upload token.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::common::data::files::tokens::UploadToken)]
pub struct FUploadToken {
    storage: i64,
    r#type: StorageType,
    token: Arc<String>,
}
