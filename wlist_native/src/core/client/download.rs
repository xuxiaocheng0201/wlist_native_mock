use bytes::BufMut;
use tokio::sync::watch::{Receiver, Sender};

use crate::common::data::files::confirmations::DownloadConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::files::information::DownloadInformation;
use crate::common::data::files::tokens::DownloadToken;
use crate::core::client::context::define_func;

define_func!(download_request(login_context, file: FileLocation, from: u64, to: u64) -> DownloadConfirmation = {
    unimplemented!()
});
define_func!(download_cancel(login_context, token: DownloadToken) -> () = {
    unimplemented!()
});
define_func!(download_confirm(login_context, token: DownloadToken) -> DownloadInformation = {
    unimplemented!()
});
define_func!(download_stream(login_context, token: DownloadToken, id: u64, start: u64, buffer: &mut impl BufMut, transferred_bytes: Sender<usize>, control: Receiver<bool>) -> () = {
    unimplemented!()
});
define_func!(download_finish(login_context, token: DownloadToken) -> () = {
    unimplemented!()
});
