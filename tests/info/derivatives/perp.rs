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
