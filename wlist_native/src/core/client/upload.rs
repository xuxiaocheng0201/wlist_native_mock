use bytes::Bytes;
use tokio::sync::watch::{Receiver, Sender};

use crate::common::data::files::confirmations::UploadConfirmation;
use crate::common::data::files::information::{FileInformation, UploadInformation};
use crate::common::data::files::options::Duplicate;
use crate::common::data::files::tokens::UploadToken;
use crate::common::data::files::FileLocation;
use crate::core::client::context::define_func;

define_func!(upload_check_name(login_context, name: String, parent: FileLocation, is_directory: bool) -> () = {
    unimplemented!()
});

define_func!(upload_mkdir(login_context, parent: FileLocation, name: String, duplicate: Duplicate) -> FileInformation = {
    unimplemented!()
});

define_func!(upload_request(login_context, parent: FileLocation, name: String, size: u64, md5: String, md5s: Vec<String>, duplicate: Duplicate) -> UploadConfirmation = {
    unimplemented!()
});
define_func!(upload_cancel(login_context, token: UploadToken) -> () = {
    unimplemented!()
});
define_func!(upload_confirm(login_context, token: UploadToken) -> UploadInformation = {
    unimplemented!()
});
define_func!(upload_stream(login_context, token: UploadToken, id: u64, buffer: &mut Bytes, transferred_bytes: Sender<usize>, control: Receiver<bool>) -> () = {
    unimplemented!()
});
define_func!(upload_finish(login_context, token: UploadToken) -> FileInformation = {
    unimplemented!()
});
