/// The metadata data structures for spots of the Hyperliquid Core API.
pub mod meta;

pub use super::pair_deploy_auction_status::PairDeployAuctionStatus;

/// The official default endpoint for the Hyperliquid Core info API.
pub const DEFAULT_ENDPOINT: &str = "https://api.hyperliquid.xyz/info";
