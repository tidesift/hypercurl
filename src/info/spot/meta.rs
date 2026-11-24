use core::mem::take;
use std::vec::Vec;

use alloy_primitives::B128;
use derive_more::{From, IsVariant, TryUnwrap, Unwrap};
use smol_str::SmolStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The default body of the spot metadata request.
pub const DEFAULT_META_REQUEST_BODY: &str = r#"{"type":"spotMeta"}"#;

/// The default body of the spot meta and asset contexts request.
pub const DEFAULT_META_AND_ASSET_CTXS_REQUEST_BODY: &str = r#"{"type":"spotMetaAndAssetCtxs"}"#;

const CONTENT_TYPE: &str = r#"application/json"#;

/// The metadata for a spot asset in the hyperliquid core API.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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

/// The token information for a spot.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TokenInfo {
  name: SmolStr,
  sz_decimals: u8,
  wei_decimals: u8,
  index: usize,
  token_id: B128,
  is_canonical: bool,
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
}

/// The metadata associated with a spot.
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

/// Represents either spot metadata or asset contexts.
#[derive(Debug, Clone, IsVariant, Unwrap, TryUnwrap, From)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MetaAndAssetContexts {
  /// Represents spot metadata.
  Meta(Meta),
  /// Represents asset contexts.
  Context(Vec<AssetContext>),
}

/// Represents a request for spot metadata.
pub struct MetaRequest<Endpoint = &'static str> {
  endpoint: Endpoint,
}

impl Default for MetaRequest<&'static str> {
  #[inline(always)]
  fn default() -> Self {
    Self {
      endpoint: super::DEFAULT_ENDPOINT,
    }
  }
}

impl<E> MetaRequest<E> {
  /// Creates a new `MetaRequest` with the specified endpoint.
  #[inline(always)]
  pub const fn new(endpoint: E) -> Self {
    Self { endpoint }
  }

  /// Returns the endpoint of the request.
  #[inline(always)]
  pub const fn endpoint(&self) -> &E {
    &self.endpoint
  }

  /// Returns a mutable reference to the endpoint of the request.
  #[inline(always)]
  pub const fn endpoint_mut(&mut self) -> &mut E {
    &mut self.endpoint
  }
}

/// Represents a request for spot metadata and asset contexts.
pub struct MetaAndAssetContextsRequest<Endpoint = &'static str> {
  endpoint: Endpoint,
}

impl Default for MetaAndAssetContextsRequest<&'static str> {
  #[inline(always)]
  fn default() -> Self {
    Self {
      endpoint: super::DEFAULT_ENDPOINT,
    }
  }
}

impl<E> MetaAndAssetContextsRequest<E> {
  /// Creates a new `MetaAndAssetContextsRequest` with the specified endpoint.
  #[inline(always)]
  pub const fn new(endpoint: E) -> Self {
    Self { endpoint }
  }

  /// Returns the endpoint of the request.
  #[inline(always)]
  pub const fn endpoint(&self) -> &E {
    &self.endpoint
  }

  /// Returns a mutable reference to the endpoint of the request.
  #[inline(always)]
  pub const fn endpoint_mut(&mut self) -> &mut E {
    &mut self.endpoint
  }
}

#[cfg(feature = "reqwest")]
const _: () = {
  use reqwest::{Client, Error, IntoUrl};

  impl<E> MetaRequest<E>
  where
    E: IntoUrl,
  {
    /// Sends the request using `reqwest` and returns the response.
    pub async fn reqwest(self) -> Result<Meta, Error> {
      Client::new()
        .post(self.endpoint)
        .header("content-type", CONTENT_TYPE)
        .body(DEFAULT_META_REQUEST_BODY)
        .send()
        .await?
        .json::<Meta>()
        .await
    }
  }

  impl<E> MetaAndAssetContextsRequest<E>
  where
    E: IntoUrl,
  {
    /// Sends the request using `reqwest` and returns the response.
    pub async fn reqwest(self) -> Result<Vec<MetaAndAssetContexts>, Error> {
      Client::new()
        .post(self.endpoint)
        .header("content-type", CONTENT_TYPE)
        .body(DEFAULT_META_AND_ASSET_CTXS_REQUEST_BODY)
        .send()
        .await?
        .json::<Vec<MetaAndAssetContexts>>()
        .await
    }
  }
};
