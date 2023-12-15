use fatcrab_trading::maker::{FatCrabMakerAccessEnum as InnerMaker, FatCrabMakerAccess, MakerBuy, MakerSell};

pub struct FatCrabMaker {
    inner: InnerMaker,
}

impl FatCrabMaker {
    pub(crate) fn new_with_buy_order(maker: FatCrabMakerAccess<MakerBuy>) -> Self {
        Self {
            inner: InnerMaker::Buy(maker),
        }
    }

    pub(crate) fn new_with_sell_order(maker: FatCrabMakerAccess<MakerSell>) -> Self {
        Self {
            inner: InnerMaker::Sell(maker),
        }
    }
}