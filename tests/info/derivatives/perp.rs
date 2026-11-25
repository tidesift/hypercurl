use hypercurl::http::AsyncClient;

async fn perp_dexs(client: AsyncClient) {
  let _ = client.info().perp_dexs().await.unwrap();
}

#[tokio::test]
async fn perp_dexs_mainnet() {
  perp_dexs(AsyncClient::mainnet()).await;
}

#[tokio::test]
async fn perp_dexs_testnet() {
  perp_dexs(AsyncClient::testnet()).await;
}

#[test]
fn t() {
  use crate::TESTNET_PERP_DEXS_JSON;
  use hypercurl::info::derivatives::perp_dex::Dexs;

  let res = serde_json::from_str::<Dexs>(TESTNET_PERP_DEXS_JSON).unwrap();
}
