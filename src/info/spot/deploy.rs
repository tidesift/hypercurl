use crate::info::spot::{PairDeployAuctionStatus, TokenMeta};
#[cfg(feature = "serde")]
use crate::utils::serde_triomphe_array;
use alloy_primitives::Address;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use smol_str::SmolStr;

use triomphe::Arc;

/// User genesis balances for the token.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct UserGenesisBalance {
  address: Address,
  balance: SmolStr,
}

impl UserGenesisBalance {
  /// Returns the address of the user.
  #[inline(always)]
  pub const fn address(&self) -> &Address {
    &self.address
  }

  /// Returns the balance of the user.
  #[inline(always)]
  pub const fn balance(&self) -> &SmolStr {
    &self.balance
  }
}

/// User genesis balances for the token.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct TokenGenesisBalance {
  token: usize,
  balance: SmolStr,
}

impl TokenGenesisBalance {
  /// Returns the token identifier.
  #[inline(always)]
  pub const fn token(&self) -> usize {
    self.token
  }

  /// Returns the balance of the token.
  #[inline(always)]
  pub const fn balance(&self) -> &SmolStr {
    &self.balance
  }
}

/// The state for a spot market.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct State {
  token: usize,
  spec: TokenMeta,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  full_name: Option<SmolStr>,
  deployer_trading_fee_share: SmolStr,
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  spots: Arc<[usize]>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  max_supply: Option<SmolStr>,
  hyperliquidity_genesis_balance: SmolStr,
  total_genesis_balance_wei: SmolStr,
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  user_genesis_balances: Arc<[UserGenesisBalance]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  existing_token_genesis_balances: Arc<[TokenGenesisBalance]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  blacklist_users: Arc<[Address]>,
}

impl State {
  /// Returns the token identifier.
  #[inline(always)]
  pub const fn token(&self) -> usize {
    self.token
  }

  /// Returns the token metadata specification.
  #[inline(always)]
  pub const fn spec(&self) -> &TokenMeta {
    &self.spec
  }

  /// Returns the full name of the token, if available.
  #[inline(always)]
  pub const fn full_name(&self) -> Option<&SmolStr> {
    self.full_name.as_ref()
  }

  /// Returns the deployer trading fee share.
  #[inline(always)]
  pub const fn deployer_trading_fee_share(&self) -> &SmolStr {
    &self.deployer_trading_fee_share
  }

  /// Returns the spots associated with the token.
  #[inline(always)]
  pub const fn spots(&self) -> &Arc<[usize]> {
    &self.spots
  }

  /// Returns a reference to the spots as a slice.
  #[inline(always)]
  pub fn spots_ref(&self) -> &[usize] {
    &self.spots
  }

  /// Returns the maximum supply of the token, if available.
  #[inline(always)]
  pub const fn max_supply(&self) -> Option<&SmolStr> {
    self.max_supply.as_ref()
  }

  /// Returns the hyperliquidity genesis balance.
  #[inline(always)]
  pub const fn hyperliquidity_genesis_balance(&self) -> &SmolStr {
    &self.hyperliquidity_genesis_balance
  }

  /// Returns the total genesis balance in wei.
  #[inline(always)]
  pub const fn total_genesis_balance_wei(&self) -> &SmolStr {
    &self.total_genesis_balance_wei
  }

  /// Returns the user genesis balances.
  #[inline(always)]
  pub const fn user_genesis_balances(&self) -> &Arc<[UserGenesisBalance]> {
    &self.user_genesis_balances
  }

  /// Returns a reference to the user genesis balances as a slice.
  #[inline(always)]
  pub fn user_genesis_balances_ref(&self) -> &[UserGenesisBalance] {
    &self.user_genesis_balances
  }

  /// Returns the existing token genesis balances.
  #[inline(always)]
  pub const fn existing_token_genesis_balances(&self) -> &Arc<[TokenGenesisBalance]> {
    &self.existing_token_genesis_balances
  }

  /// Returns a reference to the existing token genesis balances as a slice.
  #[inline(always)]
  pub fn existing_token_genesis_balances_ref(&self) -> &[TokenGenesisBalance] {
    &self.existing_token_genesis_balances
  }

  /// Returns the blacklist users.
  #[inline(always)]
  pub const fn blacklist_users(&self) -> &Arc<[Address]> {
    &self.blacklist_users
  }

  /// Returns a reference to the blacklist users as a slice.
  #[inline(always)]
  pub fn blacklist_users_ref(&self) -> &[Address] {
    &self.blacklist_users
  }
}

/// The deployment state for a spot market.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct DeployState {
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  states: Arc<[State]>,
  gas_auction: PairDeployAuctionStatus,
}

impl DeployState {
  /// Returns the states associated with the deploy.
  #[inline(always)]
  pub const fn states(&self) -> &Arc<[State]> {
    &self.states
  }

  /// Returns a reference to the states as a slice.
  #[inline(always)]
  pub fn states_ref(&self) -> &[State] {
    &self.states
  }

  /// Returns the gas auction status.
  #[inline(always)]
  pub const fn gas_auction(&self) -> &PairDeployAuctionStatus {
    &self.gas_auction
  }
}

crate::http::serde_response_data!(@forward DeployState);
