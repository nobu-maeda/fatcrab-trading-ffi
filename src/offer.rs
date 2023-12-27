use fatcrab_trading::offer::FatCrabOfferEnvelope as InnerEnvelope;

#[derive(Clone)]
pub struct FatCrabOfferEnvelope {
    inner: InnerEnvelope,
}

impl From<InnerEnvelope> for FatCrabOfferEnvelope {
    fn from(envelope: InnerEnvelope) -> Self {
        Self { inner: envelope }
    }
}

impl Into<InnerEnvelope> for FatCrabOfferEnvelope {
    fn into(self) -> InnerEnvelope {
        self.inner
    }
}
