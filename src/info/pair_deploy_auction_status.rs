use core::time::Duration;

use smol_str::SmolStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the status of a pair deploy auction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct PairDeployAuctionStatus {
  #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde_seconds"))]
  start_time_seconds: Duration,
  #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde_seconds"))]
  duration_seconds: Duration,
  start_gas: SmolStr,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  current_gas: Option<SmolStr>,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  end_gas: Option<SmolStr>,
}

impl PairDeployAuctionStatus {
  /// Returns the start time of the auction in seconds.
  #[inline(always)]
  pub const fn start_time_seconds(&self) -> Duration {
    self.start_time_seconds
  }

  /// Returns the duration of the auction in seconds.
  #[inline(always)]
  pub const fn duration_seconds(&self) -> Duration {
    self.duration_seconds
  }

  /// Returns the starting gas value.
  #[inline(always)]
  pub const fn start_gas(&self) -> &SmolStr {
    &self.start_gas
  }

  /// Returns the current gas value.
  #[inline(always)]
  pub const fn current_gas(&self) -> Option<&SmolStr> {
    self.current_gas.as_ref()
  }

  /// Returns the ending gas value, if available.
  #[inline(always)]
  pub const fn end_gas(&self) -> Option<&SmolStr> {
    self.end_gas.as_ref()
  }
}

crate::http::serde_response_data!(@forward PairDeployAuctionStatus);
