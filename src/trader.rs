
use std::net::SocketAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use bitcoin::{Address, Network};
use fatcrab_trading::order::{FatCrabOrderType, FatCrabOrderEnvelope, self};
use secp256k1::SecretKey;

use fatcrab_trading::trader::FatCrabTrader as InnerTrader;

use crate::RUNTIME;
use crate::error::FatCrabError;
use crate::types::{BlockchainInfo, RelayInfo};
use crate::order::FatCrabOrder;
use crate::maker::FatCrabMaker;
use crate::taker::FatCrabTaker;

pub struct Trader {
    inner: InnerTrader,
    network: Network
}

impl Trader {
    pub fn new(info: BlockchainInfo) -> Self {
        let network = match &info {
            BlockchainInfo::Electrum { network, .. } => network.to_owned(),
            BlockchainInfo::Rpc { network, .. } => network.to_owned(),
        };
        let inner = RUNTIME.block_on(async { InnerTrader::new(info.into()).await });
        Self { inner, network: network.into() }
    }

    pub fn new_with_keys(key: String, info: BlockchainInfo) -> Self {
        let network = match &info {
            BlockchainInfo::Electrum { network, .. } => network.to_owned(),
            BlockchainInfo::Rpc { network, .. } => network.to_owned(),
        };
        let secret_key = SecretKey::from_str(&key).unwrap();
        let inner = RUNTIME.block_on(async {
            InnerTrader::new_with_keys(secret_key, info.into()).await
        });
        Self { inner, network: network.into() }
    }

    pub fn wallet_bip39_mnemonic(&self) -> Result<String, FatCrabError> {
        let result = RUNTIME.block_on(async { self.inner.wallet_bip39_mnemonic().await });
        match result {
            Ok(mnemonic) => Ok(mnemonic.to_string()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn wallet_spendable_balance(&self) -> Result<u64, FatCrabError> {
        RUNTIME.block_on(async { self.inner.wallet_spendable_balance().await }).map_err(|e| e.into())
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
        let result = RUNTIME.block_on(async { self.inner.wallet_send_to_address(address, amount).await });
        match result {
            Ok(txid) => Ok(txid.to_string()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn wallet_blockchain_sync(&self) -> Result<(), FatCrabError> {
        RUNTIME.block_on(async { self.inner.wallet_blockchain_sync().await }).map_err(|e| e.into())
    }

    pub fn nostr_pubkey(&self) -> String {
        RUNTIME.block_on(async { self.inner.nostr_pubkey().await }).to_string()
    }

    pub fn add_relays(
        &self,
        relays: Vec<RelayInfo>,
    ) -> Result<(), FatCrabError> {
        let relays = relays
            .into_iter()
            .map(| relay_info | {
                let socket =
                match relay_info.socket_addr {
                    Some(socket_str) => {
                        SocketAddr::from_str(&socket_str).ok()
                    },
                    None => None
                };
                (relay_info.addr, socket)
            })
            .collect();
        RUNTIME.block_on(async { self.inner.add_relays(relays).await }).map_err(|e| e.into())
    }

    pub fn make_buy_order(
        &self,
        order: Arc<FatCrabOrder>,
        fatcrab_rx_addr: String,
    ) -> Arc<FatCrabMaker> {
        let order = order.as_ref().clone();
        let maker_access = RUNTIME.block_on(async { self.inner.make_buy_order(order.into(), fatcrab_rx_addr).await });
        Arc::new(FatCrabMaker::new_with_buy_order(maker_access))
    }

    pub fn make_sell_order(&self, order: Arc<FatCrabOrder>) -> Arc<FatCrabMaker> {
        let order = order.as_ref().clone();
        let maker_access = RUNTIME.block_on(async { self.inner.make_sell_order(order.into()).await });
        Arc::new(FatCrabMaker::new_with_sell_order(maker_access))
    }

    pub fn query_orders(
        &self,
        order_type: FatCrabOrderType,
    ) -> Result<Vec<Arc<FatCrabOrderEnvelope>>, FatCrabError> {
        match RUNTIME.block_on(async { self.inner.query_orders(order_type).await }) {
            Ok(orders) => Ok(orders.into_iter().map(|order| Arc::new(order)).collect()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn take_buy_order(
        &self,
        order_envelope: Arc<FatCrabOrderEnvelope>,
    ) -> Arc<FatCrabTaker> {
        let order_envelope = order_envelope.as_ref().clone();
        let taker_access = RUNTIME.block_on(async { self.inner.take_buy_order(order_envelope).await });
        Arc::new(FatCrabTaker::new_with_buy_order(taker_access))
    }

    pub fn take_sell_order(
        &self,
        order_envelope: Arc<FatCrabOrderEnvelope>,
        fatcrab_rx_addr: String,
    ) -> Arc<FatCrabTaker> {
        let order_envelope = order_envelope.as_ref().clone();
        let taker_access = RUNTIME.block_on(async { self.inner.take_sell_order(order_envelope, fatcrab_rx_addr).await });
        Arc::new(FatCrabTaker::new_with_sell_order(taker_access))
    }

    pub fn shutdown(self) -> Result<(), FatCrabError> {
        RUNTIME.block_on(async { self.inner.shutdown().await }).map_err(|e| e.into())
    }
}
