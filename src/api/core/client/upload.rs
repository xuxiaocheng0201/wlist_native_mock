use std::fs::File;

use crate::api::common::data::files::confirmations::FUploadConfirmation;
use crate::api::common::data::files::FFileLocation;
use crate::api::common::data::files::information::{FFileInformation, FUploadInformation};
use crate::api::common::data::files::options::FDuplicate;
use crate::api::common::data::files::tokens::FUploadToken;
use crate::api::common::exceptions::UniverseError;
use crate::api::core::client::{define_func, PauseController, WlistClientManager};

define_func!(
    /// Check whether the file/directory name is valid.
    ///
    /// Notice that this method only provides fast filtering, and some cases may still not be covered.
    ///
    /// parent: .isDirectory == true
    ///
    /// May returns [NameTooLongError], [InvalidFilenameError], [IllegalSuffixError] and [DuplicateFileError].
    /// The [DuplicateFileError] is the last error to check.
    ///
    /// >[NameTooLongError]: crate::api::common::exceptions::UniverseError::NameTooLongError
    /// >[InvalidFilenameError]: crate::api::common::exceptions::UniverseError::InvalidFilenameError
    /// >[IllegalSuffixError]: crate::api::common::exceptions::UniverseError::IllegalSuffixError
    /// >[DuplicateFileError]: crate::api::common::exceptions::UniverseError::DuplicateFileError
    upload_check_name(name: String, parent: FFileLocation, is_directory: bool) -> () = wlist_native::core::client::upload::upload_check_name
);
define_func!(
    /// Create a new empty directory.
    ///
    ///
    /// parent: .isDirectory == true
    ///
    /// name: 0 < len < 32768
    upload_mkdir(parent: FFileLocation, name: String, duplicate: FDuplicate) -> FFileInformation = wlist_native::core::client::upload::upload_mkdir
);

define_func!(
    /// Request to upload a new file.
    ///
    ///
    /// parent: .isDirectory == true
    ///
    /// name: 0 < len < 32768
    ///
    /// md5: the hash md5 of the entire new file. (This should be a lowercase string with a length of 32.)
    ///
    /// md5s: the md5 slice of each 4MB part of the new file.
    upload_request(parent: FFileLocation, name: String, size: u64, md5: String, md5s: Vec<String>, duplicate: FDuplicate) -> FUploadConfirmation = wlist_native::core::client::upload::upload_request
);
define_func!(
    /// Cancel an upload.
    ///
    /// What ever the upload is paused or not, or not confirmed, it will be canceled.
    upload_cancel(token: FUploadToken) -> () = wlist_native::core::client::upload::upload_cancel
);
define_func!(
    /// Confirm an upload.
    ///
    /// Then the upload is automatically resumed.
    upload_confirm(token: FUploadToken) -> FUploadInformation = wlist_native::core::client::upload::upload_confirm
);
define_func!(
    /// Finish an upload.
    ///
    /// May return [UploadChunkIncompleteError](crate::api::common::exceptions::UniverseError::UploadChunkIncompleteError).
    upload_finish(token: FUploadToken) -> FFileInformation = wlist_native::core::client::upload::upload_finish
);

/// flutter_rust_bridge:ignore
mod internal {
    use bytes::Bytes;
    use tokio::sync::watch::Receiver;
    use super::*;

    define_func!(upload_stream(token: FUploadToken, id: u64, buffer: &mut Bytes, control: Receiver<bool>) -> () = wlist_native::core::client::upload::upload_stream);
}

/// Upload the file chunk.
///
///
/// id: see the `chunks` field in [FUploadInformation]. (0 <= id < chunks_length)
///
/// buffer: a pointer to the buffer to read the data.
pub async fn upload_stream(client: Option<WlistClientManager>, token: FUploadToken, id: u64, buffer: ConstU8, buffer_size: usize, control: PauseController) -> Result<(), UniverseError> {
    let mut buffer = unsafe { wlist_native::core::helper::buffer::new_read_buffer(buffer.0, buffer_size) };
    internal::upload_stream(client, token, id, &mut buffer, control.sender.subscribe()).await
}


#[flutter_rust_bridge::frb(opaque)]
/// Native pointer of `*const u8`.
#[derive(Copy, Clone)]
pub struct ConstU8(*const u8);

unsafe impl Send for ConstU8 { }
unsafe impl Sync for ConstU8 { }



#[flutter_rust_bridge::frb(opaque)]
/// The internal resource of the pointed buffer.
pub struct PointedBufferResource {
    buffer: Vec<u8>,
}

/// Point a buffer from memory with copy.
///
/// returns a pointer to the buffer, the length of the buffer, and the internal resource.
pub fn point_buffer(buffer: Vec<u8>) -> (ConstU8, usize, PointedBufferResource) {
    (ConstU8(buffer.as_ptr()), buffer.len(), PointedBufferResource { buffer })
}

/// Drop the pointed buffer.
pub fn drop_buffer(resource: PointedBufferResource) {
    drop(resource.buffer);
}

#[flutter_rust_bridge::frb(opaque)]
/// The internal resource of the read-only mapped buffer.
pub struct MappedReadonlyBufferResource {
    file: File,
    mmap: memmap2::MmapRaw,
}

/// Map the read-only buffer in file.
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
/// returns: a pointer to the read-only buffer and the internal resource.
pub fn map_buffer(file: String, offset: u64, len: usize) -> Result<(ConstU8, MappedReadonlyBufferResource), UniverseError> {
    let file = File::options().read(true).open(&file).map_err(anyhow::Error::from)?;
    let mmap = memmap2::MmapOptions::new()
        .offset(offset).len(len)
        .map_raw_read_only(&file).map_err(anyhow::Error::from)?;
    Ok((ConstU8(mmap.as_ptr()), MappedReadonlyBufferResource { file, mmap }))
}

/// Drop the mapped read-only buffer from file.
pub fn drop_buffer_mapped(resource: MappedReadonlyBufferResource) -> Result<(), UniverseError> {
    drop(resource.mmap);
    drop(resource.file);
    Ok(())
}


/// Read the buffer.
///
/// Same as [clone_buffer](crate::api::core::client::download::clone_buffer),
/// but provide a `*const u8` version.
///
///
/// ptr: the pointer to the buffer.
///
/// len: the length of the buffer.
pub fn clone_buffer(ptr: ConstU8, len: usize) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(ptr.0, len) }.to_vec()
}
