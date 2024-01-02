use std::sync::Arc;

use fatcrab_trading::maker::{FatCrabMakerAccess, FatCrabMakerNotif, MakerBuy, MakerSell};
use tokio::sync::mpsc;

use crate::offer::FatCrabOfferEnvelope;
use crate::trade_rsp::FatCrabTradeRspType;
use crate::types::FatCrabMakerNotifDelegate;
use crate::{error::FatCrabError, RUNTIME};

const MAKER_NOTIF_CHANNEL_SIZE: usize = 10;

pub struct FatCrabBuyMaker {
    inner: FatCrabMakerAccess<MakerBuy>,
}

pub struct FatCrabSellMaker {
    inner: FatCrabMakerAccess<MakerSell>,
}

impl FatCrabBuyMaker {
    pub(crate) fn new(maker: FatCrabMakerAccess<MakerBuy>) -> Self {
        Self { inner: maker }
    }

    pub fn post_new_order(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.post_new_order().await })
            .map_err(|e| e.into())
    }

    pub fn trade_response(
        &self,
        trade_rsp_type: FatCrabTradeRspType,
        offer_envelope: Arc<FatCrabOfferEnvelope>,
    ) -> Result<(), FatCrabError> {
        let offer_envelope = offer_envelope.as_ref().clone();
        RUNTIME
            .block_on(async {
                self.inner
                    .trade_response(trade_rsp_type, offer_envelope.into())
                    .await
            })
            .map_err(|e| e.into())
    }

    pub fn release_notify_peer(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.release_notify_peer().await })
            .map_err(|e| e.into())
    }

    pub fn trade_complete(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.trade_complete().await })
            .map_err(|e| e.into())
    }

    pub fn register_notif_delegate(
        &self,
        delegate: Arc<dyn FatCrabMakerNotifDelegate>,
    ) -> Result<(), FatCrabError> {
        RUNTIME.block_on(async {
            _ = self.inner.unregister_notif_tx().await;
        });
        let (tx, mut rx) = mpsc::channel(MAKER_NOTIF_CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(notif) = rx.recv().await {
                match notif {
                    FatCrabMakerNotif::Offer(offer_envelope) => {
                        delegate.on_maker_offer_notif(Arc::new(offer_envelope.into()));
                    }
                    FatCrabMakerNotif::Peer(peer_envelope) => {
                        delegate.on_maker_peer_notif(Arc::new(peer_envelope.into()));
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

impl FatCrabSellMaker {
    pub(crate) fn new(maker: FatCrabMakerAccess<MakerSell>) -> Self {
        Self { inner: maker }
    }

    pub fn post_new_order(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.post_new_order().await })
            .map_err(|e| e.into())
    }

    pub fn trade_response(
        &self,
        trade_rsp_type: FatCrabTradeRspType,
        offer_envelope: Arc<FatCrabOfferEnvelope>,
    ) -> Result<(), FatCrabError> {
        let offer_envelope = offer_envelope.as_ref().clone();
        RUNTIME
            .block_on(async {
                self.inner
                    .trade_response(trade_rsp_type, offer_envelope.into())
                    .await
            })
            .map_err(|e| e.into())
    }

    pub fn check_btc_tx_confirmation(&self) -> Result<u32, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.check_btc_tx_confirmation().await })
            .map_err(|e| e.into())
    }

    pub fn notify_peer(&self, fatcrab_txid: String) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.notify_peer(fatcrab_txid).await })
            .map_err(|e| e.into())
    }

    pub fn trade_complete(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.trade_complete().await })
            .map_err(|e| e.into())
    }

    pub fn register_notif_delegate(
        &self,
        delegate: Arc<dyn FatCrabMakerNotifDelegate>,
    ) -> Result<(), FatCrabError> {
        RUNTIME.block_on(async {
            _ = self.inner.unregister_notif_tx().await;
        });
        let (tx, mut rx) = mpsc::channel(MAKER_NOTIF_CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(notif) = rx.recv().await {
                match notif {
                    FatCrabMakerNotif::Offer(offer_envelope) => {
                        delegate.on_maker_offer_notif(Arc::new(offer_envelope.into()));
                    }
                    FatCrabMakerNotif::Peer(peer_envelope) => {
                        delegate.on_maker_peer_notif(Arc::new(peer_envelope.into()));
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
