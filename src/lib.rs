mod error;
mod types;
mod order;
mod maker;
mod taker;
mod trader;

use tokio::runtime::Runtime;
use uniffi::deps::once_cell::sync::Lazy;
uniffi::include_scaffolding!("fatcrab_trading");
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

use error::FatCrabError;
use types::{Auth, Network, BlockchainInfo, RelayInfo};
use order::{FatCrabOrder, FatCrabOrderType, FatCrabOrderEnvelope};
use maker::FatCrabMaker;
use taker::FatCrabTaker;
use trader::Trader;
