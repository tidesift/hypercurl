use core::mem::take;
use std::vec::Vec;

use derive_more::{Display, From, Into, IsVariant, TryUnwrap, Unwrap};
use jiff::civil::DateTime;
use smol_str::SmolStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The margin tier for derivatives trading.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MarginTier {
  lower_bound: SmolStr,
  max_leverage: usize,
}

impl MarginTier {
  /// Creates a new margin tier with empty values.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      lower_bound: SmolStr::new_inline(""),
      max_leverage: 0,
    }
  }

  /// Returns the lower bound of the margin tier.
  #[inline(always)]
  pub const fn lower_bound(&self) -> &SmolStr {
    &self.lower_bound
  }

  /// Sets the lower bound of the margin tier.
  #[inline(always)]
  pub fn set_lower_bound(&mut self, lower_bound: impl Into<SmolStr>) -> &mut Self {
    self.lower_bound = lower_bound.into();
    self
  }

  /// Returns a new margin tier with the specified lower bound.
  #[inline(always)]
  pub fn with_lower_bound(mut self, lower_bound: impl Into<SmolStr>) -> Self {
    self.lower_bound = lower_bound.into();
    self
  }

  /// Returns the maximum leverage of the margin tier.
  #[inline(always)]
  pub const fn max_leverage(&self) -> usize {
    self.max_leverage
  }

  /// Sets the maximum leverage of the margin tier.
  #[inline(always)]
  pub const fn set_max_leverage(&mut self, max_leverage: usize) -> &mut Self {
    self.max_leverage = max_leverage;
    self
  }

  /// Sets the maximum leverage of the margin tier and returns a new instance.
  #[inline(always)]
  pub const fn with_max_leverage(mut self, max_leverage: usize) -> Self {
    self.max_leverage = max_leverage;
    self
  }
}

/// The ID of a margin table.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct MarginTableId(u64);

impl MarginTableId {
  /// Creates a new margin table ID with the specified value.
  #[inline(always)]
  pub const fn new(value: u64) -> Self {
    Self(value)
  }

  /// Returns the value of the margin table ID.
  #[inline(always)]
  pub const fn value(&self) -> u64 {
    self.0
  }

  /// Sets the value of the margin table ID.
  #[inline(always)]
  pub const fn set_value(&mut self, value: u64) -> &mut Self {
    self.0 = value;
    self
  }

  /// Sets the value of the margin table ID and returns a new instance.
  #[inline(always)]
  pub const fn with_value(mut self, value: u64) -> Self {
    self.0 = value;
    self
  }
}

/// The margin table fields
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MarginTableFields {
  description: SmolStr,
  margin_tiers: Vec<MarginTier>,
}

/// The margin table for derivatives trading.
#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Unwrap, TryUnwrap, From)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum MarginTable {
  /// The ID of the margin table.
  Id(MarginTableId),
  /// The fields of the margin table.
  Fields(MarginTableFields),
}

/// Growth mode for a perpetual market.
/// Determines how the growth of positions is managed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IsVariant, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[display("{}", self.as_str())]
pub enum GrowthMode {
  /// Growth mode is enabled.
  Enabled,
}

impl GrowthMode {
  /// Returns the string representation of the growth mode.
  #[inline(always)]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Enabled => "enabled",
    }
  }
}

/// Margin mode for a perpetual market.
/// Determines how margin is managed across positions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IsVariant, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[display("{}", self.as_str())]
pub enum MarginMode {
  /// Strict isolated margin mode.
  StrictIsolated,
  /// No cross margin mode.
  NoCross,
}

impl MarginMode {
  /// Returns the string representation of the margin mode.
  #[inline(always)]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::StrictIsolated => "strictIsolated",
      Self::NoCross => "noCross",
    }
  }
}

/// Trading pair metadata for derivatives trading.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AssetMeta {
  sz_decimals: u32,
  name: SmolStr,
  max_leverage: usize,
  margin_table_id: MarginTableId,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  only_isolated: Option<bool>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  is_delisted: Option<bool>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  margin_mode: Option<MarginMode>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  growth_mode: Option<GrowthMode>,
  /// Date and time of the last growth mode change, as reported without a UTC offset.
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  last_growth_mode_change_time: Option<jiff::civil::DateTime>,
  /// Date and time of the last fee scale change, as reported without a UTC offset.
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  last_fee_scale_change_time: Option<jiff::civil::DateTime>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  deployer_fee_scale: Option<SmolStr>,
}

impl AssetMeta {
  /// Creates a new instance of `AssetMeta` with empty values.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      sz_decimals: 0,
      name: SmolStr::new_inline(""),
      max_leverage: 0,
      margin_table_id: MarginTableId::new(0),
      only_isolated: None,
      is_delisted: None,
      margin_mode: None,
      growth_mode: None,
      deployer_fee_scale: None,
      last_growth_mode_change_time: None,
      last_fee_scale_change_time: None,
    }
  }

  /// Returns the number of decimal places for size precision.
  #[inline(always)]
  pub const fn sz_decimals(&self) -> u32 {
    self.sz_decimals
  }

  /// Returns the trading pair symbol.
  #[inline(always)]
  pub const fn name(&self) -> &SmolStr {
    &self.name
  }

  /// Returns the maximum leverage allowed for this pair.
  #[inline(always)]
  pub const fn max_leverage(&self) -> usize {
    self.max_leverage
  }

  /// Returns the ID referencing the margin table configuration.
  #[inline(always)]
  pub const fn margin_table_id(&self) -> MarginTableId {
    self.margin_table_id
  }

  /// Returns whether this pair can only be traded in isolated margin mode.
  #[inline(always)]
  pub const fn only_isolated(&self) -> Option<bool> {
    self.only_isolated
  }

  /// Returns whether this trading pair has been delisted.
  #[inline(always)]
  pub const fn is_delisted(&self) -> Option<bool> {
    self.is_delisted
  }

  /// Returns the margin mode for this trading pair.
  #[inline(always)]
  pub const fn margin_mode(&self) -> Option<MarginMode> {
    self.margin_mode
  }

  /// Returns the growth mode for this trading pair.
  #[inline(always)]
  pub const fn growth_mode(&self) -> Option<GrowthMode> {
    self.growth_mode
  }

  /// Returns the date and time of the last growth mode change, as reported without a UTC offset.
  #[inline(always)]
  pub const fn last_growth_mode_change_time(&self) -> Option<DateTime> {
    self.last_growth_mode_change_time
  }

  /// Returns the date and time of the last fee scale change, as reported without a UTC offset.
  #[inline(always)]
  pub const fn last_fee_scale_change_time(&self) -> Option<DateTime> {
    self.last_fee_scale_change_time
  }

  /// Returns the deployer fee scale.
  #[inline(always)]
  pub const fn deployer_fee_scale(&self) -> Option<&SmolStr> {
    self.deployer_fee_scale.as_ref()
  }

  /// Sets the number of decimal places for size precision.
  #[inline(always)]
  pub const fn set_sz_decimals(&mut self, sz_decimals: u32) -> &mut Self {
    self.sz_decimals = sz_decimals;
    self
  }

  /// Sets the trading pair symbol.
  #[inline(always)]
  pub fn set_name(&mut self, name: impl Into<SmolStr>) -> &mut Self {
    self.name = name.into();
    self
  }

  /// Sets the maximum leverage allowed for this pair.
  #[inline(always)]
  pub const fn set_max_leverage(&mut self, max_leverage: usize) -> &mut Self {
    self.max_leverage = max_leverage;
    self
  }

  /// Sets the ID referencing the margin table configuration.
  #[inline(always)]
  pub const fn set_margin_table_id(&mut self, margin_table_id: MarginTableId) -> &mut Self {
    self.margin_table_id = margin_table_id;
    self
  }

  /// Sets whether this pair can only be traded in isolated margin mode.
  #[inline(always)]
  pub const fn set_only_isolated(&mut self, only_isolated: Option<bool>) -> &mut Self {
    self.only_isolated = only_isolated;
    self
  }

  /// Sets whether this trading pair has been delisted.
  #[inline(always)]
  pub const fn set_is_delisted(&mut self, is_delisted: Option<bool>) -> &mut Self {
    self.is_delisted = is_delisted;
    self
  }

  /// Sets the margin mode for this trading pair.
  #[inline(always)]
  pub const fn set_margin_mode(&mut self, margin_mode: Option<MarginMode>) -> &mut Self {
    self.margin_mode = margin_mode;
    self
  }

  /// Sets the growth mode for this trading pair.
  #[inline(always)]
  pub const fn set_growth_mode(&mut self, growth_mode: Option<GrowthMode>) -> &mut Self {
    self.growth_mode = growth_mode;
    self
  }

  /// Sets the last growth mode change time.
  #[inline(always)]
  pub const fn set_last_growth_mode_change_time(
    &mut self,
    last_growth_mode_change_time: Option<DateTime>,
  ) -> &mut Self {
    self.last_growth_mode_change_time = last_growth_mode_change_time;
    self
  }

  /// Sets the last fee scale change time.
  #[inline(always)]
  pub const fn set_last_fee_scale_change_time(
    &mut self,
    last_fee_scale_change_time: Option<DateTime>,
  ) -> &mut Self {
    self.last_fee_scale_change_time = last_fee_scale_change_time;
    self
  }

  /// Sets the deployer fee scale.
  #[inline(always)]
  pub fn set_deployer_fee_scale(
    &mut self,
    deployer_fee_scale: Option<impl Into<SmolStr>>,
  ) -> &mut Self {
    self.deployer_fee_scale = deployer_fee_scale.map(Into::into);
    self
  }

  /// Returns a new instance with the specified size decimals.
  #[inline(always)]
  pub const fn with_sz_decimals(mut self, sz_decimals: u32) -> Self {
    self.sz_decimals = sz_decimals;
    self
  }

  /// Returns a new instance with the specified trading pair symbol.
  #[inline(always)]
  pub fn with_name(mut self, name: impl Into<SmolStr>) -> Self {
    self.name = name.into();
    self
  }

  /// Returns a new instance with the specified maximum leverage.
  #[inline(always)]
  pub const fn with_max_leverage(mut self, max_leverage: usize) -> Self {
    self.max_leverage = max_leverage;
    self
  }

  /// Returns a new instance with the specified margin table ID.
  #[inline(always)]
  pub const fn with_margin_table_id(mut self, margin_table_id: MarginTableId) -> Self {
    self.margin_table_id = margin_table_id;
    self
  }

  /// Returns a new instance with the specified isolated margin mode setting.
  #[inline(always)]
  pub const fn with_only_isolated(mut self, only_isolated: Option<bool>) -> Self {
    self.only_isolated = only_isolated;
    self
  }

  /// Returns a new instance with the specified delisted status.
  #[inline(always)]
  pub const fn with_is_delisted(mut self, is_delisted: Option<bool>) -> Self {
    self.is_delisted = is_delisted;
    self
  }

  /// Returns a new instance with the specified margin mode.
  #[inline(always)]
  pub const fn with_margin_mode(mut self, margin_mode: Option<MarginMode>) -> Self {
    self.margin_mode = margin_mode;
    self
  }

  /// Returns a new instance with the specified growth mode.
  #[inline(always)]
  pub const fn with_growth_mode(mut self, growth_mode: Option<GrowthMode>) -> Self {
    self.growth_mode = growth_mode;
    self
  }

  /// Returns a new instance with the specified last growth mode change time.
  #[inline(always)]
  pub const fn with_last_growth_mode_change_time(
    mut self,
    last_growth_mode_change_time: Option<DateTime>,
  ) -> Self {
    self.last_growth_mode_change_time = last_growth_mode_change_time;
    self
  }

  /// Returns a new instance with the specified last fee scale change time.
  #[inline(always)]
  pub const fn with_last_fee_scale_change_time(
    mut self,
    last_fee_scale_change_time: Option<DateTime>,
  ) -> Self {
    self.last_fee_scale_change_time = last_fee_scale_change_time;
    self
  }

  /// Returns a new instance with the specified deployer fee scale.
  #[inline(always)]
  pub fn with_deployer_fee_scale(mut self, deployer_fee_scale: Option<impl Into<SmolStr>>) -> Self {
    self.deployer_fee_scale = deployer_fee_scale.map(Into::into);
    self
  }
}

/// Metadata for the derivatives market, including the universe of assets, collateral token, and margin tables.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Meta {
  universe: Vec<AssetMeta>,
  margin_tables: Vec<MarginTable>,
  collateral_token: usize,
}

impl Meta {
  /// Returns a reference to the universe of assets.
  #[inline(always)]
  pub const fn universe(&self) -> &[AssetMeta] {
    self.universe.as_slice()
  }

  /// Returns the index of the collateral token.
  #[inline(always)]
  pub const fn collateral_token(&self) -> usize {
    self.collateral_token
  }

  /// Returns a reference to the margin tables.
  #[inline(always)]
  pub const fn margin_tables(&self) -> &[MarginTable] {
    self.margin_tables.as_slice()
  }

  /// Returns a mutable reference to the universe of assets.
  #[inline(always)]
  pub const fn universe_mut(&mut self) -> &mut [AssetMeta] {
    self.universe.as_mut_slice()
  }

  /// Returns a mutable reference to the margin tables.
  #[inline(always)]
  pub const fn margin_tables_mut(&mut self) -> &mut [MarginTable] {
    self.margin_tables.as_mut_slice()
  }

  /// Sets the universe of assets, replacing the existing one.
  #[inline(always)]
  pub fn set_universe(&mut self, universe: Vec<AssetMeta>) -> &mut Self {
    self.universe = universe;
    self
  }

  /// Sets the margin tables, replacing the existing ones.
  #[inline(always)]
  pub fn set_margin_tables(&mut self, margin_tables: Vec<MarginTable>) -> &mut Self {
    self.margin_tables = margin_tables;
    self
  }

  /// Returns a new instance with the specified universe of assets.
  #[inline(always)]
  pub fn with_universe(mut self, universe: Vec<AssetMeta>) -> Self {
    self.universe = universe;
    self
  }

  /// Returns a new instance with the specified margin tables.
  #[inline(always)]
  pub fn with_margin_tables(mut self, margin_tables: Vec<MarginTable>) -> Self {
    self.margin_tables = margin_tables;
    self
  }

  /// Returns the universe of assets, replacing it with an empty vector.
  #[inline(always)]
  pub fn take_universe(&mut self) -> Vec<AssetMeta> {
    take(&mut self.universe)
  }

  /// Returns the margin tables, replacing them with an empty vector.
  #[inline(always)]
  pub fn take_margin_tables(&mut self) -> Vec<MarginTable> {
    take(&mut self.margin_tables)
  }

  /// Consumes the `Meta` instance and returns the universe of assets.
  #[inline(always)]
  pub fn into_universe(self) -> Vec<AssetMeta> {
    self.universe
  }

  /// Consumes the `Meta` instance and returns the margin tables.
  #[inline(always)]
  pub fn into_margin_tables(self) -> Vec<MarginTable> {
    self.margin_tables
  }

  /// Consumes the `Meta` instance and returns its parts as a tuple.
  #[inline(always)]
  pub fn into_parts(self) -> (Vec<AssetMeta>, Vec<MarginTable>, usize) {
    (self.universe, self.margin_tables, self.collateral_token)
  }
}

/// The impact price for a derivative asset, containing bid and ask prices.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImpactPrices {
  bid: SmolStr,
  ask: SmolStr,
}

impl ImpactPrices {
  /// Creates an empty `ImpactPrices` instance with bid and ask prices set to empty strings.
  #[inline(always)]
  pub const fn empty() -> Self {
    Self {
      bid: SmolStr::new_inline(""),
      ask: SmolStr::new_inline(""),
    }
  }

  /// Creates a new `ImpactPrices` instance with the specified bid and ask prices.
  #[inline(always)]
  pub fn new(bid: impl Into<SmolStr>, ask: impl Into<SmolStr>) -> Self {
    Self {
      bid: bid.into(),
      ask: ask.into(),
    }
  }

  /// Returns the bid price of the impact price.
  #[inline(always)]
  pub const fn bid(&self) -> &SmolStr {
    &self.bid
  }

  /// Sets the bid price of the impact price and returns a new instance.
  #[inline(always)]
  pub fn with_bid(mut self, bid: impl Into<SmolStr>) -> Self {
    self.bid = bid.into();
    self
  }

  /// Sets the bid price of the impact price.
  #[inline(always)]
  pub fn set_bid(&mut self, bid: impl Into<SmolStr>) -> &mut Self {
    self.bid = bid.into();
    self
  }

  /// Returns the ask price of the impact price.
  #[inline(always)]
  pub const fn ask(&self) -> &SmolStr {
    &self.ask
  }

  /// Sets the ask price of the impact price and returns a new instance.
  #[inline(always)]
  pub fn with_ask(mut self, ask: impl Into<SmolStr>) -> Self {
    self.ask = ask.into();
    self
  }

  /// Sets the ask price of the impact price.
  #[inline(always)]
  pub fn set_ask(&mut self, ask: impl Into<SmolStr>) -> &mut Self {
    self.ask = ask.into();
    self
  }
}

#[cfg(feature = "serde")]
mod impact_price_serde {
  use serde::Serializer;

  use super::*;

  pub fn serialize<S>(impact_price: &Option<ImpactPrices>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match impact_price {
      Some(pxs) => <[&str; 2]>::serialize(&[&pxs.bid, &pxs.ask], serializer),
      None => serializer.serialize_unit(),
    }
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ImpactPrices>, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    <Option<[&str; 2]>>::deserialize(deserializer).map(|opt_arr| {
      opt_arr.map(|arr| ImpactPrices {
        bid: arr[0].into(),
        ask: arr[1].into(),
      })
    })
  }
}

/// Perpetual asset context.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AssetContext {
  prev_day_px: SmolStr,
  day_ntl_vlm: SmolStr,
  mark_px: SmolStr,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  mid_px: Option<SmolStr>,
  funding: SmolStr,
  open_interest: SmolStr,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  premium: Option<SmolStr>,
  oracle_px: SmolStr,
  #[cfg_attr(
    feature = "serde",
    serde(
      default,
      skip_serializing_if = "Option::is_none",
      with = "impact_price_serde"
    )
  )]
  impact_pxs: Option<ImpactPrices>,
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
      day_base_vlm: SmolStr::new_inline(""),
      funding: SmolStr::new_inline(""),
      open_interest: SmolStr::new_inline(""),
      premium: None,
      oracle_px: SmolStr::new_inline(""),
      impact_pxs: None,
    }
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

  /// Sets the funding.
  #[inline(always)]
  pub fn with_funding(mut self, funding: impl Into<SmolStr>) -> Self {
    self.funding = funding.into();
    self
  }

  /// Sets the funding.
  #[inline(always)]
  pub fn set_funding(&mut self, funding: impl Into<SmolStr>) -> &mut Self {
    self.funding = funding.into();
    self
  }

  /// Returns the funding.
  #[inline(always)]
  pub const fn funding(&self) -> &SmolStr {
    &self.funding
  }

  /// Sets the open interest.
  #[inline(always)]
  pub fn with_open_interest(mut self, open_interest: impl Into<SmolStr>) -> Self {
    self.open_interest = open_interest.into();
    self
  }

  /// Sets the open interest.
  #[inline(always)]
  pub fn set_open_interest(&mut self, open_interest: impl Into<SmolStr>) -> &mut Self {
    self.open_interest = open_interest.into();
    self
  }

  /// Returns the open interest.
  #[inline(always)]
  pub const fn open_interest(&self) -> &SmolStr {
    &self.open_interest
  }

  /// Set the premium
  #[inline(always)]
  pub fn with_premium(mut self, premium: Option<impl Into<SmolStr>>) -> Self {
    self.premium = premium.map(Into::into);
    self
  }

  /// Sets the premium.
  #[inline(always)]
  pub fn set_premium(&mut self, premium: Option<impl Into<SmolStr>>) -> &mut Self {
    self.premium = premium.map(Into::into);
    self
  }

  /// Returns the premium.
  #[inline(always)]
  pub const fn premium(&self) -> Option<&SmolStr> {
    self.premium.as_ref()
  }

  /// Set the oracle price.
  #[inline(always)]
  pub fn with_oracle_px(mut self, oracle_px: impl Into<SmolStr>) -> Self {
    self.oracle_px = oracle_px.into();
    self
  }

  /// Sets the oracle price.
  #[inline(always)]
  pub fn set_oracle_px(&mut self, oracle_px: impl Into<SmolStr>) -> &mut Self {
    self.oracle_px = oracle_px.into();
    self
  }

  /// Returns the oracle price.
  #[inline(always)]
  pub const fn oracle_px(&self) -> &SmolStr {
    &self.oracle_px
  }

  /// Sets the impact price.
  #[inline(always)]
  pub fn with_impact_pxs(mut self, impact_px: Option<ImpactPrices>) -> Self {
    self.impact_pxs = impact_px;
    self
  }

  /// Sets the impact price.
  #[inline(always)]
  pub fn set_impact_pxs(&mut self, impact_px: Option<ImpactPrices>) -> &mut Self {
    self.impact_pxs = impact_px;
    self
  }

  /// Returns the impact price.
  #[inline(always)]
  pub const fn impact_pxs(&self) -> Option<&ImpactPrices> {
    self.impact_pxs.as_ref()
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
