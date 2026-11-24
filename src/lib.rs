#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

/// The version of this crate, as `Cargo.toml` declares it.
///
/// This is the core tier: it needs neither `std` nor `alloc`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The data structures for the hyperliquid core API info.
pub mod info;

#[cfg(test)]
mod tests;
