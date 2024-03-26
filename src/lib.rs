mod error;
mod maker;
mod offer;
mod order;
mod peer;
mod taker;
mod trade_rsp;
mod trader;
mod types;

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
uniffi::include_scaffolding!("fatcrab_trading");
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

use error::FatCrabError;
use maker::{
    FatCrabBuyMaker, FatCrabMakerNotifOfferStruct, FatCrabMakerNotifPeerStruct, FatCrabMakerState,
    FatCrabSellMaker,
};
use offer::FatCrabOfferEnvelope;
use order::{FatCrabOrder, FatCrabOrderEnvelope, FatCrabOrderType};
use peer::{FatCrabPeerEnvelope, FatCrabPeerMessage};
use taker::{
    FatCrabBuyTaker, FatCrabSellTaker, FatCrabTakerNotifPeerStruct,
    FatCrabTakerNotifTradeRspStruct, FatCrabTakerState,
};
use trade_rsp::{FatCrabTradeRsp, FatCrabTradeRspEnvelope, FatCrabTradeRspType};
use trader::FatCrabTrader;
use types::{
    Auth, BlockchainInfo, FatCrabMakerNotifDelegate, FatCrabTakerNotifDelegate, Network, RelayAddr,
    RelayInfo, RelayInformationDocument, RelayStatus,
};
