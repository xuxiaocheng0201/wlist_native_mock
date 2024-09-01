/// The version of the common API.
/// Only displayed in detail version information.
#[flutter_rust_bridge::frb(sync, getter, name = "common_version")]
pub fn get_common_api_version() -> String {
    wlist_native::common::versions::get_common_api_version()
}

/// The version of the web API.
/// Only displayed in detail version information.
#[flutter_rust_bridge::frb(sync, getter, name = "web_version")]
pub fn get_web_api_version() -> String {
    wlist_native::common::versions::get_web_api_version()
}

/// The version of the native API.
/// Only displayed in detail version information.
#[flutter_rust_bridge::frb(sync, getter, name = "core_version")]
pub fn get_core_api_version() -> String {
    wlist_native::common::versions::get_core_api_version()
}
