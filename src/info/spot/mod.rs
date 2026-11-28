use smol_str::SmolStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use meta::*;
pub use state::*;

/// The metadata data structures for spots of the Hyperliquid Core API.
mod meta;

mod state;

pub use super::pair_deploy_auction_status::PairDeployAuctionStatus;

/// The metadata for a token in the Hyperliquid Core API.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]

pub struct TokenMeta {
  name: SmolStr,
  sz_decimals: u8,
  wei_decimals: u8,
}

impl TokenMeta {
  /// Returns the name of the token.
  #[inline(always)]
  pub const fn name(&self) -> &SmolStr {
    &self.name
  }

  /// Returns the number of significant decimals for the token.
  #[inline(always)]
  pub const fn sz_decimals(&self) -> u8 {
    self.sz_decimals
  }

  /// Returns the number of wei decimals for the token.
  #[inline(always)]
  pub const fn wei_decimals(&self) -> u8 {
    self.wei_decimals
  }
}

/// The official default endpoint for the Hyperliquid Core info API.
pub const DEFAULT_ENDPOINT: &str = "https://api.hyperliquid.xyz/info";
