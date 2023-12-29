use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use bitcoin::{Address, Network};
use fatcrab_trading::order::FatCrabOrderType;
use secp256k1::SecretKey;

use fatcrab_trading::trader::FatCrabTrader as InnerTrader;
use url::Url;

use crate::error::FatCrabError;
use crate::maker::{FatCrabBuyMaker, FatCrabSellMaker};
use crate::order::{FatCrabOrder, FatCrabOrderEnvelope};
use crate::taker::{FatCrabBuyTaker, FatCrabSellTaker};
use crate::types::{BlockchainInfo, RelayInfo};
use crate::RUNTIME;

pub struct FatCrabTrader {
    inner: InnerTrader,
    network: Network,
}

impl FatCrabTrader {
    pub fn new(info: BlockchainInfo, app_dir_path: String) -> Self {
        let network = match &info {
            BlockchainInfo::Electrum { network, .. } => network.to_owned(),
            BlockchainInfo::Rpc { network, .. } => network.to_owned(),
        };
        let inner = RUNTIME.block_on(async { InnerTrader::new(info.into(), app_dir_path).await });
        Self {
            inner,
            network: network.into(),
        }
    }

    pub fn new_with_keys(key: String, info: BlockchainInfo, app_dir_path: String) -> Self {
        let network = match &info {
            BlockchainInfo::Electrum { network, .. } => network.to_owned(),
            BlockchainInfo::Rpc { network, .. } => network.to_owned(),
        };
        let secret_key = SecretKey::from_str(&key).unwrap();
        let inner = RUNTIME.block_on(async {
            InnerTrader::new_with_key(secret_key, info.into(), app_dir_path).await
        });
        Self {
            inner,
            network: network.into(),
        }
    }

    pub fn wallet_bip39_mnemonic(&self) -> Result<String, FatCrabError> {
        let result = RUNTIME.block_on(async { self.inner.wallet_bip39_mnemonic().await });
        match result {
            Ok(mnemonic) => Ok(mnemonic.to_string()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn wallet_spendable_balance(&self) -> Result<u64, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.wallet_spendable_balance().await })
            .map_err(|e| e.into())
    }

    pub fn wallet_allocated_amount(&self) -> Result<u64, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.wallet_allocated_amount().await })
            .map_err(|e| e.into())
    }

    pub fn wallet_generate_receive_address(&self) -> Result<String, FatCrabError> {
        let result = RUNTIME.block_on(async { self.inner.wallet_generate_receive_address().await });
        match result {
            Ok(address) => Ok(address.to_string()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn wallet_send_to_address(
        &self,
        address: String,
        amount: u64,
    ) -> Result<String, FatCrabError> {
        let address = Address::from_str(&address).unwrap();
        let address = address.require_network(self.network).unwrap();
        let result =
            RUNTIME.block_on(async { self.inner.wallet_send_to_address(address, amount).await });
        match result {
            Ok(txid) => Ok(txid.to_string()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn wallet_blockchain_sync(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.wallet_blockchain_sync().await })
            .map_err(|e| e.into())
    }

    pub fn nostr_pubkey(&self) -> String {
        RUNTIME
            .block_on(async { self.inner.nostr_pubkey().await })
            .to_string()
    }

    pub fn add_relays(&self, relays_info: Vec<RelayInfo>) -> Result<(), FatCrabError> {
        let mut relays = Vec::new();

        for relay_info in relays_info {
            let socket = match relay_info.socket_addr {
                Some(socket_str) => SocketAddr::from_str(&socket_str).ok(),
                None => None,
            };
            let url = Url::parse(&relay_info.addr)?;
            relays.push((url, socket));
        }

        RUNTIME
            .block_on(async { self.inner.add_relays(relays).await })
            .map_err(|e| e.into())
    }

    pub fn make_buy_order(
        &self,
        order: FatCrabOrder,
        fatcrab_rx_addr: String,
    ) -> Arc<FatCrabBuyMaker> {
        let maker_access = RUNTIME.block_on(async {
            self.inner
                .make_buy_order(&order.into(), fatcrab_rx_addr)
                .await
        });
        Arc::new(FatCrabBuyMaker::new(maker_access))
    }

    pub fn make_sell_order(&self, order: FatCrabOrder) -> Arc<FatCrabSellMaker> {
        let maker_access =
            RUNTIME.block_on(async { self.inner.make_sell_order(&order.into()).await });
        Arc::new(FatCrabSellMaker::new(maker_access))
    }

    pub fn query_orders(
        &self,
        order_type: Option<FatCrabOrderType>,
    ) -> Result<Vec<Arc<FatCrabOrderEnvelope>>, FatCrabError> {
        match RUNTIME.block_on(async { self.inner.query_orders(order_type).await }) {
            Ok(order_envelopes) => Ok(order_envelopes
                .into_iter()
                .map(|order_envelope| Arc::new(order_envelope.into()))
                .collect()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn take_buy_order(
        &self,
        order_envelope: Arc<FatCrabOrderEnvelope>,
    ) -> Arc<FatCrabBuyTaker> {
        let order_envelope = order_envelope.as_ref().clone();
        let taker_access =
            RUNTIME.block_on(async { self.inner.take_buy_order(&order_envelope.into()).await });
        Arc::new(FatCrabBuyTaker::new(taker_access))
    }

    pub fn take_sell_order(
        &self,
        order_envelope: Arc<FatCrabOrderEnvelope>,
        fatcrab_rx_addr: String,
    ) -> Arc<FatCrabSellTaker> {
        let order_envelope = order_envelope.as_ref().clone();
        let taker_access: fatcrab_trading::taker::FatCrabTakerAccess<
            fatcrab_trading::taker::TakerSell,
        > = RUNTIME.block_on(async {
            self.inner
                .take_sell_order(&order_envelope.into(), fatcrab_rx_addr)
                .await
        });
        Arc::new(FatCrabSellTaker::new(taker_access))
    }

    pub fn shutdown(&self) -> Result<(), FatCrabError> {
        // TODO: FFI requires shared reference to self as argument
        // But Trader Shutdown is a self consuming method
        // Not sure how to do this at this point. Ommitting for now
        // RUNTIME.block_on(async { self.inner.shutdown().await }).map_err(|e| e.into())
        Ok(())
    }
}
