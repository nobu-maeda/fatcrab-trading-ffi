use fatcrab_trading::taker::{FatCrabTakerAccess, TakerBuy, TakerSell};

pub struct FatCrabBuyTaker {
    inner: FatCrabTakerAccess<TakerBuy>,
}

pub struct FatCrabSellTaker {
    inner: FatCrabTakerAccess<TakerSell>,
}

impl FatCrabBuyTaker {
    pub(crate) fn new(taker: FatCrabTakerAccess<TakerBuy>) -> Self {
        Self {
            inner: taker,
        }
    }
}

impl FatCrabSellTaker {
    pub(crate) fn new(taker: FatCrabTakerAccess<TakerSell>) -> Self {
        Self {
            inner: taker,
        }
    }
}