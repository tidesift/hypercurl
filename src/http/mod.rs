use core::net::SocketAddr;

use crate::info::{derivatives, spot};
use derive_more::{AsMut, AsRef, Deref, DerefMut};

#[cfg(feature = "reqwest")]
pub use async_client::AsyncClient;

#[cfg(feature = "reqwest")]
mod async_client;

/// The official information endpoint URL.
pub const OFFICIAL_INFORMATION_ENDPOINT: &str = "https://api.hyperliquid.xyz/info";

/// The official information endpoint URL.
pub const OFFICIAL_INFORMATION_TESTNET_ENDPOINT: &str = "https://api.hyperliquid-testnet.xyz/info";

/// The body of the spot metadata request.
pub const SPOT_META_REQUEST_BODY: &str = r#"{"type":"spotMeta"}"#;
/// The body of the spot meta and asset contexts request.
pub const SPOT_META_AND_ASSET_CTXS_REQUEST_BODY: &str = r#"{"type":"spotMetaAndAssetCtxs"}"#;
/// The body of the meta request.
pub const META_REQUEST_BODY: &str = r#"{"type":"meta"}"#;
/// The body of the meta and asset contexts request.
pub const META_AND_ASSET_CTXS_REQUEST_BODY: &str = r#"{"type":"metaAndAssetCtxs"}"#;
/// The body of the perpetual DEXs request.
pub const PERP_DEXS_REQUEST_BODY: &str = r#"{"type":"perpDexs"}"#;
/// The body of the spot pair deploy auction status request.
pub const SPOT_PAIR_DEPLOY_AUCTION_STATUS_REQUEST_BODY: &str =
  r#"{"type":"spotPairDeployAuctionStatus"}"#;
/// The body of the perps pair deploy auction status request.
pub const PERPS_PAIR_DEPLOY_AUCTION_STATUS_REQUEST_BODY: &str =
  r#"{"type":"perpDeployAuctionStatus"}"#;

mod sealed {
  pub trait Endpoint {}
}

/// The information endpoint
pub struct Info(());

/// The exchanges endpoint
pub struct Exchanges(());

impl sealed::Endpoint for Info {}
impl sealed::Endpoint for Exchanges {}
impl sealed::Endpoint for () {}

/// The response data wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRef, AsMut, Deref, DerefMut)]
#[repr(transparent)]
pub struct ResponseData<D: ?Sized>(pub(crate) D);

impl<D> From<D> for ResponseData<D> {
  #[inline(always)]
  fn from(data: D) -> Self {
    ResponseData(data)
  }
}

impl<D> ResponseData<D> {
  /// Consumes the `ResponseData` and returns the inner data.
  #[inline(always)]
  pub fn into_inner(self) -> D {
    self.0
  }

  /// Returns a reference to the inner data.
  #[inline(always)]
  pub const fn as_inner(&self) -> &D {
    &self.0
  }

  /// Returns a mutable reference to the inner data.
  #[inline(always)]
  pub const fn as_inner_mut(&mut self) -> &mut D {
    &mut self.0
  }

  /// Casts a reference to the inner data to a reference to `ResponseData`.
  #[inline(always)]
  pub const fn ref_cast(data: &D) -> &Self {
    // SAFETY: `ResponseData` is `#[repr(transparent)]`
    unsafe { &*(data as *const D as *const Self) }
  }
}

/// The HTTP response
#[derive(Debug, Clone)]
pub struct Response<D> {
  resp: http::Response<ResponseData<D>>,
  remote_addr: Option<SocketAddr>,
}

impl<D> From<Response<D>> for http::Response<ResponseData<D>> {
  #[inline(always)]
  fn from(resp: Response<D>) -> Self {
    resp.resp
  }
}

impl<D> core::ops::Deref for Response<D> {
  type Target = http::Response<ResponseData<D>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.resp
  }
}

impl<D> core::ops::DerefMut for Response<D> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.resp
  }
}

impl<D> AsRef<http::Response<ResponseData<D>>> for Response<D> {
  #[inline(always)]
  fn as_ref(&self) -> &http::Response<ResponseData<D>> {
    &self.resp
  }
}

impl<D> AsMut<http::Response<ResponseData<D>>> for Response<D> {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut http::Response<ResponseData<D>> {
    &mut self.resp
  }
}

impl<D> AsRef<D> for Response<D> {
  #[inline(always)]
  fn as_ref(&self) -> &D {
    self.resp.body().as_inner()
  }
}

impl<D> AsMut<D> for Response<D> {
  #[inline(always)]
  fn as_mut(&mut self) -> &mut D {
    self.resp.body_mut().as_inner_mut()
  }
}

impl<D> Response<D> {
  #[inline(always)]
  pub(crate) const fn new(resp: http::Response<ResponseData<D>>) -> Self {
    Self {
      remote_addr: None,
      resp,
    }
  }

  /// Returns the remote address of the HTTP response, if available.
  #[inline(always)]
  pub const fn remote_addr(&self) -> Option<SocketAddr> {
    self.remote_addr
  }

  #[inline(always)]
  pub(crate) const fn with_remote_addr(mut self, remote_addr: Option<SocketAddr>) -> Self {
    self.remote_addr = remote_addr;
    self
  }
}

#[cfg(feature = "serde")]
const _: () = {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  impl<T> Serialize for ResponseData<Option<T>>
  where
    ResponseData<T>: Serialize,
  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      match self.as_inner() {
        Some(inner) => ResponseData::ref_cast(inner).serialize(serializer),
        None => serializer.serialize_none(),
      }
    }
  }

  impl<'de, T> Deserialize<'de> for ResponseData<Option<T>>
  where
    ResponseData<T>: Deserialize<'de>,
  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let opt = Option::<ResponseData<T>>::deserialize(deserializer)?;
      Ok(opt.map(|inner| inner.into_inner()).into())
    }
  }
};

macro_rules! serde_response_data {
  (@forward $($ty:ty),+$(,)?) => {
    $(
      #[cfg(feature = "serde")]
      const _: () = {
        impl<'de> ::serde::Deserialize<'de> for $crate::http::ResponseData<$ty> {
          fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
          where
            D: ::serde::Deserializer<'de>,
          {
            ::serde::Deserialize::deserialize(deserializer).map($crate::http::ResponseData)
          }
        }

        impl ::serde::Serialize for $crate::http::ResponseData<$ty> {
          fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
          where
            S: ::serde::Serializer,
          {
            ::serde::Serialize::serialize(&self.0, serializer)
          }
        }
      };
    )*
  };
}

pub(crate) use serde_response_data;
