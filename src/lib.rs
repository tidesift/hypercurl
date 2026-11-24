#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

/// The version of this crate, as `Cargo.toml` declares it.
///
/// This is the core tier: it needs neither `std` nor `alloc`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The data structures for the hyperliquid core API info.
pub mod info;

/// The data structures for the hyperliquid core API exchange.
pub mod exchange;

/// The http client and related utilities for the hyperliquid core API.
#[cfg(any(feature = "reqwest", feature = "ureq"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "reqwest", feature = "ureq"))))]
pub mod http;

mod req;

#[cfg(test)]
mod tests;
