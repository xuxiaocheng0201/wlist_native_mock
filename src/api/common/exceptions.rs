use std::collections::HashMap;

use crate::api::common::data::files::FFileLocation;
use crate::api::common::o2o;

macro_rules! define_exception {
    ($exception: ident $origin: ident()) => {
        #[flutter_rust_bridge::frb(non_opaque)]
        #[doc = concat!("See [UniverseError](crate::api::common::exceptions::UniverseError::", stringify!($origin), ") for details.")]
        #[derive(o2o::o2o)]
        #[map_owned(::wlist_native::common::exceptions::$origin)]
        pub struct $exception {}
    };
    ($exception: ident $origin: ident($($(#[$o2o:meta])* $field: ident: $ty: ty),+)) => {
        #[flutter_rust_bridge::frb(non_opaque)]
        #[doc = concat!("See [UniverseError](crate::api::common::exceptions::UniverseError::", stringify!($origin), ") for details.")]
        #[derive(o2o::o2o)]
        #[map_owned(::wlist_native::common::exceptions::$origin)]
        pub struct $exception {
            $($(#[$o2o])* pub $field: $ty),+
        }
    };
}

define_exception!(FUnavailableApiVersionError UnavailableApiVersionError());
define_exception!(FMatchFrequencyControlError MatchFrequencyControlError());
define_exception!(FIncorrectArgumentError IncorrectArgumentError(#[map(o2o::map(~))] e: String));
define_exception!(FPasswordMismatchedError PasswordMismatchedError());
define_exception!(FTokenExpiredError TokenExpiredError());

define_exception!(FInvalidStorageConfigError InvalidStorageConfigError(#[doc = "The key is the name of the invalid field"] #[from(o2o::from_hash_brown(~))] #[into(o2o::into_hash_brown(~))] e: HashMap<String, String>));
define_exception!(FIncorrectStorageAccountError IncorrectStorageAccountError());
define_exception!(FDuplicateStorageError DuplicateStorageError());
define_exception!(FStorageNotFoundError StorageNotFoundError());
define_exception!(FStorageTypeMismatchedError StorageTypeMismatchedError());
define_exception!(FStorageInLockError StorageInLockError(#[map(o2o::map(~))] e: String));

define_exception!(FComplexOperationError ComplexOperationError());
define_exception!(FDuplicateFileError DuplicateFileError(#[map(o2o::map(~))] l: FFileLocation, #[from(o2o::from_arc(~))] #[into(o2o::into_arc(~))] name: String));
define_exception!(FFileNotFoundError FileNotFoundError(#[map(o2o::map(~))] l: FFileLocation));
define_exception!(FFileInLockError FileInLockError(#[map(o2o::map(~))] l: FFileLocation, #[map(o2o::map(~))] e: String));
define_exception!(FUploadChunkIncompleteError UploadChunkIncompleteError());

define_exception!(FIllegalSuffixError IllegalSuffixError(#[map(o2o::map(~))] suffix: String));
define_exception!(FInvalidFilenameError InvalidFilenameError(#[from(o2o::from_arc(~))] #[into(o2o::into_arc(~))] name: String, ch: Option<char>));
define_exception!(FNameTooLongError NameTooLongError(#[from(o2o::from_arc(~))] #[into(o2o::into_arc(~))] name: String, max: Option<u64>));
define_exception!(FReadOnlyStorageError ReadOnlyStorageError());
define_exception!(FSpaceNotEnoughError SpaceNotEnoughError(left: Option<u64>, need: Option<u64>));
define_exception!(FFlowNotEnoughError FlowNotEnoughError(#[doc = "true is upload, false is download"] upload: bool, left: Option<u64>, need: Option<u64>));
define_exception!(FFileTooLargeError FileTooLargeError(size: u64, max: Option<u64>));


/// The universe error.
///
/// Ensured all api can throw and only throw this error.
/// This is the universe error of all errors.
#[flutter_rust_bridge::frb(non_opaque)]
pub enum UniverseError {
    // Api part

    /// Current version is unavailable.
    /// This means the server is no longer supported current version of software/app.
    UnavailableApiVersionError(FUnavailableApiVersionError),
    /// Matched the server (whatever core or web) frequency control.
    MatchFrequencyControlError(FMatchFrequencyControlError),
    /// Passed an incorrect argument.
    /// See docs of the function you used for more information.
    IncorrectArgumentError(FIncorrectArgumentError),
    /// Wrong userid or password in [core](crate::api::core::client::users::users_login) or [web](crate::api::web::account::login).
    PasswordMismatchedError(FPasswordMismatchedError),
    /// For web (not logged in) and core ([refresh token], [download token] and [upload token]).
    /// >[refresh token]: crate::api::common::data::files::tokens::FRefreshToken
    /// >[download token]: crate::api::common::data::files::tokens::FDownloadToken
    /// >[upload token]: crate::api::common::data::files::tokens::FUploadToken
    TokenExpiredError(FTokenExpiredError),

    // Storage part

    /// The storage config is invalid.
    InvalidStorageConfigError(FInvalidStorageConfigError),
    /// The storage account is incorrect. (e.g. password is wrong.)
    IncorrectStorageAccountError(FIncorrectStorageAccountError),
    /// The name of storage is conflict.
    DuplicateStorageError(FDuplicateStorageError),
    /// The storage does not exist.
    StorageNotFoundError(FStorageNotFoundError),
    /// When calling mismatched storage methods.
    /// See docs of the function you used for more information.
    StorageTypeMismatchedError(FStorageTypeMismatchedError),
    /// The storage is locked.
    StorageInLockError(FStorageInLockError),

    // Files part

    /// The operation is complex. (Complex means the operation cannot be done in one step.)
    /// This means you need to break down this operation into small pieces.
    /// UI should raise a dialog to ask user to confirm at first time.
    /// See docs of the function you used for more information.
    ComplexOperationError(FComplexOperationError),
    /// File duplicate and policy is [Error](crate::api::common::data::files::options::FDuplicate::Error)
    DuplicateFileError(FDuplicateFileError),
    /// The file/directory does not exist.
    FileNotFoundError(FFileNotFoundError),
    /// The file/directory is locked.
    FileInLockError(FFileInLockError),
    /// Only throw by [upload_finish](crate::api::core::client::upload_finish)
    /// when the upload is not finished.
    UploadChunkIncompleteError(FUploadChunkIncompleteError),

    // Limits part

    /// The suffix is not allowed by the storage.
    /// Or the suffix does not match when renamed in some storage.
    IllegalSuffixError(FIllegalSuffixError),
    /// The filename contains invalid character.
    InvalidFilenameError(FInvalidFilenameError),
    /// The file/directory name is too long.
    NameTooLongError(FNameTooLongError),
    /// Creating/uploading/modifying in a read-only storage.
    ReadOnlyStorageError(FReadOnlyStorageError),
    /// There is not enough space on the storage.
    SpaceNotEnoughError(FSpaceNotEnoughError),
    /// The transmission flow of the storage is not enough.
    FlowNotEnoughError(FFlowNotEnoughError),
    /// The maximum single file size limit for the storage is exceeded.
    FileTooLargeError(FFileTooLargeError),

    // Other part

    /// Any IO error.
    IO(Option<String>),
    /// Any network error.
    Network(String),
    /// Any other error.
    Other(String),
}

impl From<anyhow::Error> for UniverseError {
    fn from(value: anyhow::Error) -> Self {
        wlist_native::common::exceptions::UniverseError::new(value).into()
    }
}

impl From<wlist_native::common::exceptions::UniverseError> for UniverseError {
    fn from(value: wlist_native::common::exceptions::UniverseError) -> UniverseError {
        match value {
            wlist_native::common::exceptions::UniverseError::UnavailableApiVersionError(f0, ) => UniverseError::UnavailableApiVersionError(f0.into()),
            wlist_native::common::exceptions::UniverseError::MatchFrequencyControlError(f0, ) => UniverseError::MatchFrequencyControlError(f0.into()),
            wlist_native::common::exceptions::UniverseError::IncorrectArgumentError(f0, ) => UniverseError::IncorrectArgumentError(f0.into()),
            wlist_native::common::exceptions::UniverseError::PasswordMismatchedError(f0, ) => UniverseError::PasswordMismatchedError(f0.into()),
            wlist_native::common::exceptions::UniverseError::TokenExpiredError(f0, ) => UniverseError::TokenExpiredError(f0.into()),
            wlist_native::common::exceptions::UniverseError::InvalidStorageConfigError(f0, ) => UniverseError::InvalidStorageConfigError(f0.into()),
            wlist_native::common::exceptions::UniverseError::IncorrectStorageAccountError(f0, ) => UniverseError::IncorrectStorageAccountError(f0.into()),
            wlist_native::common::exceptions::UniverseError::DuplicateStorageError(f0, ) => UniverseError::DuplicateStorageError(f0.into()),
            wlist_native::common::exceptions::UniverseError::StorageNotFoundError(f0, ) => UniverseError::StorageNotFoundError(f0.into()),
            wlist_native::common::exceptions::UniverseError::StorageTypeMismatchedError(f0, ) => UniverseError::StorageTypeMismatchedError(f0.into()),
            wlist_native::common::exceptions::UniverseError::StorageInLockError(f0, ) => UniverseError::StorageInLockError(f0.into()),
            wlist_native::common::exceptions::UniverseError::ComplexOperationError(f0, ) => UniverseError::ComplexOperationError(f0.into()),
            wlist_native::common::exceptions::UniverseError::DuplicateFileError(f0, ) => UniverseError::DuplicateFileError(f0.into()),
            wlist_native::common::exceptions::UniverseError::FileNotFoundError(f0, ) => UniverseError::FileNotFoundError(f0.into()),
            wlist_native::common::exceptions::UniverseError::FileInLockError(f0, ) => UniverseError::FileInLockError(f0.into()),
            wlist_native::common::exceptions::UniverseError::UploadChunkIncompleteError(f0, ) => UniverseError::UploadChunkIncompleteError(f0.into()),
            wlist_native::common::exceptions::UniverseError::IllegalSuffixError(f0, ) => UniverseError::IllegalSuffixError(f0.into()),
            wlist_native::common::exceptions::UniverseError::InvalidFilenameError(f0, ) => UniverseError::InvalidFilenameError(f0.into()),
            wlist_native::common::exceptions::UniverseError::NameTooLongError(f0, ) => UniverseError::NameTooLongError(f0.into()),
            wlist_native::common::exceptions::UniverseError::ReadOnlyStorageError(f0, ) => UniverseError::ReadOnlyStorageError(f0.into()),
            wlist_native::common::exceptions::UniverseError::SpaceNotEnoughError(f0, ) => UniverseError::SpaceNotEnoughError(f0.into()),
            wlist_native::common::exceptions::UniverseError::FlowNotEnoughError(f0, ) => UniverseError::FlowNotEnoughError(f0.into()),
            wlist_native::common::exceptions::UniverseError::FileTooLargeError(f0, ) => UniverseError::FileTooLargeError(f0.into()),
            wlist_native::common::exceptions::UniverseError::IO(_f0, f1, ) => UniverseError::IO(f1),
            wlist_native::common::exceptions::UniverseError::Network(f0, ) => UniverseError::Network(f0),
            wlist_native::common::exceptions::UniverseError::Other(f0, ) => UniverseError::Other(f0),
            _ => unreachable!(),
        }
    }
}
