use std::path::PathBuf;

use bitcoin::Network as InnerNetwork;
use core_rpc::Auth as InnerAuth;
use fatcrab_trading::common::BlockchainInfo as InnerBlockchainInfo;
use fatcrab_trading::RelayInfo as InnerRelayInfo;
use fatcrab_trading::RelayInformationDocument as InnerRelayInformationDocument;
pub use fatcrab_trading::{maker::FatCrabMakerNotif, taker::FatCrabTakerNotif, RelayStatus};

use crate::maker::FatCrabMakerNotifOfferStruct;
use crate::maker::FatCrabMakerNotifPeerStruct;
use crate::taker::FatCrabTakerNotifPeerStruct;
use crate::taker::FatCrabTakerNotifTradeRspStruct;
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

impl From<BlockchainInfo> for InnerBlockchainInfo {
    fn from(info: BlockchainInfo) -> Self {
        match info {
            BlockchainInfo::Electrum { url, network } => InnerBlockchainInfo::Electrum {
                url,
                network: network.into(),
            },
            BlockchainInfo::Rpc { url, auth, network } => InnerBlockchainInfo::Rpc {
                url,
                auth: auth.into(),
                network: network.into(),
            },
        }
    }
}

pub struct RelayAddr {
    pub url: String,
    pub socket_addr: Option<String>,
}

pub struct RelayInformationDocument {
    /// Name
    pub name: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Owner public key
    pub pubkey: Option<String>,
    /// Owner contact
    pub contact: Option<String>,
    /// Supported NIPs
    pub supported_nips: Option<Vec<u16>>,
    /// Software
    pub software: Option<String>,
    /// Software version
    pub version: Option<String>,
    /// Country codes whose laws and policies may affect this relay
    pub relay_countries: Vec<String>,
    /// Ordered list of IETF language tags indicating the major languages spoken on the relay
    pub language_tags: Vec<String>,
    /// List of limitations on the topics to be discussed
    pub tags: Vec<String>,
    /// Link to a human-readable page which specifies the community policies
    pub posting_policy: Option<String>,
    /// Link to relay's fee schedules
    pub payments_url: Option<String>,
    /// URL pointing to an image to be used as an icon for the relay
    pub icon: Option<String>,
}

impl From<InnerRelayInformationDocument> for RelayInformationDocument {
    fn from(document: InnerRelayInformationDocument) -> Self {
        RelayInformationDocument {
            name: document.name,
            description: document.description,
            pubkey: document.pubkey,
            contact: document.contact,
            supported_nips: document.supported_nips,
            software: document.software,
            version: document.version,
            relay_countries: document.relay_countries,
            language_tags: document.language_tags,
            tags: document.tags,
            posting_policy: document.posting_policy,
            payments_url: document.payments_url,
            icon: document.icon,
        }
    }
}

pub struct RelayInfo {
    pub url: String,
    pub status: RelayStatus,
    pub document: RelayInformationDocument,
}

impl From<InnerRelayInfo> for RelayInfo {
    fn from(relay_info: InnerRelayInfo) -> Self {
        RelayInfo {
            url: relay_info.url.to_string(),
            status: relay_info.status,
            document: relay_info.document.into(),
        }
    }
}

pub trait FatCrabMakerNotifDelegate: Sync + Send {
    fn on_maker_offer_notif(&self, offer_notif: FatCrabMakerNotifOfferStruct);
    fn on_maker_peer_notif(&self, peer_notif: FatCrabMakerNotifPeerStruct);
}

pub trait FatCrabTakerNotifDelegate: Sync + Send {
    fn on_taker_trade_rsp_notif(&self, trade_rsp_notif: FatCrabTakerNotifTradeRspStruct);
    fn on_taker_peer_notif(&self, peer_notif: FatCrabTakerNotifPeerStruct);
}
