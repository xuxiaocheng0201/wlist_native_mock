use crate::api::common::data::files::confirmations::FRefreshConfirmation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::files::progresses::FRefreshProgress;
use crate::api::common::data::files::tokens::FRefreshToken;
use crate::api::core::client::define_func;

define_func!(
    /// Refresh the directory.
    ///
    /// Notice that the refresh token will lock the directory until it is canceled/finished.
    ///
    /// directory: .isDirectory == true
    refresh_request(directory: FFileLocation) -> FRefreshConfirmation = wlist_native::core::client::refresh::refresh_request
);
define_func!(
    /// Cancel a refresh.
    ///
    /// What ever the refresh is paused or not, or not confirmed, it will be canceled.
    refresh_cancel(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_cancel
);
define_func!(
    /// Confirm a refresh.
    ///
    /// Then the refresh is automatically resumed.
    refresh_confirm(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_confirm
);
define_func!(
    /// Pause a refresh.
    ///
    /// If refresh has been paused, the method will return normally.
    refresh_pause(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_pause
);
define_func!(
    /// Resume a refresh.
    ///
    /// If refresh has been resumed, the method will return normally.
    refresh_resume(token: FRefreshToken) -> () = wlist_native::core::client::refresh::refresh_resume
);
define_func!(
    /// Check whether a refresh is paused.
    refresh_is_paused(token: FRefreshToken) -> bool = wlist_native::core::client::refresh::refresh_is_paused
);
define_func!(
    /// Get the progress of refresh.
    ///
    /// Notice that if the refresh is finished/canceled, it will return [TokenExpiredError](crate::api::common::exceptions::UniverseError::TokenExpiredError).
    refresh_progress(token: FRefreshToken) -> FRefreshProgress = wlist_native::core::client::refresh::refresh_progress
);
define_func!(
    /// Check the refresh state.
    ///
    /// returns: true means the refresh is finished, false means the refresh is in progress (or be paused).
    ///
    /// Notice you must call this method to release resources.
    ///
    /// If the refresh progress returns any error, it will return the error (and the resources are released auto).
    /// If this method returned true (or returned the error above),
    /// the next time call will return [TokenExpiredError](crate::api::common::exceptions::UniverseError::TokenExpiredError).
    refresh_check(token: FRefreshToken) -> bool = wlist_native::core::client::refresh::refresh_check
);
