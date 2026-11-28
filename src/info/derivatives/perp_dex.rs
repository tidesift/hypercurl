use alloy_primitives::Address;
use smol_str::SmolStr;

use derive_more::Into;
use triomphe::Arc;

use super::sub_deployer::SubDeployer;

#[cfg(feature = "serde")]
use crate::{http::ResponseData, utils::serde_arc_array};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! asset_to {
  ($((
    $(#[$meta:meta])*
    $name:ident($field_name:ident: $to:ident: $description:literal)
  )), +$(,)?) => {
    paste::paste! {
      $(
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Into)]
        #[repr(transparent)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct [<AssetTo $name:camel >](AssetTo<$to>);

        impl [<AssetTo $name:camel >] {
          /// Returns a reference to the asset in the mapping.
          #[inline(always)]
          pub const fn asset(&self) -> &SmolStr {
            self.0.asset()
          }

          #[doc = "Returns a reference to the " $description " in the mapping."]
          #[inline(always)]
          pub const fn $field_name(&self) -> &$to {
            self.0.to()
          }

          #[doc = "Consumes the " [<AssetTo $name:camel>] " instance and returns the asset component."]
          #[inline(always)]
          pub fn into_asset(self) -> SmolStr {
            self.0.into_asset()
          }

          #[doc = "Consumes the " [<AssetTo $name:camel>] " instance and returns the " $description " component."]
          #[inline(always)]
          pub fn [< into_ $field_name >](self) -> $to {
            self.0.into_to()
          }

          #[doc = "Consumes the " [<AssetTo $name:camel>] " instance and returns its components as a tuple `(asset, " $field_name ")"]
          #[inline(always)]
          pub fn into_parts(self) -> (SmolStr, $to) {
            self.0.into_parts()
          }
        }
      )*
    }
  };
}

asset_to!(
  (
    /// Mapping of asset symbols to their streaming open interest caps.
    streaming_oi_cap(oi_cap:SmolStr: "streaming open interest caps")
  ),
  (
    /// Mapping of asset symbols to their funding multipliers.
    funding_multiplier(multiplier:SmolStr: "funding multipliers")
  ),
  (
    /// Mapping of asset symbols to their funding interest rates.
    funding_interest_rate(rate:SmolStr: "funding interest rates")
  ),
  (
    /// Mapping of asset symbols to their funding clamps.
    funding_clamp(clamp:SmolStr: "funding clamps")
  )
);

/// The mapping of an asset to another value, typically used to represent relationships between assets and their corresponding data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetTo<T, A = SmolStr> {
  asset: A,
  to: T,
}

impl<T, A> AssetTo<T, A> {
  /// Creates a new `AssetTo` mapping with the given asset and corresponding value.
  #[inline(always)]
  pub const fn new(asset: A, to: T) -> Self {
    Self { asset, to }
  }

  /// Returns a reference to the asset in the mapping.
  #[inline(always)]
  pub const fn asset(&self) -> &A {
    &self.asset
  }

  /// Returns a mutable reference to the asset in the mapping.
  #[inline(always)]
  pub const fn asset_mut(&mut self) -> &mut A {
    &mut self.asset
  }

  /// Returns a reference to the value in the mapping.
  #[inline(always)]
  pub const fn to(&self) -> &T {
    &self.to
  }

  /// Returns a mutable reference to the value in the mapping.
  #[inline(always)]
  pub const fn to_mut(&mut self) -> &mut T {
    &mut self.to
  }

  /// Consumes the `AssetTo` instance and returns the asset component.
  #[inline(always)]
  pub fn into_asset(self) -> A {
    self.asset
  }

  /// Consumes the `AssetTo` instance and returns the value component.
  #[inline(always)]
  pub fn into_to(self) -> T {
    self.to
  }

  /// Consumes the `AssetTo` instance and returns its components as a tuple `(asset, to)`.
  #[inline(always)]
  pub fn into_parts(self) -> (A, T) {
    (self.asset, self.to)
  }
}

/// The data structure representing a perpetual DEX in the Hyperliquid Core API.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "strict-serde", serde(deny_unknown_fields))]
pub struct Dex {
  name: SmolStr,
  full_name: SmolStr,
  deployer: Address,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  oracle_updater: Option<Address>,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  fee_recipient: Option<Address>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  asset_to_streaming_oi_cap: Arc<[AssetToStreamingOiCap]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  sub_deployers: Arc<[SubDeployer]>,
  #[cfg_attr(
    feature = "serde",
    serde(skip_serializing_if = "Option::is_none", default)
  )]
  deployer_fee_scale: Option<SmolStr>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  asset_to_funding_multiplier: Arc<[AssetToFundingMultiplier]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  asset_to_funding_interest_rate: Arc<[AssetToFundingInterestRate]>,
  #[cfg_attr(feature = "serde", serde(with = "serde_arc_array"))]
  asset_to_funding_clamp: Arc<[AssetToFundingClamp]>,
}

impl Dex {
  /// Returns a reference to the name of the perpetual DEX.
  #[inline(always)]
  pub const fn name(&self) -> &SmolStr {
    &self.name
  }
  /// Returns a reference to the full name of the perpetual DEX.
  #[inline(always)]
  pub const fn full_name(&self) -> &SmolStr {
    &self.full_name
  }
  /// Returns a reference to the deployer of the perpetual DEX.
  #[inline(always)]
  pub const fn deployer(&self) -> &Address {
    &self.deployer
  }
  /// Returns a reference to the oracle updater of the perpetual DEX.
  #[inline(always)]
  pub const fn oracle_updater(&self) -> Option<&Address> {
    self.oracle_updater.as_ref()
  }
  /// Returns a reference to the fee recipient of the perpetual DEX.
  #[inline(always)]
  pub const fn fee_recipient(&self) -> Option<&Address> {
    self.fee_recipient.as_ref()
  }

  /// Returns a reference to the deployer fee scale of the perpetual DEX.
  #[inline(always)]
  pub const fn deployer_fee_scale(&self) -> Option<&SmolStr> {
    self.deployer_fee_scale.as_ref()
  }

  /// Returns a reference to the asset to streaming OI cap mapping of the perpetual DEX.
  #[inline(always)]
  pub const fn asset_to_streaming_oi_cap(&self) -> &Arc<[AssetToStreamingOiCap]> {
    &self.asset_to_streaming_oi_cap
  }

  /// Returns a reference to the sub deployers of the perpetual DEX.
  #[inline(always)]
  pub const fn sub_deployers(&self) -> &Arc<[SubDeployer]> {
    &self.sub_deployers
  }

  /// Returns a reference to the asset to funding multiplier mapping of the perpetual DEX.
  #[inline(always)]
  pub const fn asset_to_funding_multiplier(&self) -> &Arc<[AssetToFundingMultiplier]> {
    &self.asset_to_funding_multiplier
  }

  /// Returns a reference to the asset to funding interest rate mapping of the perpetual DEX.
  #[inline(always)]
  pub const fn asset_to_funding_interest_rate(&self) -> &Arc<[AssetToFundingInterestRate]> {
    &self.asset_to_funding_interest_rate
  }

  /// Returns a reference to the asset to funding clamp mapping of the perpetual DEX.
  #[inline(always)]
  pub const fn asset_to_funding_clamp(&self) -> &Arc<[AssetToFundingClamp]> {
    &self.asset_to_funding_clamp
  }

  /// Returns a reference to the asset to streaming OI cap mapping of the perpetual DEX.
  #[inline(always)]
  pub fn asset_to_streaming_oi_cap_ref(&self) -> &[AssetToStreamingOiCap] {
    self.asset_to_streaming_oi_cap.as_ref()
  }

  /// Returns a reference to the sub deployers of the perpetual DEX.
  #[inline(always)]
  pub fn sub_deployers_ref(&self) -> &[SubDeployer] {
    self.sub_deployers.as_ref()
  }

  /// Returns a reference to the asset to funding multiplier mapping of the perpetual DEX.
  #[inline(always)]
  pub fn asset_to_funding_multiplier_ref(&self) -> &[AssetToFundingMultiplier] {
    self.asset_to_funding_multiplier.as_ref()
  }
  /// Returns a reference to the asset to funding interest rate mapping of the perpetual DEX.
  #[inline(always)]
  pub fn asset_to_funding_interest_rate_ref(&self) -> &[AssetToFundingInterestRate] {
    self.asset_to_funding_interest_rate.as_ref()
  }
  /// Returns a reference to the asset to funding clamp mapping of the perpetual DEX.
  #[inline(always)]
  pub fn asset_to_funding_clamp_ref(&self) -> &[AssetToFundingClamp] {
    self.asset_to_funding_clamp.as_ref()
  }
}

#[cfg(feature = "serde")]
const _: () = {
  use serde::{
    Deserializer, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
  };

  impl<T, A> Serialize for AssetTo<T, A>
  where
    T: Serialize,
    A: Serialize,
  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      (&self.asset, &self.to).serialize(serializer)
    }
  }

  impl<'de, T, A> Deserialize<'de> for AssetTo<T, A>
  where
    T: Deserialize<'de>,
    A: Deserialize<'de>,
  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      <(A, T)>::deserialize(deserializer).map(|(asset, to)| Self::new(asset, to))
    }
  }

  impl Serialize for ResponseData<Vec<Dex>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut seq = serializer.serialize_seq(Some(self.len() + 1))?;
      seq.serialize_element(&Option::<Dex>::None)?;
      for dex in &self.0 {
        seq.serialize_element(&Some(dex))?;
      }
      seq.end()
    }
  }

  struct DexsVisitor;

  impl<'de> Visitor<'de> for DexsVisitor {
    type Value = Vec<Dex>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      formatter.write_str("a sequence of optional Dex elements")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
      A: SeqAccess<'de>,
    {
      let mut dexes = Vec::new();

      match seq.next_element::<Option<Dex>>()? {
        Some(Some(_)) => return Err(de::Error::custom("first element of Dexs must be null")),
        Some(None) => {}
        None => return Err(de::Error::custom("`Dexs` cannot be empty")),
      }

      while let Some(dex) = seq.next_element()? {
        dexes.push(dex);
      }
      Ok(dexes)
    }
  }

  impl<'de> Deserialize<'de> for ResponseData<Vec<Dex>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_seq(DexsVisitor).map(ResponseData)
    }
  }
};
