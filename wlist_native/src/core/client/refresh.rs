use crate::common::data::files::confirmations::RefreshConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::files::progresses::RefreshProgress;
use crate::common::data::files::tokens::RefreshToken;
use crate::core::client::context::define_func;

define_func!(refresh_request(login_context, directory: FileLocation) -> RefreshConfirmation = {
    unimplemented!()
});
define_func!(refresh_cancel(login_context, token: RefreshToken) -> () = {
    unimplemented!()
});
define_func!(refresh_confirm(login_context, token: RefreshToken) -> () = {
    unimplemented!()
});
define_func!(refresh_pause(login_context, token: RefreshToken) -> () = {
    unimplemented!()
});
define_func!(refresh_resume(login_context, token: RefreshToken) -> () = {
    unimplemented!()
});
define_func!(refresh_is_paused(login_context, token: RefreshToken) -> bool = {
    unimplemented!()
});
define_func!(refresh_progress(login_context, token: RefreshToken) -> RefreshProgress = {
    unimplemented!()
});
define_func!(refresh_check(login_context, token: RefreshToken) -> bool = {
    unimplemented!()
});
