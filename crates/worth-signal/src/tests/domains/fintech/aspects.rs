use crate::facade::*;

pub(super) const PRICE: Aspect = Aspect::new(0);
pub(super) const VOL: Aspect = Aspect::new(1);
pub(super) const CURVE: Aspect = Aspect::new(2);
pub(super) const LIQUIDITY: Aspect = Aspect::new(3);
pub(super) const RISK: Aspect = Aspect::new(4);
pub(super) const ALERT: Aspect = Aspect::new(5);

pub(super) fn market_mask() -> AspectMask {
    AspectMask::from([PRICE, VOL, CURVE, LIQUIDITY])
}

pub(super) fn pricing_mask() -> AspectMask {
    AspectMask::from([PRICE, VOL, CURVE, LIQUIDITY, RISK])
}

pub(super) fn full_mask() -> AspectMask {
    AspectMask::from([PRICE, VOL, CURVE, LIQUIDITY, RISK, ALERT])
}
