use hypercurl::http::AsyncClient;

async fn meta(client: AsyncClient) {
  let _ = client.info().meta().await.unwrap();
}

#[tokio::test]
async fn meta_mainnet() {
  meta(AsyncClient::mainnet()).await;
}

#[tokio::test]
async fn meta_testnet() {
  meta(AsyncClient::testnet()).await;
}

async fn meta_and_asset_contexts(client: AsyncClient) {
  let _ = client.info().meta_and_asset_ctxs().await.unwrap();
}

#[tokio::test]
async fn meta_and_asset_contexts_mainnet() {
  meta_and_asset_contexts(AsyncClient::mainnet()).await;
}

#[tokio::test]
async fn meta_and_asset_contexts_testnet() {
  meta_and_asset_contexts(AsyncClient::testnet()).await;
}
