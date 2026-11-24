use hypercurl::http::AsyncClient;

async fn spot_meta(client: AsyncClient) {
  let _ = client.info().spot_meta().await.unwrap();
}

#[tokio::test]
async fn spot_meta_mainnet() {
  spot_meta(AsyncClient::mainnet()).await;
}

#[tokio::test]
async fn spot_meta_testnet() {
  spot_meta(AsyncClient::testnet()).await;
}

async fn spot_meta_and_asset_contexts(client: AsyncClient) {
  let _ = client.info().spot_meta_and_asset_ctxs().await.unwrap();
}

#[tokio::test]
async fn spot_meta_and_asset_contexts_mainnet() {
  spot_meta_and_asset_contexts(AsyncClient::mainnet()).await;
}

#[tokio::test]
async fn spot_meta_and_asset_contexts_testnet() {
  spot_meta_and_asset_contexts(AsyncClient::testnet()).await;
}
