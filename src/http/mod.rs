use crate::info::{derivatives, spot};

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
