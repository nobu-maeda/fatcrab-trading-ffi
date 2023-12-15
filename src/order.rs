use uuid::Uuid;

pub use fatcrab_trading::order::FatCrabOrderType;
pub use fatcrab_trading::order::FatCrabOrderEnvelope;
use fatcrab_trading::order::FatCrabOrder as InnerOrder;

#[derive(Clone)]
pub struct FatCrabOrder {
    pub order_type: FatCrabOrderType,
    pub trade_uuid: String,
    pub amount: f64, // in FC
    pub price: f64, 
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

impl FatCrabOrder {
    pub fn new(order_type: FatCrabOrderType, trade_uuid: String, amount: f64, price: f64) -> Self {
        Self {
            order_type,
            trade_uuid,
            amount,
            price,
        }
    }
}