use hypercurl::http::AsyncClient;

mod meta;

#[tokio::test]
async fn spot_pair_deploy_auction_status_mainnet() {
  AsyncClient::mainnet()
    .info()
    .spot_pair_deploy_auction_status()
    .await
    .unwrap();
}

#[tokio::test]
async fn spot_pair_deploy_auction_status_testnet() {
  AsyncClient::testnet()
    .info()
    .spot_pair_deploy_auction_status()
    .await
    .unwrap();
}
