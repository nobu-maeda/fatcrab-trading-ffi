use fatcrab_trading::taker::{FatCrabTakerAccessEnum as InnerTaker, FatCrabTakerAccess, TakerBuy, TakerSell};

pub struct FatCrabTaker {
    inner: InnerTaker,
}

impl FatCrabTaker {
    pub(crate) fn new_with_buy_order(taker: FatCrabTakerAccess<TakerBuy>) -> Self {
        Self {
            inner: InnerTaker::Buy(taker),
        }
    }

    pub(crate) fn new_with_sell_order(taker: FatCrabTakerAccess<TakerSell>) -> Self {
        Self {
            inner: InnerTaker::Sell(taker),
        }
    }
}