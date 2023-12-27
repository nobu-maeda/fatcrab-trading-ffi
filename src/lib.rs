mod error;
mod maker;
mod offer;
mod order;
mod peer;
mod taker;
mod trade_rsp;
mod trader;
mod types;

use tokio::runtime::Runtime;
use uniffi::deps::once_cell::sync::Lazy;
uniffi::include_scaffolding!("fatcrab_trading");
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

use error::FatCrabError;
use maker::{FatCrabBuyMaker, FatCrabSellMaker};
use offer::FatCrabOfferEnvelope;
use order::{FatCrabOrder, FatCrabOrderEnvelope, FatCrabOrderType};
use peer::{FatCrabPeerEnvelope, FatCrabPeerMessage};
use taker::{FatCrabBuyTaker, FatCrabSellTaker};
use trade_rsp::{FatCrabTradeRsp, FatCrabTradeRspEnvelope, FatCrabTradeRspType};
use trader::FatCrabTrader;
use types::{
    Auth, BlockchainInfo, FatCrabMakerNotifDelegate, FatCrabTakerNotifDelegate, Network, RelayInfo,
};
