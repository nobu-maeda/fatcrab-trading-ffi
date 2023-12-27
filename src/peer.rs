use fatcrab_trading::peer::FatCrabPeerEnvelope as InnerEnvelope;
pub use fatcrab_trading::peer::FatCrabPeerMessage;

#[derive(Clone)]
pub struct FatCrabPeerEnvelope {
    inner: InnerEnvelope,
}

impl From<InnerEnvelope> for FatCrabPeerEnvelope {
    fn from(envelope: InnerEnvelope) -> Self {
        Self { inner: envelope }
    }
}

impl Into<InnerEnvelope> for FatCrabPeerEnvelope {
    fn into(self) -> InnerEnvelope {
        self.inner
    }
}

impl FatCrabPeerEnvelope {
    pub fn message(&self) -> FatCrabPeerMessage {
        self.inner.message.clone().into()
    }
}
