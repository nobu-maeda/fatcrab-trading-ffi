use std::sync::Arc;
use std::thread::spawn;

pub use fatcrab_trading::maker::FatCrabMakerState;
use fatcrab_trading::maker::{FatCrabMakerAccess, FatCrabMakerNotif, MakerBuy, MakerSell};
use fatcrab_trading::maker::{
    FatCrabMakerNotifOfferStruct as InnerMakerNotifOfferStruct,
    FatCrabMakerNotifPeerStruct as InnerMakerNotifPeerStruct,
};
use tokio::sync::mpsc;

use crate::offer::FatCrabOfferEnvelope;
use crate::order::FatCrabOrder;
use crate::peer::FatCrabPeerEnvelope;
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

    pub fn post_new_order(&self) -> Result<FatCrabMakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.post_new_order().await })
            .map_err(|e| e.into())
    }

    pub fn get_order_details(&self) -> Result<FatCrabOrder, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_order_details().await })
            .map(|order| order.into())
            .map_err(|e| e.into())
    }

    pub fn get_state(&self) -> Result<FatCrabMakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_state().await })
            .map_err(|e| e.into())
    }

    pub fn get_peer_pubkey(&self) -> Result<Option<String>, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_peer_pubkey().await })
            .map_err(|e| e.into())
    }

    pub fn query_offers(&self) -> Result<Vec<Arc<FatCrabOfferEnvelope>>, FatCrabError> {
        RUNTIME
            .block_on(async {
                match self.inner.query_offers().await {
                    Ok(offers) => {
                        let offer_envelopes: Vec<Arc<FatCrabOfferEnvelope>> = offers
                            .iter()
                            .map(|o| Arc::new(o.to_owned().into()))
                            .collect();
                        return Ok(offer_envelopes);
                    }
                    Err(e) => return Err(e),
                }
            })
            .map_err(|e| e.into())
    }

    pub fn query_peer_msg(&self) -> Result<Option<Arc<FatCrabPeerEnvelope>>, FatCrabError> {
        RUNTIME
            .block_on(async {
                match self.inner.query_peer_msg().await {
                    Ok(peer_msg) => {
                        if let Some(msg) = peer_msg {
                            return Ok(Some(Arc::new(msg.into())));
                        } else {
                            return Ok(None);
                        }
                    }
                    Err(e) => return Err(e),
                }
            })
            .map_err(|e| e.into())
    }

    pub fn cancel_order(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.cancel_order().await })
            .map_err(|e| e.into())
    }

    pub fn trade_response(
        &self,
        trade_rsp_type: FatCrabTradeRspType,
        offer_envelope: Arc<FatCrabOfferEnvelope>,
    ) -> Result<FatCrabMakerState, FatCrabError> {
        let offer_envelope = offer_envelope.as_ref().clone();
        RUNTIME
            .block_on(async {
                self.inner
                    .trade_response(trade_rsp_type, offer_envelope.into())
                    .await
            })
            .map_err(|e| e.into())
    }

    pub fn release_notify_peer(&self) -> Result<FatCrabMakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.release_notify_peer().await })
            .map_err(|e| e.into())
    }

    pub fn trade_complete(&self) -> Result<FatCrabMakerState, FatCrabError> {
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
        spawn(move || loop {
            match rx.blocking_recv() {
                Some(notif) => match notif {
                    FatCrabMakerNotif::Offer(offer_notif) => {
                        delegate.on_maker_offer_notif(offer_notif.into());
                    }
                    FatCrabMakerNotif::Peer(peer_notif) => {
                        delegate.on_maker_peer_notif(peer_notif.into());
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

impl FatCrabSellMaker {
    pub(crate) fn new(maker: FatCrabMakerAccess<MakerSell>) -> Self {
        Self { inner: maker }
    }

    pub fn post_new_order(&self) -> Result<FatCrabMakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.post_new_order().await })
            .map_err(|e| e.into())
    }

    pub fn get_order_details(&self) -> Result<FatCrabOrder, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_order_details().await })
            .map(|order| order.into())
            .map_err(|e| e.into())
    }

    pub fn get_state(&self) -> Result<FatCrabMakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_state().await })
            .map_err(|e| e.into())
    }

    pub fn get_peer_pubkey(&self) -> Result<Option<String>, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_peer_pubkey().await })
            .map_err(|e| e.into())
    }

    pub fn get_peer_btc_txid(&self) -> Result<Option<String>, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.get_peer_btc_txid().await })
            .map_err(|e| e.into())
    }

    pub fn query_offers(&self) -> Result<Vec<Arc<FatCrabOfferEnvelope>>, FatCrabError> {
        RUNTIME
            .block_on(async {
                match self.inner.query_offers().await {
                    Ok(offers) => {
                        let offer_envelopes: Vec<Arc<FatCrabOfferEnvelope>> = offers
                            .iter()
                            .map(|o| Arc::new(o.to_owned().into()))
                            .collect();
                        return Ok(offer_envelopes);
                    }
                    Err(e) => return Err(e),
                }
            })
            .map_err(|e| e.into())
    }

    pub fn query_peer_msg(&self) -> Result<Option<Arc<FatCrabPeerEnvelope>>, FatCrabError> {
        RUNTIME
            .block_on(async {
                match self.inner.query_peer_msg().await {
                    Ok(peer_msg) => {
                        if let Some(msg) = peer_msg {
                            return Ok(Some(Arc::new(msg.into())));
                        } else {
                            return Ok(None);
                        }
                    }
                    Err(e) => return Err(e),
                }
            })
            .map_err(|e| e.into())
    }

    pub fn cancel_order(&self) -> Result<(), FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.cancel_order().await })
            .map_err(|e| e.into())
    }

    pub fn trade_response(
        &self,
        trade_rsp_type: FatCrabTradeRspType,
        offer_envelope: Arc<FatCrabOfferEnvelope>,
    ) -> Result<FatCrabMakerState, FatCrabError> {
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

    pub fn notify_peer(&self, fatcrab_txid: String) -> Result<FatCrabMakerState, FatCrabError> {
        RUNTIME
            .block_on(async { self.inner.notify_peer(fatcrab_txid).await })
            .map_err(|e| e.into())
    }

    pub fn trade_complete(&self) -> Result<FatCrabMakerState, FatCrabError> {
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
        spawn(move || loop {
            match rx.blocking_recv() {
                Some(notif) => match notif {
                    FatCrabMakerNotif::Offer(offer_notif) => {
                        delegate.on_maker_offer_notif(offer_notif.into());
                    }
                    FatCrabMakerNotif::Peer(peer_notif) => {
                        delegate.on_maker_peer_notif(peer_notif.into());
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

pub struct FatCrabMakerNotifOfferStruct {
    pub state: FatCrabMakerState,
    pub offer_envelope: Arc<FatCrabOfferEnvelope>,
}

impl From<InnerMakerNotifOfferStruct> for FatCrabMakerNotifOfferStruct {
    fn from(offer_notif: InnerMakerNotifOfferStruct) -> Self {
        Self {
            state: offer_notif.state,
            offer_envelope: Arc::new(offer_notif.offer_envelope.into()),
        }
    }
}

impl Into<InnerMakerNotifOfferStruct> for FatCrabMakerNotifOfferStruct {
    fn into(self) -> InnerMakerNotifOfferStruct {
        InnerMakerNotifOfferStruct {
            state: self.state,
            offer_envelope: self.offer_envelope.as_ref().clone().into(),
        }
    }
}

pub struct FatCrabMakerNotifPeerStruct {
    pub state: FatCrabMakerState,
    pub peer_envelope: Arc<FatCrabPeerEnvelope>,
}

impl From<InnerMakerNotifPeerStruct> for FatCrabMakerNotifPeerStruct {
    fn from(peer_notif: InnerMakerNotifPeerStruct) -> Self {
        Self {
            state: peer_notif.state,
            peer_envelope: Arc::new(peer_notif.peer_envelope.into()),
        }
    }
}

impl Into<InnerMakerNotifPeerStruct> for FatCrabMakerNotifPeerStruct {
    fn into(self) -> InnerMakerNotifPeerStruct {
        InnerMakerNotifPeerStruct {
            state: self.state,
            peer_envelope: self.peer_envelope.as_ref().clone().into(),
        }
    }
}
