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

#[tokio::test]
async fn perps_pair_deploy_auction_status_mainnet() {
  AsyncClient::mainnet()
    .info()
    .perps_pair_deploy_auction_status()
    .await
    .unwrap();
}

#[tokio::test]
async fn perps_pair_deploy_auction_status_testnet() {
  AsyncClient::testnet()
    .info()
    .perps_pair_deploy_auction_status()
    .await
    .unwrap();
}
