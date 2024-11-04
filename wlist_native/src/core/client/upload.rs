use bytes::Bytes;
use tokio::sync::watch::{Receiver, Sender};

use crate::common::data::files::confirmations::UploadConfirmation;
use crate::common::data::files::information::{FileInformation, UploadInformation};
use crate::common::data::files::options::Duplicate;
use crate::common::data::files::tokens::UploadToken;
use crate::common::data::files::FileLocation;
use crate::core::client::define_func;

define_func!(upload_check_name(name: String, parent: FileLocation, is_directory: bool) -> ());

define_func!(upload_mkdir(parent: FileLocation, name: String, duplicate: Duplicate) -> FileInformation);

define_func!(upload_request(parent: FileLocation, name: String, size: u64, md5: String, md5s: Vec<String>, duplicate: Duplicate) -> UploadConfirmation);
define_func!(upload_cancel(token: UploadToken) -> ());
define_func!(upload_confirm(token: UploadToken) -> UploadInformation);
define_func!(upload_stream(token: UploadToken, id: u64, buffer: &mut Bytes, transferred_bytes: Sender<usize>, control: Receiver<bool>) -> ());
define_func!(upload_finish(token: UploadToken) -> FileInformation);
