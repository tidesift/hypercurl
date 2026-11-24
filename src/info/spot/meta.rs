use core::mem::take;
use std::vec::Vec;

use alloy_primitives::{Address, B128};
use smol_str::SmolStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
  /// Creates a new `AssetMeta` instance with empty values.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      tokens: [0; 2],
      name: SmolStr::new_inline(""),
      index: 0,
      is_canonical: false,
    }
  }

  /// Returns the tokens associated with the spot asset.
  #[inline(always)]
  pub const fn tokens(&self) -> &[usize; 2] {
    &self.tokens
  }

  /// Sets the tokens associated with the spot asset and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_tokens(&mut self, tokens: [usize; 2]) -> &mut Self {
    self.tokens = tokens;
    self
  }

  /// Sets the tokens associated with the spot asset and returns a modified instance.
  #[inline(always)]
  pub const fn with_tokens(mut self, tokens: [usize; 2]) -> Self {
    self.tokens = tokens;
    self
  }

  /// Returns the name of the spot asset.
  #[inline(always)]
  pub const fn name(&self) -> &SmolStr {
    &self.name
  }

  /// Sets the name of the spot asset and returns a mutable reference to self.
  #[inline(always)]
  pub fn set_name(&mut self, name: impl Into<SmolStr>) -> &mut Self {
    self.name = name.into();
    self
  }

  /// Sets the name of the spot asset and returns a modified instance.
  #[inline(always)]
  pub fn with_name(mut self, name: impl Into<SmolStr>) -> Self {
    self.name = name.into();
    self
  }

  /// Returns the index of the spot asset.
  #[inline(always)]
  pub const fn index(&self) -> usize {
    self.index
  }

  /// Sets the index of the spot asset and returns a modified instance.
  #[inline(always)]
  pub const fn with_index(mut self, index: usize) -> Self {
    self.index = index;
    self
  }

  /// Sets the index of the spot asset and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_index(&mut self, index: usize) -> &mut Self {
    self.index = index;
    self
  }

  /// Sets the canonical status of the spot asset.
  #[inline(always)]
  pub const fn with_is_canonical(mut self, is_canonical: bool) -> Self {
    self.is_canonical = is_canonical;
    self
  }

  /// Returns whether the spot asset is canonical.
  #[inline(always)]
  pub const fn is_canonical(&self) -> bool {
    self.is_canonical
  }

  /// Sets whether the spot asset is canonical and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_is_canonical(&mut self, is_canonical: bool) -> &mut Self {
    self.is_canonical = is_canonical;
    self
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
  /// Returns an empty `EvmContract` instance with empty values.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      address: Address::new([0; 20]),
      extra_wei_decimals: 0,
    }
  }

  /// Creates a new `EvmContract` instance with the specified address and extra wei decimals.
  #[inline(always)]
  pub const fn new(address: Address, extra_wei_decimals: isize) -> Self {
    Self {
      address,
      extra_wei_decimals,
    }
  }

  /// Returns the address of the EVM contract.
  #[inline(always)]
  pub const fn address(&self) -> Address {
    self.address
  }

  /// Sets the address of the EVM contract and returns a modified instance.
  #[inline(always)]
  pub const fn with_address(mut self, address: Address) -> Self {
    self.address = address;
    self
  }

  /// Sets the address of the EVM contract and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_address(&mut self, address: Address) -> &mut Self {
    self.address = address;
    self
  }

  /// Returns the extra wei decimals of the EVM contract.
  #[inline(always)]
  pub const fn extra_wei_decimals(&self) -> isize {
    self.extra_wei_decimals
  }

  /// Sets the extra wei decimals of the EVM contract and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_extra_wei_decimals(&mut self, extra_wei_decimals: isize) -> &mut Self {
    self.extra_wei_decimals = extra_wei_decimals;
    self
  }

  /// Sets the extra wei decimals of the EVM contract and returns a modified instance.
  #[inline(always)]
  pub const fn with_extra_wei_decimals(mut self, extra_wei_decimals: isize) -> Self {
    self.extra_wei_decimals = extra_wei_decimals;
    self
  }
}

/// The token information for a spot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct TokenInfo {
  name: SmolStr,
  sz_decimals: u8,
  wei_decimals: u8,
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
  /// Creates a new `TokenInfo` instance with empty values.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      name: SmolStr::new_inline(""),
      sz_decimals: 0,
      wei_decimals: 0,
      index: 0,
      token_id: B128::new([0; 16]),
      is_canonical: false,
      evm_contract: None,
      full_name: None,
      deployer_trading_fee_share: SmolStr::new_inline(""),
      deployer_label: None,
    }
  }

  /// Returns the name of the token.
  #[inline(always)]
  pub const fn name(&self) -> &SmolStr {
    &self.name
  }

  /// Sets the name of the token and returns a mutable reference to self.
  #[inline(always)]
  pub fn set_name(&mut self, name: impl Into<SmolStr>) -> &mut Self {
    self.name = name.into();
    self
  }

  /// Sets the name of the token and returns a modified instance.
  #[inline(always)]
  pub fn with_name(mut self, name: impl Into<SmolStr>) -> Self {
    self.name = name.into();
    self
  }

  /// Returns the size decimals of the token.
  #[inline(always)]
  pub const fn sz_decimals(&self) -> u8 {
    self.sz_decimals
  }

  /// Sets the size decimals of the token and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_sz_decimals(&mut self, sz_decimals: u8) -> &mut Self {
    self.sz_decimals = sz_decimals;
    self
  }

  /// Sets the size decimals of the token and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub const fn with_sz_decimals(mut self, sz_decimals: u8) -> Self {
    self.sz_decimals = sz_decimals;
    self
  }

  /// Returns the wei decimals of the token.
  #[inline(always)]
  pub const fn wei_decimals(&self) -> u8 {
    self.wei_decimals
  }

  /// Sets the wei decimals of the token and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_wei_decimals(&mut self, wei_decimals: u8) -> &mut Self {
    self.wei_decimals = wei_decimals;
    self
  }

  /// Sets the wei decimals of the token and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub const fn with_wei_decimals(mut self, wei_decimals: u8) -> Self {
    self.wei_decimals = wei_decimals;
    self
  }

  /// Returns the index of the token.
  #[inline(always)]
  pub const fn index(&self) -> usize {
    self.index
  }

  /// Sets the index of the token and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_index(&mut self, index: usize) -> &mut Self {
    self.index = index;
    self
  }

  /// Sets the index of the token and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub const fn with_index(mut self, index: usize) -> Self {
    self.index = index;
    self
  }

  /// Returns the token ID.
  #[inline(always)]
  pub const fn token_id(&self) -> B128 {
    self.token_id
  }

  /// Sets the token ID and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_token_id(&mut self, token_id: B128) -> &mut Self {
    self.token_id = token_id;
    self
  }

  /// Sets the token ID and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub const fn with_token_id(mut self, token_id: B128) -> Self {
    self.token_id = token_id;
    self
  }

  /// Returns whether the token is canonical.
  #[inline(always)]
  pub const fn is_canonical(&self) -> bool {
    self.is_canonical
  }

  /// Sets whether the token is canonical and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_is_canonical(&mut self, is_canonical: bool) -> &mut Self {
    self.is_canonical = is_canonical;
    self
  }

  /// Sets whether the token is canonical and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub const fn with_is_canonical(mut self, is_canonical: bool) -> Self {
    self.is_canonical = is_canonical;
    self
  }

  /// Returns the EVM contract associated with the token, if any.
  #[inline(always)]
  pub const fn evm_contract(&self) -> Option<&EvmContract> {
    self.evm_contract.as_ref()
  }

  /// Sets the EVM contract and returns a mutable reference to self.
  #[inline(always)]
  pub const fn set_evm_contract(&mut self, evm_contract: Option<EvmContract>) -> &mut Self {
    self.evm_contract = evm_contract;
    self
  }

  /// Sets the EVM contract and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub const fn with_evm_contract(mut self, evm_contract: Option<EvmContract>) -> Self {
    self.evm_contract = evm_contract;
    self
  }

  /// Returns the full name of the token, if any.
  #[inline(always)]
  pub const fn full_name(&self) -> Option<&SmolStr> {
    self.full_name.as_ref()
  }

  /// Sets the full name of the token and returns a mutable reference to self.
  #[inline(always)]
  pub fn set_full_name(&mut self, full_name: Option<impl Into<SmolStr>>) -> &mut Self {
    self.full_name = full_name.map(Into::into);
    self
  }

  /// Sets the full name of the token and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub fn with_full_name(mut self, full_name: Option<impl Into<SmolStr>>) -> Self {
    self.full_name = full_name.map(Into::into);
    self
  }

  /// Returns the deployer trading fee share associated with the token.
  #[inline(always)]
  pub const fn deployer_trading_fee_share(&self) -> &SmolStr {
    &self.deployer_trading_fee_share
  }

  /// Sets the deployer trading fee share and returns a mutable reference to self.
  #[inline(always)]
  pub fn set_deployer_trading_fee_share(
    &mut self,
    deployer_trading_fee_share: SmolStr,
  ) -> &mut Self {
    self.deployer_trading_fee_share = deployer_trading_fee_share;
    self
  }

  /// Sets the deployer trading fee share and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub fn with_deployer_trading_fee_share(mut self, deployer_trading_fee_share: SmolStr) -> Self {
    self.deployer_trading_fee_share = deployer_trading_fee_share;
    self
  }

  /// Returns the deployer label associated with the token, if any.
  #[inline(always)]
  pub const fn deployer_label(&self) -> Option<&SmolStr> {
    self.deployer_label.as_ref()
  }

  /// Sets the deployer label and returns a mutable reference to self.
  #[inline(always)]
  pub fn set_deployer_label(&mut self, deployer_label: Option<impl Into<SmolStr>>) -> &mut Self {
    self.deployer_label = deployer_label.map(Into::into);
    self
  }

  /// Sets the deployer label and returns a new instance of `TokenInfo`.
  #[inline(always)]
  pub fn with_deployer_label(mut self, deployer_label: Option<impl Into<SmolStr>>) -> Self {
    self.deployer_label = deployer_label.map(Into::into);
    self
  }
}

/// The metadata associated with a spot.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct Meta {
  universe: Vec<AssetMeta>,
  tokens: Vec<TokenInfo>,
}

impl From<Meta> for Vec<TokenInfo> {
  fn from(spot_meta: Meta) -> Self {
    spot_meta.into_tokens()
  }
}

impl From<Meta> for Vec<AssetMeta> {
  fn from(spot_meta: Meta) -> Self {
    spot_meta.into_universe()
  }
}

impl Meta {
  /// Creates a new `Meta` instance with empty universe and tokens.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      universe: Vec::new(),
      tokens: Vec::new(),
    }
  }

  /// Returns the universe of spot asset metadata.
  #[inline(always)]
  pub const fn universe(&self) -> &[AssetMeta] {
    self.universe.as_slice()
  }

  /// Returns a mutable reference to the universe of spot asset metadata.
  #[inline(always)]
  pub const fn universe_mut(&mut self) -> &mut [AssetMeta] {
    self.universe.as_mut_slice()
  }

  /// Returns the spot's base asset metadata.
  #[inline(always)]
  pub const fn tokens(&self) -> &[TokenInfo] {
    self.tokens.as_slice()
  }

  /// Returns a mutable reference to the spot's base asset metadata.
  #[inline(always)]
  pub const fn tokens_mut(&mut self) -> &mut [TokenInfo] {
    self.tokens.as_mut_slice()
  }

  /// Takes the spot's base asset metadata, leaving an empty vector in its place.
  #[inline(always)]
  pub fn take_tokens(&mut self) -> Vec<TokenInfo> {
    take(&mut self.tokens)
  }

  /// Takes the universe of spot asset metadata, leaving an empty vector in its place.
  #[inline(always)]
  pub fn take_universe(&mut self) -> Vec<AssetMeta> {
    take(&mut self.universe)
  }

  /// Consumes the `Meta` instance and returns the spot's base asset metadata.
  #[inline(always)]
  pub fn into_tokens(self) -> Vec<TokenInfo> {
    self.tokens
  }

  /// Consumes the `Meta` instance and returns the universe of spot asset metadata.
  #[inline(always)]
  pub fn into_universe(self) -> Vec<AssetMeta> {
    self.universe
  }

  /// Consumes the `Meta` instance and returns both the universe and the spot's base asset metadata.
  #[inline(always)]
  pub fn into_part(self) -> (Vec<AssetMeta>, Vec<TokenInfo>) {
    (self.universe, self.tokens)
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
  /// Creates a new `AssetContext` instance with empty values.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      day_ntl_vlm: SmolStr::new_inline(""),
      mark_px: SmolStr::new_inline(""),
      mid_px: None,
      prev_day_px: SmolStr::new_inline(""),
      circulating_supply: SmolStr::new_inline(""),
      coin: SmolStr::new_inline(""),
      total_supply: SmolStr::new_inline(""),
      day_base_vlm: SmolStr::new_inline(""),
    }
  }

  /// Returns the day notional volume.
  #[inline(always)]
  pub const fn day_ntl_vlm(&self) -> &SmolStr {
    &self.day_ntl_vlm
  }

  /// Sets the day notional volume.
  #[inline(always)]
  pub fn with_day_ntl_vlm(mut self, day_ntl_vlm: impl Into<SmolStr>) -> Self {
    self.day_ntl_vlm = day_ntl_vlm.into();
    self
  }

  /// Sets the day notional volume.
  #[inline(always)]
  pub fn set_day_ntl_vlm(&mut self, day_ntl_vlm: impl Into<SmolStr>) -> &mut Self {
    self.day_ntl_vlm = day_ntl_vlm.into();
    self
  }

  /// Returns the mark price.
  #[inline(always)]
  pub const fn mark_px(&self) -> &SmolStr {
    &self.mark_px
  }

  /// Sets the mark price.
  #[inline(always)]
  pub fn with_mark_px(mut self, mark_px: impl Into<SmolStr>) -> Self {
    self.mark_px = mark_px.into();
    self
  }

  /// Sets the mark price.
  #[inline(always)]
  pub fn set_mark_px(&mut self, mark_px: impl Into<SmolStr>) -> &mut Self {
    self.mark_px = mark_px.into();
    self
  }

  /// Returns the mid price, if available.
  #[inline(always)]
  pub fn with_mid_px(mut self, mid_px: Option<impl Into<SmolStr>>) -> Self {
    self.mid_px = mid_px.map(Into::into);
    self
  }

  /// Sets the mid price.
  #[inline(always)]
  pub fn set_mid_px(&mut self, mid_px: Option<impl Into<SmolStr>>) -> &mut Self {
    self.mid_px = mid_px.map(Into::into);
    self
  }

  /// Returns the mid price, if available.
  #[inline(always)]
  pub const fn mid_px(&self) -> Option<&SmolStr> {
    self.mid_px.as_ref()
  }

  /// Sets the previous day's price.
  #[inline(always)]
  pub fn with_prev_day_px(mut self, prev_day_px: impl Into<SmolStr>) -> Self {
    self.prev_day_px = prev_day_px.into();
    self
  }

  /// Sets the previous day's price.
  #[inline(always)]
  pub fn set_prev_day_px(&mut self, prev_day_px: impl Into<SmolStr>) -> &mut Self {
    self.prev_day_px = prev_day_px.into();
    self
  }

  /// Returns the previous day's price.
  #[inline(always)]
  pub const fn prev_day_px(&self) -> &SmolStr {
    &self.prev_day_px
  }

  /// Sets the circulating supply.
  #[inline(always)]
  pub fn with_circulating_supply(mut self, circulating_supply: impl Into<SmolStr>) -> Self {
    self.circulating_supply = circulating_supply.into();
    self
  }

  /// Sets the circulating supply.
  #[inline(always)]
  pub fn set_circulating_supply(&mut self, circulating_supply: impl Into<SmolStr>) -> &mut Self {
    self.circulating_supply = circulating_supply.into();
    self
  }

  /// Returns the circulating supply.
  #[inline(always)]
  pub const fn circulating_supply(&self) -> &SmolStr {
    &self.circulating_supply
  }

  /// Sets the coin identifier.
  #[inline(always)]
  pub fn with_coin(mut self, coin: impl Into<SmolStr>) -> Self {
    self.coin = coin.into();
    self
  }

  /// Sets the coin identifier.
  #[inline(always)]
  pub fn set_coin(&mut self, coin: impl Into<SmolStr>) -> &mut Self {
    self.coin = coin.into();
    self
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

  /// Sets the total supply.
  #[inline(always)]
  pub fn with_total_supply(mut self, total_supply: impl Into<SmolStr>) -> Self {
    self.total_supply = total_supply.into();
    self
  }

  /// Sets the total supply.
  #[inline(always)]
  pub fn set_total_supply(&mut self, total_supply: impl Into<SmolStr>) -> &mut Self {
    self.total_supply = total_supply.into();
    self
  }

  /// Returns the day base volume.
  #[inline(always)]
  pub const fn day_base_vlm(&self) -> &SmolStr {
    &self.day_base_vlm
  }

  /// Sets the day base volume.
  #[inline(always)]
  pub fn with_day_base_vlm(mut self, day_base_vlm: impl Into<SmolStr>) -> Self {
    self.day_base_vlm = day_base_vlm.into();
    self
  }

  /// Sets the day base volume.
  #[inline(always)]
  pub fn set_day_base_vlm(&mut self, day_base_vlm: impl Into<SmolStr>) -> &mut Self {
    self.day_base_vlm = day_base_vlm.into();
    self
  }
}

/// Represents the metadata and asset contexts for derivatives.
#[derive(Debug, Clone)]
pub struct MetaAndAssetContexts {
  /// Represents derivatives metadata.
  meta: Meta,
  /// Represents asset contexts.
  contexts: Vec<AssetContext>,
}

impl From<MetaAndAssetContexts> for Meta {
  #[inline(always)]
  fn from(value: MetaAndAssetContexts) -> Self {
    value.meta
  }
}

impl From<MetaAndAssetContexts> for Vec<AssetContext> {
  #[inline(always)]
  fn from(value: MetaAndAssetContexts) -> Self {
    value.contexts
  }
}

impl From<MetaAndAssetContexts> for (Meta, Vec<AssetContext>) {
  #[inline(always)]
  fn from(value: MetaAndAssetContexts) -> Self {
    (value.meta, value.contexts)
  }
}

impl MetaAndAssetContexts {
  /// Creates an empty instance of `MetaAndAssetContexts` with an empty meta and no asset contexts.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self::new(Meta::empty(), Vec::new())
  }

  /// Creates a new instance of `MetaAndAssetContexts` with the given meta and asset contexts.
  #[inline(always)]
  pub const fn new(meta: Meta, contexts: Vec<AssetContext>) -> Self {
    Self { meta, contexts }
  }

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
  pub const fn contexts(&self) -> &[AssetContext] {
    self.contexts.as_slice()
  }

  /// Returns a mutable reference to the contexts field.
  #[inline(always)]
  pub const fn contexts_mut(&mut self) -> &mut [AssetContext] {
    self.contexts.as_mut_slice()
  }

  /// Takes the asset contexts out of the `MetaAndAssetContexts` instance, leaving an empty vector in its place.
  #[inline(always)]
  pub fn take_contexts(&mut self) -> Vec<AssetContext> {
    take(&mut self.contexts)
  }

  /// Consumes the `MetaAndAssetContexts` instance and returns the contained `Meta`.
  #[inline(always)]
  pub fn into_meta(self) -> Meta {
    self.into()
  }

  /// Consumes the `MetaAndAssetContexts` instance and returns the contained asset contexts.
  #[inline(always)]
  pub fn into_contexts(self) -> Vec<AssetContext> {
    self.into()
  }

  /// Consumes the `MetaAndAssetContexts` instance and returns both the contained `Meta` and asset contexts.
  #[inline(always)]
  pub fn into_parts(self) -> (Meta, Vec<AssetContext>) {
    self.into()
  }
}

#[cfg(feature = "serde")]
const _: () = {
  use serde::{Deserializer, Serializer};

  impl<'de> Deserialize<'de> for MetaAndAssetContexts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      <(Meta, Vec<AssetContext>)>::deserialize(deserializer)
        .map(|(meta, contexts)| Self::new(meta, contexts))
    }
  }

  impl Serialize for MetaAndAssetContexts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      (&self.meta, &self.contexts).serialize(serializer)
    }
  }
};
