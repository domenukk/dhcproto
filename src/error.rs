//! Error types for Encoding/Decoding
#[cfg(feature = "std")]
use std::io;

use alloc::boxed::Box;
use thiserror::Error;

/// Convenience type for decode errors
pub type DecodeResult<T> = Result<T, DecodeError>;

/// Returned from types that decode
#[derive(Error, Debug)]
pub enum DecodeError {
    /// add overflow
    #[error("decoder checked_add failed")]
    AddOverflow,

    /// ran out of bytes
    #[error("parser ran out of data-- not enough bytes")]
    NotEnoughBytes,

    /// error converting from slice
    #[error("error converting from slice {0}")]
    SliceError(#[from] core::array::TryFromSliceError),

    /// error finding nul in string
    #[error("error getting null terminated string {0}")]
    NulError(#[from] core::ffi::FromBytesWithNulError),

    /// error converting to utf-8
    #[error("error converting to UTF-8 {0}")]
    Utf8Error(#[from] core::str::Utf8Error),

    /// io error
    #[cfg(feature = "std")]
    #[error("io error {0}")]
    IoError(#[from] io::Error),

    /// url parse error
    #[error("url parse error")]
    UrlParseError(#[from] url::ParseError),

    /// domain parse error
    #[error("domain parse error {0}")]
    DomainParseError(#[from] trust_dns_proto::error::ProtoError),

    /// Unknown decode error
    #[error("unknown error")]
    #[cfg(feature = "std")]
    Unknown(Box<dyn std::error::Error + Send + Sync + 'static>),

    /// Unknown decode error
    #[error("unknown error")]
    #[cfg(not(feature = "std"))]
    Unknown(Box<dyn core::error::Error + Send + Sync + 'static>),
}

/// Returned from types that encode
#[derive(Error, Debug)]
pub enum EncodeError {
    /// addition overflow
    #[error("encoder checked_add failed")]
    AddOverflow,

    /// string exceeds bounds
    #[error(
        "message is trying to write a string to the message that exceeds the max size of {len}"
    )]
    StringSizeTooBig {
        /// size of string
        len: usize,
    },

    /// io error
    #[error("io error {0}")]
    #[cfg(feature = "std")]
    IoError(#[from] io::Error),

    /// DNS encoding error from trust-dns
    #[error("domain encoding error {0}")]
    DomainEncodeError(#[from] trust_dns_proto::error::ProtoError),
}

/// Convenience type for encode errors
pub type EncodeResult<T> = Result<T, EncodeError>;
