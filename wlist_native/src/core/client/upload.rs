use std::cmp::min;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

use bytes::{Buf, Bytes};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use rand::random;
use tokio::sync::watch::{Receiver, Sender};

use crate::common::data::files::confirmations::UploadConfirmation;
use crate::common::data::files::FileLocation;
use crate::common::data::files::information::{FileInformation, UploadChunkInformation, UploadInformation};
use crate::common::data::files::options::Duplicate;
use crate::common::data::files::tokens::UploadToken;
use crate::common::data::storages::StorageType;
use crate::common::exceptions::{IncorrectArgumentError, TokenExpiredError};
use crate::core::client::context::define_func;
use crate::core::helper::utils::generate_string;

define_func!(upload_check_name(login_context, name: String, parent: FileLocation, is_directory: bool) -> () = {
    if name.len() < 1 || 32767 < name.len() {
        return Err(IncorrectArgumentError::new("name must be between 0 and 32767 characters long".into()).into());
    }
    if name.contains('/') || name.contains('\\') || name.contains(":") {
        return Err(IncorrectArgumentError::new("name must not contain '/' or '\\' or ':'".into()).into());
    }
    // No other check in mock.
    Ok(())
});

define_func!(upload_mkdir(login_context, parent: FileLocation, name: String, duplicate: Duplicate) -> FileInformation = {
    Ok(FileInformation {
        id: random(),
        parent_id: parent.file_id,
        name: Arc::new(name),
        is_directory: true,
        size: None,
        create_time: Some(Utc::now()),
        update_time: Some(Utc::now()),
    })
});

static UPLOAD_MAP: Lazy<DashMap<String, (AtomicBool, String, AtomicU64, u64, i64)>> = Lazy::new(DashMap::new);

define_func!(upload_request(login_context, parent: FileLocation, name: String, size: u64, md5: String, md5s: Vec<String>, duplicate: Duplicate) -> UploadConfirmation = {
    if name.len() < 1 || 32767 < name.len() {
        return Err(IncorrectArgumentError::new("name must be between 0 and 32767 characters long".into()).into());
    }
    if name.contains('/') || name.contains('\\') || name.contains(":") {
        return Err(IncorrectArgumentError::new("name must not contain '/' or '\\' or ':'".into()).into());
    }
    if md5.len() != 32 || md5s.len() == 0 {
        return Err(IncorrectArgumentError::new("md5 must be 32 characters long and md5s must not be empty".into()).into());
    }

    let token = generate_string(32);
    UPLOAD_MAP.insert(token.clone(), (AtomicBool::new(false), name, AtomicU64::new(0), size, parent.file_id));
    Ok(UploadConfirmation {
        done: false,
        token: UploadToken {
            storage: parent.storage,
            r#type: StorageType::Lanzou,
            token: Arc::new(token),
        },
    })
});
define_func!(upload_cancel(login_context, token: UploadToken) -> () = {
    UPLOAD_MAP.remove(token.token.as_str()).ok_or(TokenExpiredError)?;
    Ok(())
});
define_func!(upload_confirm(login_context, token: UploadToken) -> UploadInformation = {
    let entry = UPLOAD_MAP.get(token.token.as_str()).ok_or(TokenExpiredError)?;
    entry.0.store(true, Ordering::Relaxed);
    Ok(UploadInformation {
        chunks: vec![UploadChunkInformation {
            start: 0,
            size: entry.3,
        }],
        expire: DateTime::<Utc>::MAX_UTC,
    })
});
define_func!(upload_stream(login_context, token: UploadToken, id: u64, buffer: &mut Bytes, transferred_bytes: Sender<usize>, control: Receiver<bool>) -> () = {
    let entry = UPLOAD_MAP.get(token.token.as_str()).ok_or(TokenExpiredError)?;
    if !entry.0.load(Ordering::Relaxed) { return Err(TokenExpiredError.into()); }
    if id != 0 {
        return Err(IncorrectArgumentError::new("invalid chunk id".into()).into());
    }
    let mut transffered = 0;
    let mut control = control;
    while buffer.has_remaining() {
        let len = min(buffer.remaining(), 1 << 10);
        buffer.advance(len);
        transffered += len;
        let len = len as u64;
        if entry.2.fetch_add(len, Ordering::Relaxed) + len >= entry.3 {
            break;
        }
        let _ = transferred_bytes.send(len as usize);
        tokio::time::sleep(Duration::from_millis(200)).await;
        while !*control.borrow_and_update() {
            if control.changed().await.is_err() {
                break;
            }
        }
    }
    Ok(())
});
define_func!(upload_finish(login_context, token: UploadToken) -> FileInformation = {
    let entry = UPLOAD_MAP.remove(token.token.as_str()).ok_or(TokenExpiredError)?.1;
    if !entry.0.load(Ordering::Relaxed) { return Err(TokenExpiredError.into()); }
    Ok(FileInformation {
        id: random(),
        parent_id: entry.4,
        name: Arc::new(entry.1),
        is_directory: false,
        size: Some(entry.3),
        create_time: Some(Utc::now()),
        update_time: Some(Utc::now()),
    })
});
