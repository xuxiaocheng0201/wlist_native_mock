use std::cmp::min;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bytes::BufMut;
use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::watch::{Receiver, Sender};

use crate::common::data::files::confirmations::DownloadConfirmation;
use crate::common::data::files::information::{DownloadChunkInformation, DownloadInformation};
use crate::common::data::files::tokens::DownloadToken;
use crate::common::data::files::FileLocation;
use crate::common::data::storages::StorageType;
use crate::common::exceptions::{IncorrectArgumentError, TokenExpiredError};
use crate::core::client::context::define_func;
use crate::core::helper::utils::generate_string;

fn get_file_content() -> String {
    // 35840 B
    "The download file content in mock.\n".repeat(1024)
}

static DOWNLOAD_MAP: Lazy<DashMap<String, (AtomicBool, u64, u64)>> = Lazy::new(DashMap::new);

define_func!(download_request(login_context, file: FileLocation, from: u64, to: u64) -> DownloadConfirmation = {
    let token = generate_string(32);
    let to = min(to, 35840);
    let size = to + 1 - from;
    DOWNLOAD_MAP.insert(token.clone(), (AtomicBool::new(false), from, size));
    Ok(DownloadConfirmation {
        size,
        token: DownloadToken {
            storage: file.storage,
            r#type: StorageType::Lanzou,
            token: Arc::new(token),
        },
    })
});
define_func!(download_cancel(login_context, token: DownloadToken) -> () = {
    DOWNLOAD_MAP.remove(token.token.as_str()).ok_or(TokenExpiredError)?;
    Ok(())
});
define_func!(download_confirm(login_context, token: DownloadToken) -> DownloadInformation = {
    let entry = DOWNLOAD_MAP.get(token.token.as_str()).ok_or(TokenExpiredError)?;
    entry.0.store(true, Ordering::Relaxed);
    Ok(DownloadInformation {
        chunks: vec![DownloadChunkInformation {
            range: true,
            start: entry.1,
            size: entry.2,
        }],
        expire: Utc::now() + chrono::Duration::minutes(30),
    })
});
define_func!(download_stream(login_context, token: DownloadToken, id: u64, start: u64, buffer: &mut impl BufMut, transferred_bytes: Sender<usize>, control: Receiver<bool>) -> () = {
    let entry = DOWNLOAD_MAP.remove(token.token.as_str()).ok_or(TokenExpiredError)?.1;
    if !entry.0.load(Ordering::Relaxed) { return Err(TokenExpiredError.into()); }
    if id != 0 {
        return Err(IncorrectArgumentError::new("invalid chunk id".into()).into());
    }

    let len = buffer.remaining_mut();
    let content = get_file_content();
    let content = &content.split_at(start as usize).1.as_bytes()[..len];
    buffer.put_slice(content);

    let mut control = control;
    for t in (0..len).step_by(1 << 10) {
        let _ = transferred_bytes.send(t);
        tokio::time::sleep(Duration::from_millis(200)).await;
        while !*control.borrow_and_update() {
            if control.changed().await.is_err() {
                break;
            }
        }
    }
    let _ = transferred_bytes.send(len);
    Ok(())
});
define_func!(download_finish(login_context, token: DownloadToken) -> () = {
    let entry = DOWNLOAD_MAP.remove(token.token.as_str()).ok_or(TokenExpiredError)?.1;
    if !entry.0.load(Ordering::Relaxed) { return Err(TokenExpiredError.into()); }
    Ok(())
});
