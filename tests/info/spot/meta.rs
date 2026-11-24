use hypercurl::info::spot::meta::*;

#[tokio::test]
async fn spot_meta() {
  let meta = MetaRequest::default().reqwest().await.unwrap();
  assert!(!meta.tokens().is_empty());
  assert!(!meta.universe().is_empty());
}

#[tokio::test]
async fn spot_meta_and_asset_contexts() {
  let meta = MetaAndAssetContextsRequest::default()
    .reqwest()
    .await
    .unwrap();

  for item in &meta {
    match item {
      MetaAndAssetContexts::Meta(meta) => {
        assert!(!meta.tokens().is_empty());
        assert!(!meta.universe().is_empty());
      }
      MetaAndAssetContexts::Context(ctxs) => {
        assert!(!ctxs.is_empty());
      }
    }
  }
}
