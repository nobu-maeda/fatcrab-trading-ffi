use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use bitcoin::{Address, Network};
use secp256k1::SecretKey;
use url::Url;

pub use fatcrab_trading::common::Balances;
use fatcrab_trading::{order::FatCrabOrderType, trader::FatCrabTrader as InnerTrader};

use crate::error::FatCrabError;
use crate::maker::{FatCrabBuyMaker, FatCrabSellMaker};
use crate::order::{FatCrabOrder, FatCrabOrderEnvelope};
use crate::taker::{FatCrabBuyTaker, FatCrabSellTaker};
use crate::types::{BlockchainInfo, ProductionLevel, RelayAddr, RelayInfo};
use crate::RUNTIME;

pub struct FatCrabTrader {
    inner: InnerTrader,
    network: Network,
}

impl FatCrabTrader {
    pub fn new(prod_lvl: ProductionLevel, info: BlockchainInfo, app_dir_path: String) -> Self {
        let network = match &info {
            BlockchainInfo::Electrum { network, .. } => network.to_owned(),
            BlockchainInfo::Rpc { network, .. } => network.to_owned(),
        };
        let inner =
            RUNTIME.block_on(async { InnerTrader::new(prod_lvl, info.into(), app_dir_path).await });
        Self {
            inner,
            network: network.into(),
        }
    }

    pub fn new_with_mnemonic(
        prod_lvl: ProductionLevel,
        mnemonic: String,
        info: BlockchainInfo,
        app_dir_path: String,
    ) -> Self {
        let network = match &info {
            BlockchainInfo::Electrum { network, .. } => network.to_owned(),
            BlockchainInfo::Rpc { network, .. } => network.to_owned(),
        };
        let entropy = match bip39::Mnemonic::parse(mnemonic) {
            Ok(mnemonic) => mnemonic.to_entropy(),
            Err(error) => panic!("Invalid mnemonic - {}", error),
        };
        let secret_key = match SecretKey::from_slice(&entropy) {
            Ok(secret_key) => secret_key,
            Err(error) => panic!("Cannot make key from mnemonic - {}", error),
        };
        let inner = RUNTIME.block_on(async {
            InnerTrader::new_with_key(prod_lvl, secret_key, info.into(), app_dir_path).await
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

    pub fn wallet_balances(&self) -> Result<Balances, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.wallet_balances().await })
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

    pub fn wallet_blockchain_height(&self) -> Result<u32, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.wallet_blockchain_height().await })
            .map_err(|e| e.into())
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

    pub fn add_relays(&self, relay_addrs: Vec<RelayAddr>) -> Result<(), FatCrabError> {
        let mut relays = Vec::new();

        for relay_addr in relay_addrs {
            let socket = match relay_addr.socket_addr {
                Some(socket_str) => SocketAddr::from_str(&socket_str).ok(),
                None => None,
            };
            let url = Url::parse(&relay_addr.url)?;
            relays.push((url, socket));
        }

        RUNTIME
            .block_on(async { self.inner.add_relays(relays).await })
            .map_err(|e| e.into())
    }

    pub fn get_relays(&self) -> Vec<RelayInfo> {
        RUNTIME.block_on(async {
            self.inner
                .get_relays()
                .await
                .into_iter()
                .map(|relay| relay.into())
                .collect()
        })
    }

    pub fn remove_relay(&self, url: String) -> Result<(), FatCrabError> {
        let url = Url::parse(&url)?;
        RUNTIME
            .block_on(async { self.inner.remove_relay(url).await })
            .map_err(|e| e.into())
    }

    pub fn reconnect(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.reconnect().await })
            .map_err(|e| e.into())
    }

    pub fn new_buy_maker(
        &self,
        order: FatCrabOrder,
        fatcrab_rx_addr: String,
    ) -> Result<Arc<FatCrabBuyMaker>, FatCrabError> {
        match RUNTIME.block_on(async {
            self.inner
                .new_buy_maker(&order.into(), fatcrab_rx_addr)
                .await
        }) {
            Ok(maker_access) => Ok(Arc::new(FatCrabBuyMaker::new(maker_access))),
            Err(e) => Err(e.into()),
        }
    }

    pub fn new_sell_maker(
        &self,
        order: FatCrabOrder,
    ) -> Result<Arc<FatCrabSellMaker>, FatCrabError> {
        match RUNTIME.block_on(async { self.inner.new_sell_maker(&order.into()).await }) {
            Ok(maker_access) => Ok(Arc::new(FatCrabSellMaker::new(maker_access))),
            Err(e) => Err(e.into()),
        }
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

    pub fn new_buy_taker(
        &self,
        order_envelope: Arc<FatCrabOrderEnvelope>,
    ) -> Result<Arc<FatCrabBuyTaker>, FatCrabError> {
        let order_envelope = order_envelope.as_ref().clone();
        match RUNTIME.block_on(async { self.inner.new_buy_taker(&order_envelope.into()).await }) {
            Ok(taker_access) => Ok(Arc::new(FatCrabBuyTaker::new(taker_access))),
            Err(e) => Err(e.into()),
        }
    }

    pub fn new_sell_taker(
        &self,
        order_envelope: Arc<FatCrabOrderEnvelope>,
        fatcrab_rx_addr: String,
    ) -> Result<Arc<FatCrabSellTaker>, FatCrabError> {
        let order_envelope = order_envelope.as_ref().clone();
        match RUNTIME.block_on(async {
            self.inner
                .new_sell_taker(&order_envelope.into(), fatcrab_rx_addr)
                .await
        }) {
            Ok(taker_access) => Ok(Arc::new(FatCrabSellTaker::new(taker_access))),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_buy_makers(&self) -> HashMap<String, Arc<FatCrabBuyMaker>> {
        RUNTIME.block_on(async {
            self.inner
                .get_buy_makers()
                .await
                .into_iter()
                .map(|(uuid, maker_access)| {
                    (
                        uuid.to_string(),
                        Arc::new(FatCrabBuyMaker::new(maker_access)),
                    )
                })
                .collect()
        })
    }

    pub fn get_sell_makers(&self) -> HashMap<String, Arc<FatCrabSellMaker>> {
        RUNTIME.block_on(async {
            self.inner
                .get_sell_makers()
                .await
                .into_iter()
                .map(|(uuid, maker_access)| {
                    (
                        uuid.to_string(),
                        Arc::new(FatCrabSellMaker::new(maker_access)),
                    )
                })
                .collect()
        })
    }

    pub fn get_buy_takers(&self) -> HashMap<String, Arc<FatCrabBuyTaker>> {
        RUNTIME.block_on(async {
            self.inner
                .get_buy_takers()
                .await
                .into_iter()
                .map(|(uuid, taker_access)| {
                    (
                        uuid.to_string(),
                        Arc::new(FatCrabBuyTaker::new(taker_access)),
                    )
                })
                .collect()
        })
    }

    pub fn get_sell_takers(&self) -> HashMap<String, Arc<FatCrabSellTaker>> {
        RUNTIME.block_on(async {
            self.inner
                .get_sell_takers()
                .await
                .into_iter()
                .map(|(uuid, taker_access)| {
                    (
                        uuid.to_string(),
                        Arc::new(FatCrabSellTaker::new(taker_access)),
                    )
                })
                .collect()
        })
    }

    pub fn shutdown(&self) -> Result<(), FatCrabError> {
        // TODO: FFI requires shared reference to self as argument
        // But Trader Shutdown is a self consuming method
        // Not sure how to do this at this point. Ommitting for now
        // RUNTIME.block_on(async { self.inner.shutdown().await }).map_err(|e| e.into())
        Ok(())
    }
}
