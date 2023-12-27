use uuid::Uuid;

use fatcrab_trading::order::FatCrabOrder as InnerOrder;
use fatcrab_trading::order::FatCrabOrderEnvelope as InnerEnvelope;
pub use fatcrab_trading::order::FatCrabOrderType;

#[derive(Clone)]
pub struct FatCrabOrderEnvelope {
    inner: InnerEnvelope,
}

impl From<InnerEnvelope> for FatCrabOrderEnvelope {
    fn from(envelope: InnerEnvelope) -> Self {
        Self { inner: envelope }
    }
}

impl Into<InnerEnvelope> for FatCrabOrderEnvelope {
    fn into(self) -> InnerEnvelope {
        self.inner
    }
}

impl FatCrabOrderEnvelope {
    pub fn order(&self) -> FatCrabOrder {
        self.inner.order.clone().into()
    }
}

#[derive(Clone)]
pub struct FatCrabOrder {
    pub order_type: FatCrabOrderType,
    pub trade_uuid: String,
    pub amount: f64, // in FC
    pub price: f64,
}

impl From<InnerOrder> for FatCrabOrder {
    fn from(order: InnerOrder) -> Self {
        Self {
            order_type: order.order_type,
            trade_uuid: order.trade_uuid.to_string(),
            amount: order.amount,
            price: order.price,
        }
    }
}

impl Into<InnerOrder> for FatCrabOrder {
    fn into(self) -> InnerOrder {
        InnerOrder {
            order_type: self.order_type,
            trade_uuid: Uuid::parse_str(&self.trade_uuid).unwrap(),
            amount: self.amount,
            price: self.price,
        }
    }
}
