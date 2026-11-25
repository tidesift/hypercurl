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

async fn meta_and_asset_contexts_with_dex(client: AsyncClient, dex: &str, exist: bool) {
  let result = client
    .info()
    .meta_and_asset_ctxs_with_dex(dex)
    .await
    .unwrap();
  assert_eq!(result.body().is_some(), exist);
}

#[tokio::test]
async fn meta_and_asset_contexts_mainnet() {
  meta_and_asset_contexts(AsyncClient::mainnet()).await;
}

#[tokio::test]
async fn meta_and_asset_contexts_testnet() {
  meta_and_asset_contexts(AsyncClient::testnet()).await;
}

#[tokio::test]
async fn meta_and_asset_contexts_with_dex_mainnet_nonexistent() {
  meta_and_asset_contexts_with_dex(
    AsyncClient::mainnet(),
    "I do not believe this can be a valid dex",
    false,
  )
  .await;
}

#[tokio::test]
async fn meta_and_asset_contexts_with_dex_testnet_nonexistent() {
  meta_and_asset_contexts_with_dex(
    AsyncClient::testnet(),
    "I do not believe this can be a valid dex",
    false,
  )
  .await;
}

#[tokio::test]
async fn meta_and_asset_contexts_with_dex_mainnet_existent() {
  meta_and_asset_contexts_with_dex(AsyncClient::mainnet(), "xyz", true).await;
}

#[tokio::test]
async fn meta_and_asset_contexts_with_dex_testnet_existent() {
  meta_and_asset_contexts_with_dex(AsyncClient::testnet(), "xyz", true).await;
}
