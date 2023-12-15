use std::path::PathBuf;

use bitcoin::Network as InnerNetwork;
use core_rpc::Auth as InnerAuth;


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