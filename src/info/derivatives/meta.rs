use derive_more::{Display, From, Into, IsVariant};
use jiff::civil::DateTime;
use smol_str::SmolStr;

use triomphe::Arc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::utils::TakeArc;
#[cfg(feature = "serde")]
use crate::utils::serde_triomphe_array;

/// The margin tier for derivatives trading.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct MarginTier {
  lower_bound: SmolStr,
  max_leverage: usize,
}

impl MarginTier {
  /// Returns the lower bound of the margin tier.
  #[inline(always)]
  pub const fn lower_bound(&self) -> &SmolStr {
    &self.lower_bound
  }

  /// Returns the maximum leverage of the margin tier.
  #[inline(always)]
  pub const fn max_leverage(&self) -> usize {
    self.max_leverage
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
}

/// The margin table fields
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarginTable {
  id: MarginTableId,
  description: SmolStr,
  margin_tiers: Arc<[MarginTier]>,
}

impl MarginTable {
  /// Returns the ID of the margin table.
  #[inline(always)]
  pub const fn id(&self) -> MarginTableId {
    self.id
  }

  /// Returns the description of the margin table.
  #[inline(always)]
  pub const fn description(&self) -> &SmolStr {
    &self.description
  }

  /// Returns the margin tiers of the margin table.
  #[inline(always)]
  pub fn margin_tiers_ref(&self) -> &[MarginTier] {
    self.margin_tiers.as_ref()
  }

  /// Returns the margin tiers of the margin table
  #[inline(always)]
  pub const fn margin_tiers(&self) -> &Arc<[MarginTier]> {
    &self.margin_tiers
  }

  /// Takes the margin tiers of the margin table, leaving an empty vector in its place.
  #[inline(always)]
  pub fn take_margin_tiers(&mut self) -> Arc<[MarginTier]> {
    self.margin_tiers.take_arc()
  }

  /// Consumes the margin table and returns its margin tiers.
  #[inline(always)]
  pub fn into_margin_tiers(self) -> Arc<[MarginTier]> {
    self.margin_tiers
  }
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
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
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
  last_growth_mode_change_time: Option<DateTime>,
  /// Date and time of the last fee scale change, as reported without a UTC offset.
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  last_fee_scale_change_time: Option<DateTime>,
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  deployer_fee_scale: Option<SmolStr>,
}

impl AssetMeta {
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
}

/// Metadata for the derivatives market, including the universe of assets, collateral token, and margin tables.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct Meta {
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  universe: Arc<[AssetMeta]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_triomphe_array"))]
  margin_tables: Arc<[MarginTable]>,
  collateral_token: usize,
}

impl Meta {
  /// Returns a reference to the universe of assets.
  #[inline(always)]
  pub fn universe(&self) -> &[AssetMeta] {
    self.universe.as_ref()
  }

  /// Returns the index of the collateral token.
  #[inline(always)]
  pub const fn collateral_token(&self) -> usize {
    self.collateral_token
  }

  /// Returns a reference to the margin tables.
  #[inline(always)]
  pub fn margin_tables(&self) -> &[MarginTable] {
    self.margin_tables.as_ref()
  }

  /// Returns the universe of assets, replacing it with an empty vector.
  #[inline(always)]
  pub fn take_universe(&mut self) -> Arc<[AssetMeta]> {
    self.universe.take_arc()
  }

  /// Returns the margin tables, replacing them with an empty vector.
  #[inline(always)]
  pub fn take_margin_tables(&mut self) -> Arc<[MarginTable]> {
    self.margin_tables.take_arc()
  }

  /// Consumes the `Meta` instance and returns the universe of assets.
  #[inline(always)]
  pub fn into_universe(self) -> Arc<[AssetMeta]> {
    self.universe
  }

  /// Consumes the `Meta` instance and returns the margin tables.
  #[inline(always)]
  pub fn into_margin_tables(self) -> Arc<[MarginTable]> {
    self.margin_tables
  }

  /// Consumes the `Meta` instance and returns its parts as a tuple.
  #[inline(always)]
  pub fn into_parts(self) -> (Arc<[AssetMeta]>, Arc<[MarginTable]>, usize) {
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
  /// Returns the bid price of the impact price.
  #[inline(always)]
  pub const fn bid(&self) -> &SmolStr {
    &self.bid
  }

  /// Returns the ask price of the impact price.
  #[inline(always)]
  pub const fn ask(&self) -> &SmolStr {
    &self.ask
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

  /// Returns the funding.
  #[inline(always)]
  pub const fn funding(&self) -> &SmolStr {
    &self.funding
  }

  /// Returns the open interest.
  #[inline(always)]
  pub const fn open_interest(&self) -> &SmolStr {
    &self.open_interest
  }

  /// Returns the premium.
  #[inline(always)]
  pub const fn premium(&self) -> Option<&SmolStr> {
    self.premium.as_ref()
  }

  /// Returns the oracle price.
  #[inline(always)]
  pub const fn oracle_px(&self) -> &SmolStr {
    &self.oracle_px
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

  /// Returns a reference to the contexts field.
  #[inline(always)]
  pub fn contexts_ref(&self) -> &[AssetContext] {
    self.contexts.as_ref()
  }

  /// Returns a refernece to the contexts field.
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
  use crate::utils::serde_triomphe_array;
  use serde::{Deserializer, Serializer};
  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
  struct MarginTableFields {
    description: SmolStr,
    #[serde(with = "serde_triomphe_array")]
    margin_tiers: Arc<[MarginTier]>,
  }

  #[derive(Serialize)]
  #[serde(rename_all = "camelCase")]
  #[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
  struct MarginTableFieldsRef<'a> {
    description: &'a str,
    margin_tiers: &'a [MarginTier],
  }

  impl<'de> Deserialize<'de> for MarginTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let (id, fields) = <(MarginTableId, MarginTableFields)>::deserialize(deserializer)?;
      Ok(MarginTable {
        id,
        description: fields.description,
        margin_tiers: fields.margin_tiers,
      })
    }
  }

  impl Serialize for MarginTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let fields = MarginTableFieldsRef {
        description: &self.description,
        margin_tiers: &self.margin_tiers,
      };
      (self.id, fields).serialize(serializer)
    }
  }

  impl<'de> Deserialize<'de> for MetaAndAssetContexts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      <(Meta, Vec<AssetContext>)>::deserialize(deserializer).map(|(meta, contexts)| Self {
        meta,
        contexts: Arc::from(contexts),
      })
    }
  }

  impl Serialize for MetaAndAssetContexts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let contexts = self.contexts_ref();
      (&self.meta, contexts).serialize(serializer)
    }
  }
};
