#[flutter_rust_bridge::frb(sync, getter, name = "getCommonVersion")]
/// The version of the common API.
/// Only displayed in detail version information.
pub fn get_common_api_version() -> String {
    wlist_native::common::versions::get_common_api_version()
}

#[flutter_rust_bridge::frb(sync, getter, name = "getWebVersion")]
/// The version of the web API.
/// Only displayed in detail version information.
pub fn get_web_api_version() -> String {
    wlist_native::common::versions::get_web_api_version()
}

#[flutter_rust_bridge::frb(sync, getter, name = "getCoreVersion")]
/// The version of the native API.
/// Only displayed in detail version information.
pub fn get_core_api_version() -> String {
    wlist_native::common::versions::get_core_api_version()
}
