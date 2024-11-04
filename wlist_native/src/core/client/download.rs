use bytes::BufMut;
use tokio::sync::watch::{Receiver, Sender};

use crate::common::data::files::confirmations::DownloadConfirmation;
use crate::common::data::files::information::DownloadInformation;
use crate::common::data::files::tokens::DownloadToken;
use crate::common::data::files::FileLocation;
use crate::core::client::define_func;

define_func!(download_request(file: FileLocation, from: u64, to: u64) -> DownloadConfirmation);
define_func!(download_cancel(token: DownloadToken) -> ());
define_func!(download_confirm(token: DownloadToken) -> DownloadInformation);
define_func!(download_stream(token: DownloadToken, id: u64, start: u64, buffer: &mut impl BufMut, transferred_bytes: Sender<usize>, control: Receiver<bool>) -> ());
define_func!(download_finish(token: DownloadToken) -> ());
