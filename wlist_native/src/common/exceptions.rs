use std::borrow::Cow;
use std::sync::Arc;

use anyhow::{anyhow, Error};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

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
    ($(#[$doc: meta])* $exception: ident($($field: ident: $ty: ty),+) |&$self: ident, $f: ident| $write: expr) => {
        $(#[$doc])*
        #[derive(Debug, ::derive_more::Constructor, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $exception {
            $(pub $field: $ty),+
        }
        impl ::std::fmt::Display for $exception {
            fn fmt(&self, $f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let $self = self;
                $write
            }
        }
        impl ::std::error::Error for $exception { }
    };
}

define_exception!(#[doc = "Mocked error for reqwest."] NetworkError(e: Cow<'static, str>) |&e, f| write!(f, "network error: {}", e.e));

define_exception!(UnavailableApiVersionError() "current api version is unavailable");
define_exception!(MatchFrequencyControlError() "match frequency control");
define_exception!(IncorrectArgumentError(e: Cow<'static, str>) |&e, f| write!(f, "incorrect argument: {}", e.e));
define_exception!(TooLargeDataError() "too large data");
define_exception!(PasswordMismatchedError() "password is mismatched");
define_exception!(TokenExpiredError() "token is expired");

define_exception!(InvalidStorageConfigError(e: HashMap<String, String>) |&e, f| write!(f, "invalid storage config: {:?}", e.e));
define_exception!(IncorrectStorageAccountError() "incorrect storage account");
define_exception!(DuplicateStorageError() "duplicate storage");
define_exception!(StorageNotFoundError() "storage not found");
define_exception!(StorageTypeMismatchedError() "storage type is mismatched");
define_exception!(StorageInLockError(e: Cow<'static, str>) |&e, f| write!(f, "storage in lock: {}", e.e));

define_exception!(ComplexOperationError() "operation is too complex");
define_exception!(DuplicateFileError(l: FileLocation, name: Arc<String>) |&e, f| write!(f, "duplicate file {} in {:?}", e.name, e.l));
define_exception!(FileNotFoundError(l: FileLocation) |&e, f| write!(f, "file not found {:?}", e.l));
define_exception!(FileInLockError(l: FileLocation, e: Cow<'static, str>) |&e, f| write!(f, "{} (locking {:?})", e.e, e.l));
define_exception!(UploadChunkIncompleteError() "upload chunks are incomplete");

define_exception!(IllegalSuffixError(suffix: Cow<'static, str>) |&e, f| write!(f, "illegal suffix: {}", e.suffix));
define_exception!(InvalidFilenameError(name: Arc<String>, ch: Option<char>) |&e, f| write!(f, "invalid filename: {} char: {:?}", e.name, e.ch));
define_exception!(NameTooLongError(name: Arc<String>, max: Option<u64>) |&e, f| write!(f, "name too long: {} limit: {:?}", e.name, e.max));
define_exception!(ReadOnlyStorageError() "readonly storage");
define_exception!(SpaceNotEnoughError(left: Option<u64>, need: Option<u64>) |&e, f| write!(f, "space not enough: {:?} < {:?}", e.left, e.need));
define_exception!(FlowNotEnoughError(upload: bool, left: Option<u64>, need: Option<u64>) |&e, f| write!(f, "{} flow not enough: {:?} < {:?}", if e.upload { "upload" } else { "download" }, e.left, e.need));
define_exception!(FileTooLargeError(size: u64, max: Option<u64>) |&e, f| write!(f, "file too large: {} limit: {:?}", e.size, e.max));
define_exception!(NestTooDeepError(max: Option<u64>) |&e, f| write!(f, "nest too deep. limit: {:?}", e.max));


#[derive(Serialize, Deserialize)]
#[serde(remote = "std::io::ErrorKind")]
#[non_exhaustive]
enum ErrorKindDeref {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
}

fn check_error_kind(kind: std::io::ErrorKind) -> std::io::ErrorKind {
    struct NoopWrite;
    impl std::io::Write for NoopWrite {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    if ErrorKindDeref::serialize(&kind, &mut rmp_serde::Serializer::new(NoopWrite)).is_ok() {
        kind
    } else {
        std::io::ErrorKind::Other
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UniverseError {
    IO(#[serde(with = "ErrorKindDeref")] std::io::ErrorKind, Option<String>),
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
        macro_rules! downcast {
            ($error: ident |$e: ident: $t: ty| $universe: expr) => {
                let $error = match $error.downcast::<$t>() {
                    Err(e) => e, Ok($e) => return $universe,
                };
            };
            ($error: ident |$e: ident: ref $t: ty| $universe: expr) => {
                if let Some($e) = $error.downcast_ref::<$t>() {
                    return $universe;
                };
            };
        }
        downcast!(error |e: std::io::Error| Self::IO(check_error_kind(e.kind()), e.into_inner().map(|e| e.to_string())));
        downcast!(error |e: NetworkError| Self::Network(e.to_string()));

        downcast!(error |e: UnavailableApiVersionError| Self::UnavailableApiVersionError(e));
        downcast!(error |e: MatchFrequencyControlError| Self::MatchFrequencyControlError(e));
        downcast!(error |e: IncorrectArgumentError| Self::IncorrectArgumentError(e));
        downcast!(error |e: PasswordMismatchedError| Self::PasswordMismatchedError(e));
        downcast!(error |e: TokenExpiredError| Self::TokenExpiredError(e));

        downcast!(error |e: InvalidStorageConfigError| Self::InvalidStorageConfigError(e));
        downcast!(error |e: IncorrectStorageAccountError| Self::IncorrectStorageAccountError(e));
        downcast!(error |e: DuplicateStorageError| Self::DuplicateStorageError(e));
        downcast!(error |e: StorageNotFoundError| Self::StorageNotFoundError(e));
        downcast!(error |e: StorageTypeMismatchedError| Self::StorageTypeMismatchedError(e));
        downcast!(error |e: StorageInLockError| Self::StorageInLockError(e));

        downcast!(error |e: ComplexOperationError| Self::ComplexOperationError(e));
        downcast!(error |e: DuplicateFileError| Self::DuplicateFileError(e));
        downcast!(error |e: FileNotFoundError| Self::FileNotFoundError(e));
        downcast!(error |e: FileInLockError| Self::FileInLockError(e));
        downcast!(error |e: UploadChunkIncompleteError| Self::UploadChunkIncompleteError(e));

        downcast!(error |e: IllegalSuffixError| Self::IllegalSuffixError(e));
        downcast!(error |e: InvalidFilenameError| Self::InvalidFilenameError(e));
        downcast!(error |e: NameTooLongError| Self::NameTooLongError(e));
        downcast!(error |e: ReadOnlyStorageError| Self::ReadOnlyStorageError(e));
        downcast!(error |e: SpaceNotEnoughError| Self::SpaceNotEnoughError(e));
        downcast!(error |e: FlowNotEnoughError| Self::FlowNotEnoughError(e));
        downcast!(error |e: FileTooLargeError| Self::FileTooLargeError(e));
        downcast!(error |e: NestTooDeepError| Self::NestTooDeepError(e));

        Self::Other(format!("{error:#}"))
    }

    pub fn to_anyhow(self) -> Error {
        match self {
            UniverseError::IO(kind, Some(error)) => std::io::Error::new(kind, error).into(),
            UniverseError::IO(kind, None) => std::io::Error::from(kind).into(),
            UniverseError::Network(error) => std::io::Error::new(std::io::ErrorKind::Other, error).into(),

            UniverseError::UnavailableApiVersionError(error) => error.into(),
            UniverseError::MatchFrequencyControlError(error) => error.into(),
            UniverseError::IncorrectArgumentError(error) => error.into(),
            UniverseError::PasswordMismatchedError(error) => error.into(),
            UniverseError::TokenExpiredError(error) => error.into(),

            UniverseError::InvalidStorageConfigError(error) => error.into(),
            UniverseError::IncorrectStorageAccountError(error) => error.into(),
            UniverseError::DuplicateStorageError(error) => error.into(),
            UniverseError::StorageNotFoundError(error) => error.into(),
            UniverseError::StorageTypeMismatchedError(error) => error.into(),
            UniverseError::StorageInLockError(error) => error.into(),

            UniverseError::ComplexOperationError(error) => error.into(),
            UniverseError::DuplicateFileError(error) => error.into(),
            UniverseError::FileNotFoundError(error) => error.into(),
            UniverseError::FileInLockError(error) => error.into(),
            UniverseError::UploadChunkIncompleteError(error) => error.into(),

            UniverseError::IllegalSuffixError(error) => error.into(),
            UniverseError::InvalidFilenameError(error) => error.into(),
            UniverseError::NameTooLongError(error) => error.into(),
            UniverseError::ReadOnlyStorageError(error) => error.into(),
            UniverseError::SpaceNotEnoughError(error) => error.into(),
            UniverseError::FlowNotEnoughError(error) => error.into(),
            UniverseError::FileTooLargeError(error) => error.into(),
            UniverseError::NestTooDeepError(error) => error.into(),

            UniverseError::Other(error) => anyhow!("{error}"),
        }
    }
}
