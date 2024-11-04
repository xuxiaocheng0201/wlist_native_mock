use std::borrow::Cow;
use std::sync::Arc;

use anyhow::Error;
use hashbrown::HashMap;

use crate::common::data::files::FileLocation;

macro_rules! define_exception {
    ($exception: ident() $msg: literal) => {
        #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $exception;
        impl ::std::fmt::Display for $exception {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str($msg)
            }
        }
        impl ::std::error::Error for $exception { }
    };
    ($(#[$doc: meta])* $exception: ident($($field: ident: $ty: ty),+)) => {
        $(#[$doc])*
        #[derive(Debug, ::derive_more::Constructor, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $exception {
            $(pub $field: $ty),+
        }
        impl ::std::fmt::Display for $exception {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(stringify!($exception))
            }
        }
        impl ::std::error::Error for $exception { }
    };
}

define_exception!(UnavailableApiVersionError() "current api version is unavailable");
define_exception!(MatchFrequencyControlError() "match frequency control");
define_exception!(IncorrectArgumentError(e: Cow<'static, str>));
define_exception!(TooLargeDataError() "too large data");
define_exception!(PasswordMismatchedError() "password is mismatched");
define_exception!(TokenExpiredError() "token is expired");

define_exception!(InvalidStorageConfigError(e: HashMap<String, String>));
define_exception!(IncorrectStorageAccountError() "incorrect storage account");
define_exception!(DuplicateStorageError() "duplicate storage");
define_exception!(StorageNotFoundError() "storage not found");
define_exception!(StorageTypeMismatchedError() "storage type is mismatched");
define_exception!(StorageInLockError(e: Cow<'static, str>));

define_exception!(ComplexOperationError() "operation is too complex");
define_exception!(DuplicateFileError(l: FileLocation, name: Arc<String>));
define_exception!(FileNotFoundError(l: FileLocation));
define_exception!(FileInLockError(l: FileLocation, e: Cow<'static, str>));
define_exception!(UploadChunkIncompleteError() "upload chunks are incomplete");

define_exception!(IllegalSuffixError(suffix: Cow<'static, str>));
define_exception!(InvalidFilenameError(name: Arc<String>, ch: Option<char>));
define_exception!(NameTooLongError(name: Arc<String>, max: Option<u64>));
define_exception!(ReadOnlyStorageError() "readonly storage");
define_exception!(SpaceNotEnoughError(left: Option<u64>, need: Option<u64>));
define_exception!(FlowNotEnoughError(upload: bool, left: Option<u64>, need: Option<u64>));
define_exception!(FileTooLargeError(size: u64, max: Option<u64>));
define_exception!(NestTooDeepError(max: Option<u64>));

#[non_exhaustive]
pub enum UniverseError {
    IO(std::io::ErrorKind, Option<String>),
    Network(String),

    UnavailableApiVersionError(UnavailableApiVersionError),
    MatchFrequencyControlError(MatchFrequencyControlError),
    IncorrectArgumentError(IncorrectArgumentError),
    PasswordMismatchedError(PasswordMismatchedError),
    TokenExpiredError(TokenExpiredError),

    InvalidStorageConfigError(InvalidStorageConfigError),
    IncorrectStorageAccountError(IncorrectStorageAccountError),
    DuplicateStorageError(DuplicateStorageError),
    StorageNotFoundError(StorageNotFoundError),
    StorageTypeMismatchedError(StorageTypeMismatchedError),
    StorageInLockError(StorageInLockError),

    ComplexOperationError(ComplexOperationError),
    DuplicateFileError(DuplicateFileError),
    FileNotFoundError(FileNotFoundError),
    FileInLockError(FileInLockError),
    UploadChunkIncompleteError(UploadChunkIncompleteError),

    IllegalSuffixError(IllegalSuffixError),
    InvalidFilenameError(InvalidFilenameError),
    NameTooLongError(NameTooLongError),
    ReadOnlyStorageError(ReadOnlyStorageError),
    SpaceNotEnoughError(SpaceNotEnoughError),
    FlowNotEnoughError(FlowNotEnoughError),
    FileTooLargeError(FileTooLargeError),
    NestTooDeepError(NestTooDeepError),

    Other(String),
}

impl UniverseError {
    pub fn new(error: Error) -> Self {
        unimplemented!()
    }
}
