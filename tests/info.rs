#[path = "info/spot/mod.rs"]
mod spot;

#[path = "info/derivatives/mod.rs"]
mod derivatives;

const TESTNET_PERP_DEXS_JSON: &str = include_str!("fixtures/testnet/perp_dexs.json");
