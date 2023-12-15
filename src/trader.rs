
use std::str::FromStr;

use secp256k1::SecretKey;

use fatcrab_trading::trader::FatCrabTrader as InnerTrader;

use crate::types::{Auth, Network};
use crate::RUNTIME;

pub struct Trader {
    inner: InnerTrader,
}

impl Trader {
    pub fn new(url: String, auth: Auth, network: Network) -> Self {
        let inner = RUNTIME.block_on(async { InnerTrader::new(url, auth.into(), network.into()).await });
        Self { inner }
    }

    pub fn new_with_keys(key: String, url: String, auth: Auth, network: Network) -> Self {
        let secret_key = SecretKey::from_str(&key).unwrap();
        let inner = RUNTIME.block_on(async {
            InnerTrader::new_with_keys(secret_key, url, auth.into(), network.into()).await
        });
        Self { inner }
    }

    // pub async fn wallet_bip39_mnemonic(&self) -> Result<Mnemonic, FatCrabError> {
    //     self.inner.wallet_bip39_mnemonic().await
    // }

    // pub async fn wallet_spendable_balance(&self) -> Result<u64, FatCrabError> {
    //     self.inner.wallet_spendable_balance().await
    // }

    // pub async fn wallet_generate_receive_address(&self) -> Result<Address, FatCrabError> {
    //     self.inner.wallet_generate_receive_address().await
    // }

    // pub async fn wallet_send_to_address(
    //     &self,
    //     address: Address,
    //     amount: u64,
    // ) -> Result<TransactionId, FatCrabError> {
    //     self.inner.wallet_send_to_address(address, amount).await
    // }

    // pub async fn wallet_blockchain_sync(&self) -> Result<(), FatCrabError> {
    //     self.inner.wallet_blockchain_sync().await
    // }

    // pub async fn nostr_pubkey(&self) -> XOnlyPublicKey {
    //     self.inner.nostr_pubkey().await
    // }

    // pub async fn add_relays(
    //     &self,
    //     relays: Vec<(String, Option<SocketAddr>)>,
    // ) -> Result<(), FatCrabError> {
    //     self.inner.add_relays(relays).await
    // }

    // pub async fn make_buy_order(
    //     &self,
    //     order: FatCrabOrder,
    //     fatcrab_rx_addr: String,
    // ) -> FatCrabMakerAccess<MakerBuy> {
    //     self.inner
    //         .make_buy_order(order, fatcrab_rx_addr)
    //         .await
    //         .unwrap()
    // }

    // pub async fn make_sell_order(&self, order: FatCrabOrder) -> FatCrabMakerAccess<MakerSell> {
    //     self.inner.make_sell_order(order).await.unwrap()
    // }

    // pub async fn query_orders(
    //     &self,
    //     order_type: FatCrabOrderType,
    // ) -> Result<Vec<FatCrabOrderEnvelope>, FatCrabError> {
    //     self.inner.query_orders(order_type).await
    // }

    // pub async fn take_buy_order(
    //     &self,
    //     order_envelope: FatCrabOrderEnvelope,
    // ) -> FatCrabTakerAccess<TakerBuy> {
    //     self.inner.take_buy_order(order_envelope).await.unwrap()
    // }

    // pub async fn take_sell_order(
    //     &self,
    //     order_envelope: FatCrabOrderEnvelope,
    //     fatcrab_rx_addr: String,
    // ) -> FatCrabTakerAccess<TakerSell> {
    //     self.inner
    //         .take_sell_order(order_envelope, fatcrab_rx_addr)
    //         .await
    //         .unwrap()
    // }

    // pub async fn shutdown(self) -> Result<(), JoinError> {
    //     self.inner.shutdown().await
    // }
}
