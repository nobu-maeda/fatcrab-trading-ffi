use std::sync::Arc;
use std::thread::spawn;

pub use fatcrab_trading::taker::FatCrabTakerState;
use fatcrab_trading::taker::{FatCrabTakerAccess, FatCrabTakerNotif, TakerBuy, TakerSell};
use fatcrab_trading::taker::{
    FatCrabTakerNotifPeerStruct as InnerTakerNotifPeerStruct,
    FatCrabTakerNotifTradeRspStruct as InnerTakerNotifTradeRspStruct,
};
use tokio::sync::mpsc;

use crate::order::FatCrabOrderEnvelope;
use crate::peer::FatCrabPeerEnvelope;
use crate::trade_rsp::FatCrabTradeRspEnvelope;
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

    pub fn take_order(&self) -> Result<FatCrabTakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.take_order().await })
            .map_err(|e| e.into())
    }

    pub fn get_order_details(&self) -> Result<Arc<FatCrabOrderEnvelope>, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_order_details().await })
            .map(|order_envelope| Arc::new(order_envelope.into()))
            .map_err(|e| e.into())
    }

    pub fn get_state(&self) -> Result<FatCrabTakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_state().await })
            .map_err(|e| e.into())
    }

    pub fn query_trade_rsp(&self) -> Result<Option<Arc<FatCrabTradeRspEnvelope>>, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.query_trade_rsp().await })
            .map(|trade_rsp| match trade_rsp {
                Some(trade_rsp) => Some(Arc::new(trade_rsp.into())),
                None => None,
            })
            .map_err(|e| e.into())
    }

    pub fn notify_peer(&self, txid: String) -> Result<FatCrabTakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.notify_peer(txid).await })
            .map_err(|e| e.into())
    }

    pub fn check_btc_tx_confirmation(&self) -> Result<u32, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.check_btc_tx_confirmation().await })
            .map_err(|e| e.into())
    }

    pub fn trade_complete(&self) -> Result<FatCrabTakerState, FatCrabError> {
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
        spawn(move || loop {
            match rx.blocking_recv() {
                Some(notif) => match notif {
                    FatCrabTakerNotif::TradeRsp(trade_rsp_notif) => {
                        delegate.on_taker_trade_rsp_notif(trade_rsp_notif.into());
                    }
                    FatCrabTakerNotif::Peer(peer_notif) => {
                        delegate.on_taker_peer_notif(peer_notif.into());
                    }
                },
                None => break,
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

    pub fn take_order(&self) -> Result<FatCrabTakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.take_order().await })
            .map_err(|e| e.into())
    }

    pub fn get_order_details(&self) -> Result<Arc<FatCrabOrderEnvelope>, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_order_details().await })
            .map(|order_envelope| Arc::new(order_envelope.into()))
            .map_err(|e| e.into())
    }

    pub fn get_state(&self) -> Result<FatCrabTakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_state().await })
            .map_err(|e| e.into())
    }

    pub fn trade_complete(&self) -> Result<FatCrabTakerState, FatCrabError> {
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
        spawn(move || loop {
            match rx.blocking_recv() {
                Some(notif) => match notif {
                    FatCrabTakerNotif::TradeRsp(trade_rsp_notif) => {
                        delegate.on_taker_trade_rsp_notif(trade_rsp_notif.into());
                    }
                    FatCrabTakerNotif::Peer(peer_notif) => {
                        delegate.on_taker_peer_notif(peer_notif.into());
                    }
                },
                None => break,
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

pub struct FatCrabTakerNotifTradeRspStruct {
    pub state: FatCrabTakerState,
    pub trade_rsp_envelope: Arc<FatCrabTradeRspEnvelope>,
}

impl From<InnerTakerNotifTradeRspStruct> for FatCrabTakerNotifTradeRspStruct {
    fn from(trade_rsp_notif: InnerTakerNotifTradeRspStruct) -> Self {
        Self {
            state: trade_rsp_notif.state,
            trade_rsp_envelope: Arc::new(trade_rsp_notif.trade_rsp_envelope.into()),
        }
    }
}

impl Into<InnerTakerNotifTradeRspStruct> for FatCrabTakerNotifTradeRspStruct {
    fn into(self) -> InnerTakerNotifTradeRspStruct {
        InnerTakerNotifTradeRspStruct {
            state: self.state,
            trade_rsp_envelope: self.trade_rsp_envelope.as_ref().clone().into(),
        }
    }
}

pub struct FatCrabTakerNotifPeerStruct {
    pub state: FatCrabTakerState,
    pub peer_envelope: Arc<FatCrabPeerEnvelope>,
}

impl From<InnerTakerNotifPeerStruct> for FatCrabTakerNotifPeerStruct {
    fn from(peer_notif: InnerTakerNotifPeerStruct) -> Self {
        Self {
            state: peer_notif.state,
            peer_envelope: Arc::new(peer_notif.peer_envelope.into()),
        }
    }
}

impl Into<InnerTakerNotifPeerStruct> for FatCrabTakerNotifPeerStruct {
    fn into(self) -> InnerTakerNotifPeerStruct {
        InnerTakerNotifPeerStruct {
            state: self.state,
            peer_envelope: self.peer_envelope.as_ref().clone().into(),
        }
    }
}
