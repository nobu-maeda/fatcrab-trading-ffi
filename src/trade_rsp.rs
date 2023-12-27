use fatcrab_trading::trade_rsp::FatCrabTradeRsp as InnerTradeRsp;
use fatcrab_trading::trade_rsp::FatCrabTradeRspEnvelope as InnerEnvelope;
pub use fatcrab_trading::trade_rsp::FatCrabTradeRspType;

#[derive(Clone)]
pub struct FatCrabTradeRspEnvelope {
    inner: InnerEnvelope,
}

impl From<InnerEnvelope> for FatCrabTradeRspEnvelope {
    fn from(envelope: InnerEnvelope) -> Self {
        Self { inner: envelope }
    }
}

impl Into<InnerEnvelope> for FatCrabTradeRspEnvelope {
    fn into(self) -> InnerEnvelope {
        self.inner
    }
}

impl FatCrabTradeRspEnvelope {
    pub fn trade_rsp(&self) -> FatCrabTradeRsp {
        self.inner.trade_rsp.clone().into()
    }
}

#[derive(Clone)]
pub enum FatCrabTradeRsp {
    Accept { receive_address: String },
    Reject,
}

impl From<InnerTradeRsp> for FatCrabTradeRsp {
    fn from(trade_rsp: InnerTradeRsp) -> Self {
        match trade_rsp {
            InnerTradeRsp::Accept { receive_address } => {
                FatCrabTradeRsp::Accept { receive_address }
            }
            InnerTradeRsp::Reject => FatCrabTradeRsp::Reject,
        }
    }
}

impl Into<InnerTradeRsp> for FatCrabTradeRsp {
    fn into(self) -> InnerTradeRsp {
        match self {
            FatCrabTradeRsp::Accept { receive_address } => {
                InnerTradeRsp::Accept { receive_address }
            }
            FatCrabTradeRsp::Reject => InnerTradeRsp::Reject,
        }
    }
}
