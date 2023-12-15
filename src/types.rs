use std::path::PathBuf;

use bitcoin::Network as InnerNetwork;
use core_rpc::Auth as InnerAuth;
use fatcrab_trading::common::BlockchainInfo as InnerInfo;

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
    }
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
            }
        }
    }
}

pub struct RelayInfo {
    pub addr: String,
    pub socket_addr: Option<String>,
}