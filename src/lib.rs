mod types;
mod trader;

use tokio::runtime::Runtime;
use uniffi::deps::once_cell::sync::Lazy;
uniffi::include_scaffolding!("fatcrab_trading");
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

use types::{Auth, Network};
use trader::Trader;
