use crate::common::data::files::confirmations::RefreshConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::files::progresses::RefreshProgress;
use crate::common::data::files::tokens::RefreshToken;
use crate::core::client::define_func;

define_func!(refresh_request(directory: FileLocation) -> RefreshConfirmation);
define_func!(refresh_cancel(token: RefreshToken) -> ());
define_func!(refresh_confirm(token: RefreshToken) -> ());
define_func!(refresh_pause(token: RefreshToken) -> ());
define_func!(refresh_resume(token: RefreshToken) -> ());
define_func!(refresh_is_paused(token: RefreshToken) -> bool);
define_func!(refresh_progress(token: RefreshToken) -> RefreshProgress);
define_func!(refresh_check(token: RefreshToken) -> bool);
