use std::sync::Arc;

use fatcrab_trading::taker::{FatCrabTakerAccess, FatCrabTakerNotif, TakerBuy, TakerSell};
use tokio::sync::mpsc;

use crate::types::FatCrabTakerNotifDelegate;
use crate::{error::FatCrabError, RUNTIME};

const TAKER_NOTIF_CHANNEL_SIZE: usize = 10;

pub struct FatCrabBuyTaker {
    inner: FatCrabTakerAccess<TakerBuy>,
}

pub struct FatCrabSellTaker {
    inner: FatCrabTakerAccess<TakerSell>,
}

impl FatCrabBuyTaker {
    pub(crate) fn new(taker: FatCrabTakerAccess<TakerBuy>) -> Self {
        Self { inner: taker }
    }

    pub fn notify_peer(&self, txid: String) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.notify_peer(txid).await })
            .map_err(|e| e.into())
    }

    pub fn check_btc_tx_confirmation(&self) -> Result<u32, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.check_btc_tx_confirmation().await })
            .map_err(|e| e.into())
    }

    pub fn trade_complete(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.trade_complete().await })
            .map_err(|e| e.into())
    }

    pub fn register_notif_delegate(
        &self,
        delegate: Arc<dyn FatCrabTakerNotifDelegate>,
    ) -> Result<(), FatCrabError> {
        RUNTIME.block_on(async {
            _ = self.inner.unregister_notif_tx().await;
        });
        let (tx, mut rx) = mpsc::channel(TAKER_NOTIF_CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(notif) = rx.recv().await {
                match notif {
                    FatCrabTakerNotif::TradeRsp(trade_rsp_envelope) => {
                        delegate.on_taker_trade_rsp_notif(Arc::new(trade_rsp_envelope.into()));
                    }
                    FatCrabTakerNotif::Peer(peer_envelope) => {
                        delegate.on_taker_peer_notif(Arc::new(peer_envelope.into()));
                    }
                }
            }
        });
        RUNTIME
            .block_on(async { self.inner.register_notif_tx(tx).await })
            .map_err(|e| e.into())
    }

    pub fn unregister_notif_delegate(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.unregister_notif_tx().await })
            .map_err(|e| e.into())
    }
}

impl FatCrabSellTaker {
    pub(crate) fn new(taker: FatCrabTakerAccess<TakerSell>) -> Self {
        Self { inner: taker }
    }

    pub fn trade_complete(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.trade_complete().await })
            .map_err(|e| e.into())
    }

    pub fn register_notif_delegate(
        &self,
        delegate: Arc<dyn FatCrabTakerNotifDelegate>,
    ) -> Result<(), FatCrabError> {
        RUNTIME.block_on(async {
            _ = self.inner.unregister_notif_tx().await;
        });
        let (tx, mut rx) = mpsc::channel(TAKER_NOTIF_CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(notif) = rx.recv().await {
                match notif {
                    FatCrabTakerNotif::TradeRsp(trade_rsp_envelope) => {
                        delegate.on_taker_trade_rsp_notif(Arc::new(trade_rsp_envelope.into()));
                    }
                    FatCrabTakerNotif::Peer(peer_envelope) => {
                        delegate.on_taker_peer_notif(Arc::new(peer_envelope.into()));
                    }
                }
            }
        });
        RUNTIME
            .block_on(async { self.inner.register_notif_tx(tx).await })
            .map_err(|e| e.into())
    }

    pub fn unregister_notif_delegate(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.unregister_notif_tx().await })
            .map_err(|e| e.into())
    }
}
