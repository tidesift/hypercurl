use crate::info::spot::{PairDeployAuctionStatus, TokenMeta};
#[cfg(feature = "serde")]
use crate::utils::{serde_arc_array, serde_optional_arc_array};
use alloy_primitives::Address;
use derive_more::{IsVariant, TryUnwrap, Unwrap};
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
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  spots: Arc<[usize]>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  max_supply: Option<SmolStr>,
  hyperliquidity_genesis_balance: SmolStr,
  total_genesis_balance_wei: SmolStr,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  user_genesis_balances: Arc<[UserGenesisBalance]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  existing_token_genesis_balances: Arc<[TokenGenesisBalance]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
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
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
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

/// The balance information for a specific token in the Hyperliquid Core API.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct AssetBalance {
  coin: SmolStr,
  token: usize,
  total: SmolStr,
  hold: SmolStr,
  entry_ntl: SmolStr,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  spot_hold: Option<SmolStr>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  ltv: Option<SmolStr>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  borrowed: Option<SmolStr>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  supplied: Option<SmolStr>,
}

impl AssetBalance {
  /// Returns the coin associated with the token balance.
  #[inline(always)]
  pub const fn coin(&self) -> &SmolStr {
    &self.coin
  }

  /// Returns the token index associated with the token balance.
  #[inline(always)]
  pub const fn token(&self) -> usize {
    self.token
  }

  /// Returns the total balance for the token.
  #[inline(always)]
  pub const fn total(&self) -> &SmolStr {
    &self.total
  }

  /// Returns the hold balance for the token.
  #[inline(always)]
  pub const fn hold(&self) -> &SmolStr {
    &self.hold
  }

  /// Returns the entry NTL for the token.
  #[inline(always)]
  pub const fn entry_ntl(&self) -> &SmolStr {
    &self.entry_ntl
  }

  /// Returns the spot hold for the token, if available.
  #[inline(always)]
  pub const fn spot_hold(&self) -> Option<&SmolStr> {
    self.spot_hold.as_ref()
  }

  /// Returns the LTV for the token, if available.
  #[inline(always)]
  pub const fn ltv(&self) -> Option<&SmolStr> {
    self.ltv.as_ref()
  }

  /// Returns the borrowed amount for the token, if available.
  #[inline(always)]
  pub const fn borrowed(&self) -> Option<&SmolStr> {
    self.borrowed.as_ref()
  }

  /// Returns the supplied amount for the token, if available.
  #[inline(always)]
  pub const fn supplied(&self) -> Option<&SmolStr> {
    self.supplied.as_ref()
  }
}

/// The balance information for a specific token in the Hyperliquid Core API.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct MarketBalance {
  coin: SmolStr,
  total: SmolStr,
  hold: SmolStr,
  entry_ntl: SmolStr,
}

impl MarketBalance {
  /// Returns the outcome market identifier:
  /// - `+N`: `N` is `assetId - 100000000` of an active outcome market.
  /// - `oN`: `N` is the identifier of a settled outcome.
  #[inline(always)]
  pub const fn coin(&self) -> &SmolStr {
    &self.coin
  }

  /// Returns the total balance for the market token.
  #[inline(always)]
  pub const fn total(&self) -> &SmolStr {
    &self.total
  }

  /// Returns the hold balance for the market token.
  #[inline(always)]
  pub const fn hold(&self) -> &SmolStr {
    &self.hold
  }

  /// Returns the entry NTL for the market token.
  #[inline(always)]
  pub const fn entry_ntl(&self) -> &SmolStr {
    &self.entry_ntl
  }
}

/// The balance information for a specific token, which can be either a spot token or a market token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, IsVariant, Unwrap, TryUnwrap)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]

pub enum TokenBalance {
  /// Asset token balance.
  Asset(AssetBalance),
  /// Market token balance.
  Market(MarketBalance),
}

/// Escrowed balance
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct EvmEscrow {
  coin: SmolStr,
  token: usize,
  total: SmolStr,
}

impl EvmEscrow {
  /// Returns the coin for the escrowed balance.
  #[inline(always)]
  pub const fn coin(&self) -> &SmolStr {
    &self.coin
  }

  /// Returns the token identifier for the escrowed balance.
  #[inline(always)]
  pub const fn token(&self) -> usize {
    self.token
  }

  /// Returns the total balance for the escrowed token.
  #[inline(always)]
  pub const fn total(&self) -> &SmolStr {
    &self.total
  }
}

/// Token ratio information
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenRatio {
  token: usize,
  ratio: SmolStr,
}

impl TokenRatio {
  /// Returns the token identifier for the ratio.
  #[inline(always)]
  pub const fn token(&self) -> usize {
    self.token
  }

  /// Returns the ratio for the token.
  #[inline(always)]
  pub const fn ratio(&self) -> &SmolStr {
    &self.ratio
  }
}

/// Token availability information after maintenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenToAvailableAfterMaintenance {
  token: usize,
  amount: SmolStr,
}

impl TokenToAvailableAfterMaintenance {
  /// Returns the token identifier for the availability information.
  #[inline(always)]
  pub const fn token(&self) -> usize {
    self.token
  }

  /// Returns the amount available after maintenance for the token.
  #[inline(always)]
  pub const fn amount(&self) -> &SmolStr {
    &self.amount
  }
}

/// Account summary for spot trading
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct ClearinghouseState {
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  portfolio_margin_enabled: Option<bool>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  balances: Arc<[TokenBalance]>,
  #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_arc_array"))]
  evm_escrows: Option<Arc<[EvmEscrow]>>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  portfolio_margin_ratio: Option<SmolStr>,
  #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_arc_array"))]
  token_to_portfolio_borrow_ratio: Option<Arc<[TokenRatio]>>,
  #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_arc_array"))]
  token_to_portfolio_supply_ratio: Option<Arc<[TokenRatio]>>,
  #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_arc_array"))]
  token_to_available_after_maintenance: Option<Arc<[TokenToAvailableAfterMaintenance]>>,
}

impl ClearinghouseState {
  /// Returns whether portfolio margin is enabled.
  #[inline(always)]
  pub const fn portfolio_margin_enabled(&self) -> Option<bool> {
    self.portfolio_margin_enabled
  }

  /// Returns the balances for the clearing house state.
  #[inline(always)]
  pub const fn balances(&self) -> &Arc<[TokenBalance]> {
    &self.balances
  }

  /// Returns a reference to the balances for the clearing house state.
  #[inline(always)]
  pub fn balances_ref(&self) -> &[TokenBalance] {
    &self.balances
  }

  /// Returns the EVM escrows for the clearing house state.
  #[inline(always)]
  pub const fn evm_escrows(&self) -> Option<&Arc<[EvmEscrow]>> {
    self.evm_escrows.as_ref()
  }

  /// Returns a reference to the EVM escrows for the clearing house state.
  #[inline(always)]
  pub fn evm_escrows_ref(&self) -> &[EvmEscrow] {
    self.evm_escrows.as_deref().unwrap_or_default()
  }

  /// Returns the portfolio margin ratio.
  #[inline(always)]
  pub const fn portfolio_margin_ratio(&self) -> Option<&SmolStr> {
    self.portfolio_margin_ratio.as_ref()
  }

  /// Returns the token to portfolio borrow ratio.
  #[inline(always)]
  pub const fn token_to_portfolio_borrow_ratio(&self) -> Option<&Arc<[TokenRatio]>> {
    self.token_to_portfolio_borrow_ratio.as_ref()
  }

  /// Returns a reference to the token to portfolio borrow ratio.
  #[inline(always)]
  pub fn token_to_portfolio_borrow_ratio_ref(&self) -> &[TokenRatio] {
    self
      .token_to_portfolio_borrow_ratio
      .as_deref()
      .unwrap_or_default()
  }

  /// Returns the token to portfolio supply ratio.
  #[inline(always)]
  pub const fn token_to_portfolio_supply_ratio(&self) -> Option<&Arc<[TokenRatio]>> {
    self.token_to_portfolio_supply_ratio.as_ref()
  }

  /// Returns a reference to the token to portfolio supply ratio.
  #[inline(always)]
  pub fn token_to_portfolio_supply_ratio_ref(&self) -> &[TokenRatio] {
    self
      .token_to_portfolio_supply_ratio
      .as_deref()
      .unwrap_or_default()
  }

  /// Returns the token to available after maintenance.
  #[inline(always)]
  pub const fn token_to_available_after_maintenance(
    &self,
  ) -> Option<&Arc<[TokenToAvailableAfterMaintenance]>> {
    self.token_to_available_after_maintenance.as_ref()
  }

  /// Returns a reference to the token to available after maintenance.
  #[inline(always)]
  pub fn token_to_available_after_maintenance_ref(&self) -> &[TokenToAvailableAfterMaintenance] {
    self
      .token_to_available_after_maintenance
      .as_deref()
      .unwrap_or_default()
  }
}

crate::http::serde_response_data!(@forward DeployState, ClearinghouseState);

#[cfg(feature = "serde")]
const _: () = {
  use serde::{Deserializer, Serializer};

  impl Serialize for TokenRatio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      (&self.token, self.ratio()).serialize(serializer)
    }
  }

  impl<'de> Deserialize<'de> for TokenRatio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let (token, ratio) = <(usize, SmolStr)>::deserialize(deserializer)?;
      Ok(TokenRatio { token, ratio })
    }
  }

  impl Serialize for TokenToAvailableAfterMaintenance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      (&self.token, self.amount()).serialize(serializer)
    }
  }

  impl<'de> Deserialize<'de> for TokenToAvailableAfterMaintenance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let (token, amount) = <(usize, SmolStr)>::deserialize(deserializer)?;
      Ok(TokenToAvailableAfterMaintenance { token, amount })
    }
  }
};
