use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use uniffi::deps::once_cell::sync::Lazy;

pub use fatcrab_trading::error::FatCrabError;
use fatcrab_trading::make_trade::MakeTradeAccess;
use fatcrab_trading::take_trade::TakeTradeAccess;
use fatcrab_trading::trader::Trader as InnerTrader;

uniffi::include_scaffolding!("fatcrab_trading");

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

pub trait MakeTradeWatching: Send + Sync {
    fn notify(&self, notification: String);
}

pub struct MakeTrade {
    make_trade_accessor: MakeTradeAccess,
}

impl MakeTrade {
    fn new(accessor: MakeTradeAccess) -> Self {
        Self {
            make_trade_accessor: accessor,
        }
    }

    pub fn watch(&self, watcher: Arc<dyn MakeTradeWatching>) -> Result<(), FatCrabError> {
        const MAKE_TRADE_WATCHER_CHANNEL_SIZE: usize = 5;

        let (tx, mut rx) = mpsc::channel(MAKE_TRADE_WATCHER_CHANNEL_SIZE);

        tokio::spawn(async move {
            while let Some(notification) = rx.recv().await {
                watcher.notify(notification);
            }
        });

        RUNTIME.block_on(async { self.make_trade_accessor.register_notif_tx(tx).await })
    }
}

pub struct TakeTrade {
    take_trade_accessor: TakeTradeAccess,
}

impl TakeTrade {
    fn new(accessor: TakeTradeAccess) -> Self {
        Self {
            take_trade_accessor: accessor,
        }
    }
}

pub struct Trader {
    inner: InnerTrader,
}

impl Trader {
    pub fn new() -> Self {
        let inner = RUNTIME.block_on(async { InnerTrader::new().await });
        Self { inner }
    }

    pub fn list_orders(&self) {
        self.inner.list_orders();
    }

    pub fn make_order(&self) -> Arc<MakeTrade> {
        let accessor = self.inner.make_order();
        Arc::new(MakeTrade::new(accessor))
    }

    pub fn take_order(&self) -> Arc<TakeTrade> {
        let accessor = self.inner.take_order();
        Arc::new(TakeTrade::new(accessor))
    }
}
