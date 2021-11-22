//! [Mumble] protocol implementation in Rust.
//!
//! [Mumble]: https://mumble.info/

#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod control;
#[cfg(feature = "openssl")]
pub mod crypt;
pub mod ping;
pub mod varint;
pub mod voice;

pub use voice::Clientbound;
pub use voice::Serverbound;

#[cfg(not(any(feature = "asynchronous-codec", feature = "tokio-codec")))]
compile_error!("need at least one of asynchronous-codec or tokio-codec features to compile");