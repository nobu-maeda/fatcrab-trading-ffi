mod error;
mod types;
mod order;
mod offer;
mod trade_rsp;
mod maker;
mod taker;
mod trader;
mod peer;

use tokio::runtime::Runtime;
use uniffi::deps::once_cell::sync::Lazy;
uniffi::include_scaffolding!("fatcrab_trading");
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

use error::FatCrabError;
use types::{Auth, Network, BlockchainInfo, RelayInfo, FatCrabMakerNotifDelegate, FatCrabTakerNotifDelegate};
use order::{FatCrabOrder, FatCrabOrderType, FatCrabOrderEnvelope};
use offer::FatCrabOfferEnvelope;
use trade_rsp::{FatCrabTradeRspType, FatCrabTradeRspEnvelope};
use peer::FatCrabPeerEnvelope;
use maker::{FatCrabBuyMaker, FatCrabSellMaker};
use taker::{FatCrabBuyTaker, FatCrabSellTaker};
use trader::FatCrabTrader;

