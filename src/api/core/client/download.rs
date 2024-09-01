use std::fs::File;

use crate::api::common::data::files::confirmations::FDownloadConfirmation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::files::information::FDownloadInformation;
use crate::api::common::data::files::tokens::FDownloadToken;
use crate::api::common::exceptions::UniverseError;
use crate::api::core::client::{define_func, PauseController, WlistClientManager};

define_func!(
    /// Request to download the file.
    ///
    /// Notice that the upload token will lock the file until it is canceled/finished.
    ///
    ///
    /// file: .isDirectory == false
    ///
    /// from: the start byte index of the entire file. (include) (0 <= from <= to)
    ///
    /// to: the last byte index of the entire file. (include) (For entire file, you can pass a value large enough.)
    download_request(file: FFileLocation, from: u64, to: u64) -> FDownloadConfirmation = wlist_native::core::client::download::download_request
);
define_func!(
    /// Cancel a download.
    ///
    /// What ever the download is paused or not, or not confirmed, it will be canceled.
    download_cancel(token: FDownloadToken) -> () = wlist_native::core::client::download::download_cancel
);
define_func!(
    /// Confirm a download.
    ///
    /// Then the download is automatically resumed.
    download_confirm(token: FDownloadToken) -> FDownloadInformation = wlist_native::core::client::download::download_confirm
);
define_func!(
    /// Finish a download.
    ///
    /// This function is similar to call [download_cancel], but marks the download as finished.
    download_finish(token: FDownloadToken) -> () = wlist_native::core::client::download::download_finish
);

/// flutter_rust_bridge:ignore
mod internal {
    use bytes::BufMut;
    use tokio::sync::watch::Receiver;
    use super::*;

    define_func!(download_stream(token: FDownloadToken, id: u64, start: u64, buffer: &mut impl BufMut, control: Receiver<bool>) -> () = wlist_native::core::client::download::download_stream);
}

/// Download the file chunk.
///
///
/// id: see the `chunks` field in [FDownloadInformation]. (0 <= id < chunks_length)
///
/// start: the start position to download of this chunk. (0 <= start <= chunk_size)
///
/// buffer: a pointer to the buffer to write the data.
pub async fn download_stream(client: Option<WlistClientManager>, token: FDownloadToken, id: u64, start: u64, buffer: MutU8, buffer_size: usize, control: PauseController) -> Result<(), UniverseError> {
    let mut buffer = unsafe { wlist_native::core::helper::buffer::WriteBuffer::new(buffer.0, buffer_size) };
    internal::download_stream(client, token, id, start, &mut buffer, control.sender.subscribe()).await
}


#[flutter_rust_bridge::frb(opaque)]
/// Native pointer of `*mut u8`.
#[derive(Copy, Clone)]
pub struct MutU8(*mut u8);

unsafe impl Send for MutU8 { }
unsafe impl Sync for MutU8 { }


#[flutter_rust_bridge::frb(opaque)]
/// The internal resource of the allocated buffer.
pub struct AllocatedBufferResource {
    buffer: Vec<u8>,
}

/// Allocate a buffer in memory.
///
///
/// len: the length of the buffer.
///
/// returns: a pointer to the buffer and the internal resource.
pub fn allocate_buffer(len: usize) -> (MutU8, AllocatedBufferResource) {
    let mut buffer = Vec::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    (MutU8(ptr), AllocatedBufferResource { buffer })
}

/// Drop the buffer in memory.
pub fn drop_buffer(resource: AllocatedBufferResource) {
    drop(resource.buffer);
}

#[flutter_rust_bridge::frb(opaque)]
/// The internal resource of the mapped buffer.
pub struct MappedBufferResource {
    file: File,
    mmap: memmap2::MmapRaw,
}

/// Map the buffer in file.
///
/// You should ensure the file len is larger than `offset + len - 1`.
///
///
/// file: the path of the file.
///
/// offset: the offset of the file to map.
///
/// len: the length of the buffer.
///
/// returns: a pointer to the buffer and the internal resource.
pub fn map_buffer(file: String, offset: u64, len: usize) -> Result<(MutU8, MappedBufferResource), UniverseError> {
    let file = File::options().read(true).write(true).create(true).open(&file).map_err(anyhow::Error::from)?;
    let mmap = memmap2::MmapOptions::new()
        .offset(offset).len(len)
        .map_raw(&file).map_err(anyhow::Error::from)?;
    Ok((MutU8(mmap.as_mut_ptr()), MappedBufferResource { file, mmap }))
}

/// Drop the mapped buffer from file.
///
/// After this function returns, all content of the buffer will be flushed in the file.
pub fn drop_buffer_mapped(resource: MappedBufferResource) -> Result<(), UniverseError> {
    resource.mmap.flush().map_err(anyhow::Error::from)?;
    drop(resource.mmap);
    drop(resource.file);
    Ok(())
}


/// Read the buffer.
///
/// Same as [clone_buffer](crate::api::core::client::upload::clone_buffer),
/// but provide a [MutU8] version.
///
///
/// ptr: the pointer to the buffer.
///
/// len: the length of the buffer.
pub fn clone_buffer(ptr: MutU8, len: usize) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts_mut(ptr.0, len) }.to_vec()
}
