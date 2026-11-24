use core::marker::PhantomData;

use reqwest::{Body, Client as HttpClient, IntoUrl};

use super::*;

type Result<T> = core::result::Result<T, reqwest::Error>;

/// A generic HTTP client for interacting with different endpoints.
#[derive(Debug, Clone)]
pub struct AsyncClient<E = (), Url = &'static str> {
  client: HttpClient,
  url: Url,
  _m: PhantomData<E>,
}

impl Default for AsyncClient {
  #[inline(always)]
  fn default() -> Self {
    Self::mainnet()
  }
}

impl AsyncClient {
  /// Returns a new instance of the HTTP client.
  #[inline(always)]
  pub fn mainnet() -> Self {
    Self {
      client: HttpClient::new(),
      url: OFFICIAL_INFORMATION_ENDPOINT,
      _m: PhantomData,
    }
  }

  /// Returns a new instance of the HTTP client.
  #[inline(always)]
  pub fn testnet() -> Self {
    Self {
      client: HttpClient::new(),
      url: OFFICIAL_INFORMATION_TESTNET_ENDPOINT,
      _m: PhantomData,
    }
  }

  /// Creates a new `AsyncClient` instance with the specified `HttpClient`.
  #[inline(always)]
  pub const fn with_client(client: HttpClient) -> Self {
    Self {
      client,
      url: OFFICIAL_INFORMATION_ENDPOINT,
      _m: PhantomData,
    }
  }
}

impl<Url> AsyncClient<(), Url> {
  /// Creates a new `AsyncClient` instance with the specified URL.
  #[inline(always)]
  pub fn with_url(url: Url) -> Self {
    Self {
      client: HttpClient::new(),
      url,
      _m: PhantomData,
    }
  }
}

impl<E, Url> AsyncClient<E, Url> {
  /// Creates a new `AsyncClient` instance with the specified `HttpClient` and URL.
  #[inline(always)]
  pub const fn with_client_and_url(client: HttpClient, url: Url) -> Self {
    Self {
      client,
      url,
      _m: PhantomData,
    }
  }

  /// Sets the URL for the client instance.
  #[inline(always)]
  pub fn set_url(&mut self, url: Url) -> &mut Self {
    self.url = url;
    self
  }

  /// Sets the HTTP client for the client instance.
  #[inline(always)]
  pub fn set_client(&mut self, client: HttpClient) -> &mut Self {
    self.client = client;
    self
  }

  /// Returns the Url of the client instance.
  #[inline(always)]
  pub const fn url(&self) -> &Url {
    &self.url
  }
}

impl<E, Url> AsyncClient<E, Url>
where
  E: sealed::Endpoint,
{
  /// Returns a new client instance configured for the `Info` endpoint.
  pub fn info(self) -> AsyncClient<Info, Url> {
    AsyncClient {
      client: self.client,
      url: self.url,
      _m: PhantomData,
    }
  }

  /// Returns a new client instance configured for the `Exchanges` endpoint.
  pub fn exchanges(self) -> AsyncClient<Exchanges, Url> {
    AsyncClient {
      client: self.client,
      url: self.url,
      _m: PhantomData,
    }
  }
}

impl<Url> AsyncClient<Info, Url>
where
  Url: IntoUrl + Clone,
{
  /// Sends a meta request to retrieve the spot metadata.
  pub async fn meta(&self) -> Result<derivatives::meta::Meta> {
    self.request(META_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the meta and asset contexts.
  pub async fn meta_and_asset_ctxs(&self) -> Result<derivatives::meta::MetaAndAssetContexts> {
    self.request(META_AND_ASSET_CTXS_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the spot metadata.
  pub async fn spot_meta(&self) -> Result<spot::meta::Meta> {
    self.request(SPOT_META_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the spot metadata and asset contexts.
  pub async fn spot_meta_and_asset_ctxs(&self) -> Result<spot::meta::MetaAndAssetContexts> {
    self.request(SPOT_META_AND_ASSET_CTXS_REQUEST_BODY).await
  }
}

impl<E, Url> AsyncClient<E, Url> {
  #[inline(always)]
  async fn request<O>(&self, body: impl Into<Body>) -> Result<O>
  where
    O: for<'de> serde::Deserialize<'de>,
    Url: IntoUrl + Clone,
  {
    self
      .client
      .post(self.url.clone())
      .header("content-type", "application/json")
      .body(body)
      .send()
      .await?
      .json()
      .await
  }
}
