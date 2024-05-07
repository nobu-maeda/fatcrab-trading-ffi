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
use tracing_oslog::OsLogger;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, Layer};

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
use trader::{Balances, FatCrabTrader};
use types::{
    Auth, BlockchainInfo, FatCrabMakerNotifDelegate, FatCrabTakerNotifDelegate, FilterLevel,
    Network, ProductionLevel, RelayAddr, RelayInfo, RelayInformationDocument, RelayStatus,
};

// Init tracing for Apple unified logging system
pub fn init_tracing_for_oslog(level: FilterLevel, log_timestamp: bool, log_level: bool) {
    let level_filter: LevelFilter = level.into();
    let collector = tracing_subscriber::registry().with(
        OsLogger::new("dev.n3xb.io.fatcrab-trading", "default")
            .log_timestamp(log_timestamp)
            .log_level(log_level)
            .with_filter(level_filter),
    );
    tracing::subscriber::set_global_default(collector).expect("failed to set global subscriber");
}
