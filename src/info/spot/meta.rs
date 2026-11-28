use alloy_primitives::{Address, B128};
use smol_str::SmolStr;

#[cfg(feature = "serde")]
use crate::utils::serde_triomphe_array;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use triomphe::Arc;

use crate::{info::spot::TokenMeta, utils::TakeArc};

/// The metadata for a spot asset in the hyperliquid core API.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct AssetMeta {
  tokens: [usize; 2],
  name: SmolStr,
  index: usize,
  is_canonical: bool,
}

impl AssetMeta {
  /// Returns the tokens associated with the spot asset.
  #[inline(always)]
  pub const fn tokens(&self) -> &[usize; 2] {
    &self.tokens
  }

  /// Returns the name of the spot asset.
  #[inline(always)]
  pub const fn name(&self) -> &SmolStr {
    &self.name
  }

  /// Returns the index of the spot asset.
  #[inline(always)]
  pub const fn index(&self) -> usize {
    self.index
  }

  /// Returns whether the spot asset is canonical.
  #[inline(always)]
  pub const fn is_canonical(&self) -> bool {
    self.is_canonical
  }
}

/// Evm contract information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct EvmContract {
  address: Address,
  #[cfg_attr(feature = "serde", serde(rename = "evm_extra_wei_decimals"))]
  extra_wei_decimals: isize,
}

impl EvmContract {
  /// Returns the address of the EVM contract.
  #[inline(always)]
  pub const fn address(&self) -> Address {
    self.address
  }

  /// Returns the extra wei decimals of the EVM contract.
  #[inline(always)]
  pub const fn extra_wei_decimals(&self) -> isize {
    self.extra_wei_decimals
  }
}

/// The token information for a spot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct TokenInfo {
  #[cfg_attr(feature = "serde", serde(flatten))]
  meta: TokenMeta,
  index: usize,
  token_id: B128,
  is_canonical: bool,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  evm_contract: Option<EvmContract>,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  full_name: Option<SmolStr>,
  deployer_trading_fee_share: SmolStr,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  deployer_label: Option<SmolStr>,
}

impl TokenInfo {
  /// Returns the name of the token.
  #[inline(always)]
  pub const fn name(&self) -> &SmolStr {
    self.meta.name()
  }

  /// Returns the size decimals of the token.
  #[inline(always)]
  pub const fn sz_decimals(&self) -> u8 {
    self.meta.sz_decimals()
  }

  /// Returns the wei decimals of the token.
  #[inline(always)]
  pub const fn wei_decimals(&self) -> u8 {
    self.meta.wei_decimals()
  }

  /// Returns the index of the token.
  #[inline(always)]
  pub const fn index(&self) -> usize {
    self.index
  }

  /// Returns the token ID.
  #[inline(always)]
  pub const fn token_id(&self) -> B128 {
    self.token_id
  }

  /// Returns whether the token is canonical.
  #[inline(always)]
  pub const fn is_canonical(&self) -> bool {
    self.is_canonical
  }

  /// Returns the EVM contract associated with the token, if any.
  #[inline(always)]
  pub const fn evm_contract(&self) -> Option<&EvmContract> {
    self.evm_contract.as_ref()
  }

  /// Returns the full name of the token, if any.
  #[inline(always)]
  pub const fn full_name(&self) -> Option<&SmolStr> {
    self.full_name.as_ref()
  }

  /// Returns the deployer trading fee share associated with the token.
  #[inline(always)]
  pub const fn deployer_trading_fee_share(&self) -> &SmolStr {
    &self.deployer_trading_fee_share
  }

  /// Returns the deployer label associated with the token, if any.
  #[inline(always)]
  pub const fn deployer_label(&self) -> Option<&SmolStr> {
    self.deployer_label.as_ref()
  }
}

/// The metadata associated with a spot.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct Meta {
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  universe: Arc<[AssetMeta]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  tokens: Arc<[TokenInfo]>,
}

impl From<Meta> for Arc<[TokenInfo]> {
  #[inline(always)]
  fn from(spot_meta: Meta) -> Self {
    spot_meta.tokens
  }
}

impl From<Meta> for Arc<[AssetMeta]> {
  #[inline(always)]
  fn from(spot_meta: Meta) -> Self {
    spot_meta.universe
  }
}

impl From<Meta> for (Arc<[AssetMeta]>, Arc<[TokenInfo]>) {
  #[inline(always)]
  fn from(spot_meta: Meta) -> Self {
    (spot_meta.universe, spot_meta.tokens)
  }
}

impl Meta {
  /// Returns the universe of spot asset metadata.
  #[inline(always)]
  pub fn universe_ref(&self) -> &[AssetMeta] {
    self.universe.as_ref()
  }

  /// Returns the universe of spot asset metadata as an `Arc` slice.
  #[inline(always)]
  pub const fn universe(&self) -> &Arc<[AssetMeta]> {
    &self.universe
  }

  /// Returns the spot's base asset metadata.
  #[inline(always)]
  pub const fn tokens(&self) -> &Arc<[TokenInfo]> {
    &self.tokens
  }

  /// Returns the spot's base asset metadata as a slice.
  #[inline(always)]
  pub fn tokens_ref(&self) -> &[TokenInfo] {
    self.tokens.as_ref()
  }

  /// Takes the spot's base asset metadata, leaving an empty vector in its place.
  #[inline(always)]
  pub fn take_tokens(&mut self) -> Arc<[TokenInfo]> {
    self.tokens.take_arc()
  }

  /// Takes the universe of spot asset metadata, leaving an empty vector in its place.
  #[inline(always)]
  pub fn take_universe(&mut self) -> Arc<[AssetMeta]> {
    self.universe.take_arc()
  }

  /// Consumes the `Meta` instance and returns the spot's base asset metadata.
  #[inline(always)]
  pub fn into_tokens(self) -> Arc<[TokenInfo]> {
    self.into()
  }

  /// Consumes the `Meta` instance and returns the universe of spot asset metadata.
  #[inline(always)]
  pub fn into_universe(self) -> Arc<[AssetMeta]> {
    self.into()
  }

  /// Consumes the `Meta` instance and returns both the universe and the spot's base asset metadata.
  #[inline(always)]
  pub fn into_part(self) -> (Arc<[AssetMeta]>, Arc<[TokenInfo]>) {
    self.into()
  }
}

/// The asset context for a spot asset.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct AssetContext {
  prev_day_px: SmolStr,
  day_ntl_vlm: SmolStr,
  mark_px: SmolStr,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  mid_px: Option<SmolStr>,
  circulating_supply: SmolStr,
  coin: SmolStr,
  total_supply: SmolStr,
  day_base_vlm: SmolStr,
}

impl AssetContext {
  /// Returns the day notional volume.
  #[inline(always)]
  pub const fn day_ntl_vlm(&self) -> &SmolStr {
    &self.day_ntl_vlm
  }

  /// Returns the mark price.
  #[inline(always)]
  pub const fn mark_px(&self) -> &SmolStr {
    &self.mark_px
  }

  /// Returns the mid price, if available.
  #[inline(always)]
  pub const fn mid_px(&self) -> Option<&SmolStr> {
    self.mid_px.as_ref()
  }

  /// Returns the previous day's price.
  #[inline(always)]
  pub const fn prev_day_px(&self) -> &SmolStr {
    &self.prev_day_px
  }

  /// Returns the circulating supply.
  #[inline(always)]
  pub const fn circulating_supply(&self) -> &SmolStr {
    &self.circulating_supply
  }

  /// Returns the coin identifier.
  #[inline(always)]
  pub const fn coin(&self) -> &SmolStr {
    &self.coin
  }

  /// Returns the total supply.
  #[inline(always)]
  pub const fn total_supply(&self) -> &SmolStr {
    &self.total_supply
  }

  /// Returns the day base volume.
  #[inline(always)]
  pub const fn day_base_vlm(&self) -> &SmolStr {
    &self.day_base_vlm
  }
}

/// Represents the metadata and asset contexts for derivatives.
#[derive(Debug, Clone)]
pub struct MetaAndAssetContexts {
  /// Represents derivatives metadata.
  meta: Meta,
  /// Represents asset contexts.
  contexts: Arc<[AssetContext]>,
}

impl From<MetaAndAssetContexts> for Meta {
  #[inline(always)]
  fn from(value: MetaAndAssetContexts) -> Self {
    value.meta
  }
}

impl From<MetaAndAssetContexts> for Arc<[AssetContext]> {
  #[inline(always)]
  fn from(value: MetaAndAssetContexts) -> Self {
    value.contexts
  }
}

impl From<MetaAndAssetContexts> for (Meta, Arc<[AssetContext]>) {
  #[inline(always)]
  fn from(value: MetaAndAssetContexts) -> Self {
    (value.meta, value.contexts)
  }
}

impl MetaAndAssetContexts {
  /// Returns a reference to the meta field.
  #[inline(always)]
  pub const fn meta(&self) -> &Meta {
    &self.meta
  }

  /// Returns a mutable reference to the meta field.
  #[inline(always)]
  pub fn meta_mut(&mut self) -> &mut Meta {
    &mut self.meta
  }

  /// Returns a reference to the contexts field.
  #[inline(always)]
  pub fn contexts_ref(&self) -> &[AssetContext] {
    self.contexts.as_ref()
  }

  /// Returns a reference to the contexts field.
  #[inline(always)]
  pub const fn contexts(&self) -> &Arc<[AssetContext]> {
    &self.contexts
  }

  /// Takes the asset contexts out of the `MetaAndAssetContexts` instance, leaving an empty vector in its place.
  #[inline(always)]
  pub fn take_contexts(&mut self) -> Arc<[AssetContext]> {
    self.contexts.take_arc()
  }

  /// Consumes the `MetaAndAssetContexts` instance and returns the contained `Meta`.
  #[inline(always)]
  pub fn into_meta(self) -> Meta {
    self.into()
  }

  /// Consumes the `MetaAndAssetContexts` instance and returns the contained asset contexts.
  #[inline(always)]
  pub fn into_contexts(self) -> Arc<[AssetContext]> {
    self.into()
  }

  /// Consumes the `MetaAndAssetContexts` instance and returns both the contained `Meta` and asset contexts.
  #[inline(always)]
  pub fn into_parts(self) -> (Meta, Arc<[AssetContext]>) {
    self.into()
  }
}

#[cfg(feature = "serde")]
const _: () = {
  use serde::{Deserializer, Serializer};

  crate::http::serde_response_data!(@forward MetaAndAssetContexts, Meta);

  impl<'de> Deserialize<'de> for MetaAndAssetContexts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      <(Meta, Vec<AssetContext>)>::deserialize(deserializer).map(|(meta, contexts)| Self {
        meta,
        contexts: contexts.into(),
      })
    }
  }

  impl Serialize for MetaAndAssetContexts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      (&self.meta, self.contexts_ref()).serialize(serializer)
    }
  }
};
