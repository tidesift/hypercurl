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
    "0x563C175E6f11582f65D6d9E360A618699DEe14a9",
    "0x1defed46db35334232b9f5fd2e5c6180276fb99d",
    "0x8c967e73e7b15087c42a10d344cff4c96d877f1d",
    "0x6b043579b088d44400dffa1ef1c5e3e3bfbdf9d2",
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

async fn spot_clearinghouse_state(client: AsyncClient, user: Address) {
  client.info().spot_clearinghouse_state(user).await.unwrap();
}

#[tokio::test]
async fn spot_clearinghouse_state_mainnet() {
  for user in user_addresses() {
    spot_clearinghouse_state(AsyncClient::mainnet(), user).await;
  }
}

#[tokio::test]
async fn spot_clearinghouse_state_testnet() {
  for user in user_addresses() {
    spot_clearinghouse_state(AsyncClient::testnet(), user).await;
  }
}
