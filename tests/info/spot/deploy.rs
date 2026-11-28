use alloy_primitives::Address;
use hypercurl::http::AsyncClient;

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

async fn spot_deploy_state(client: AsyncClient, user: Address) {
  client.info().spot_deploy_state(user).await.unwrap();
}

fn user_addresses() -> impl Iterator<Item = Address> {
  [
    "0x0000000000000000000000000000000000000000",
    "0x051dbfc562d44e4a01ebb986da35a47ab4f346db",
    "0x1234567890abcdef1234567890abcdef12345678",
  ]
  .iter()
  .map(|s| s.parse().unwrap())
}

#[tokio::test]
async fn spot_deploy_state_mainnet() {
  for user in user_addresses() {
    spot_deploy_state(AsyncClient::mainnet(), user).await;
  }
}

#[tokio::test]
async fn spot_deploy_state_testnet() {
  for user in user_addresses() {
    spot_deploy_state(AsyncClient::testnet(), user).await;
  }
}
