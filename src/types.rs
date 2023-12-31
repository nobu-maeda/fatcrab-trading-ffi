use std::{path::PathBuf, sync::Arc};

use bitcoin::Network as InnerNetwork;
use core_rpc::Auth as InnerAuth;
use fatcrab_trading::common::BlockchainInfo as InnerInfo;
pub use fatcrab_trading::{maker::FatCrabMakerNotif, taker::FatCrabTakerNotif};

pub use crate::{
    offer::FatCrabOfferEnvelope, peer::FatCrabPeerEnvelope, trade_rsp::FatCrabTradeRspEnvelope,
};

pub enum Auth {
    None,
    UserPass { username: String, password: String },
    Cookie { file: String },
}

impl From<Auth> for InnerAuth {
    fn from(auth: Auth) -> Self {
        match auth {
            Auth::None => InnerAuth::None,
            Auth::UserPass { username, password } => InnerAuth::UserPass(username, password),
            Auth::Cookie { file } => InnerAuth::CookieFile(PathBuf::from(file)),
        }
    }
}

#[derive(Clone)]
pub enum Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<Network> for InnerNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => InnerNetwork::Bitcoin,
            Network::Testnet => InnerNetwork::Testnet,
            Network::Signet => InnerNetwork::Signet,
            Network::Regtest => InnerNetwork::Regtest,
        }
    }
}

pub enum BlockchainInfo {
    Electrum {
        url: String,
        network: Network,
    },
    Rpc {
        url: String,
        auth: Auth,
        network: Network,
    },
}

impl From<BlockchainInfo> for InnerInfo {
    fn from(info: BlockchainInfo) -> Self {
        match info {
            BlockchainInfo::Electrum { url, network } => InnerInfo::Electrum {
                url,
                network: network.into(),
            },
            BlockchainInfo::Rpc { url, auth, network } => InnerInfo::Rpc {
                url,
                auth: auth.into(),
                network: network.into(),
            },
        }
    }
}

pub struct RelayInfo {
    pub addr: String,
    pub socket_addr: Option<String>,
}

pub trait FatCrabMakerNotifDelegate: Sync + Send {
    fn on_maker_offer_notif(&self, offer_envelope: Arc<FatCrabOfferEnvelope>);
    fn on_maker_peer_notif(&self, peer_envelope: Arc<FatCrabPeerEnvelope>);
}

pub trait FatCrabTakerNotifDelegate: Sync + Send {
    fn on_taker_trade_rsp_notif(&self, trade_rsp_envelope: Arc<FatCrabTradeRspEnvelope>);
    fn on_taker_peer_notif(&self, peer_envelope: Arc<FatCrabPeerEnvelope>);
}
