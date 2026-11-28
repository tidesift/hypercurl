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

  /// Returns a mutable reference to the URL of the client instance.
  #[inline(always)]
  pub const fn url_mut(&mut self) -> &mut Url {
    &mut self.url
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
  pub async fn meta(&self) -> Result<Response<derivatives::meta::Meta>> {
    self.call(META_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the meta and asset contexts.
  pub async fn meta_and_asset_ctxs(
    &self,
  ) -> Result<Response<derivatives::meta::MetaAndAssetContexts>> {
    self.call(META_AND_ASSET_CTXS_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the meta and asset contexts.
  pub async fn meta_and_asset_ctxs_with_dex<D>(
    &self,
    dex: D,
  ) -> Result<Response<Option<derivatives::meta::MetaAndAssetContexts>>>
  where
    D: AsRef<str>,
  {
    self
      .call(format!(
        "{{\"type\":\"metaAndAssetCtxs\",\"dex\":\"{}\"}}",
        dex.as_ref()
      ))
      .await
  }

  /// Sends a request to retrieve the perpetual DEXs.
  pub async fn perp_dexs(&self) -> Result<Response<Vec<derivatives::perp_dex::Dex>>> {
    self.call(PERP_DEXS_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the spot metadata.
  pub async fn spot_meta(&self) -> Result<Response<spot::meta::Meta>> {
    self.call(SPOT_META_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the spot metadata and asset contexts.
  pub async fn spot_meta_and_asset_ctxs(
    &self,
  ) -> Result<Response<spot::meta::MetaAndAssetContexts>> {
    self.call(SPOT_META_AND_ASSET_CTXS_REQUEST_BODY).await
  }

  /// Sends a request to retrieve the perps pair deploy auction status.
  pub async fn perps_pair_deploy_auction_status(
    &self,
  ) -> Result<Response<derivatives::PairDeployAuctionStatus>> {
    self
      .call(PERPS_PAIR_DEPLOY_AUCTION_STATUS_REQUEST_BODY)
      .await
  }

  /// Sends a request to retrieve the spot pair deploy auction status.
  pub async fn spot_pair_deploy_auction_status(
    &self,
  ) -> Result<Response<spot::PairDeployAuctionStatus>> {
    self
      .call(SPOT_PAIR_DEPLOY_AUCTION_STATUS_REQUEST_BODY)
      .await
  }
}

impl<E, Url> AsyncClient<E, Url> {
  #[inline(always)]
  async fn call<O>(&self, body: impl Into<Body>) -> Result<Response<O>>
  where
    ResponseData<O>: for<'de> serde::Deserialize<'de>,
    Url: IntoUrl + Clone,
  {
    use core::mem::take;

    let mut resp = self
      .client
      .post(self.url.clone())
      .header("content-type", "application/json")
      .body(body)
      .send()
      .await?;

    let headers = take(resp.headers_mut());
    let extensions = take(resp.extensions_mut());
    let version = resp.version();
    let status = resp.status();
    let remote_addr = resp.remote_addr();

    let data = resp.json::<ResponseData<O>>().await?;

    let mut ret = http::Response::new(data);
    *ret.headers_mut() = headers;
    *ret.version_mut() = version;
    *ret.status_mut() = status;
    *ret.extensions_mut() = extensions;

    Ok(Response::new(ret).with_remote_addr(remote_addr))
  }
}
