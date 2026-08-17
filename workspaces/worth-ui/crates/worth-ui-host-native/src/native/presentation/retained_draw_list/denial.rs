use super::{UiNativeDamageIndexDenial, UiNativeRetainedOrderDenial};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeRetainedDrawListDenial {
    AffinityMismatch,
    BaselineUnavailable,
    CommandMismatch,
    OrderMismatch,
    DamageIndex,
    CounterOverflow,
}

impl From<UiNativeDamageIndexDenial> for UiNativeRetainedDrawListDenial {
    fn from(_: UiNativeDamageIndexDenial) -> Self {
        Self::DamageIndex
    }
}

impl From<UiNativeRetainedOrderDenial> for UiNativeRetainedDrawListDenial {
    fn from(_: UiNativeRetainedOrderDenial) -> Self {
        Self::OrderMismatch
    }
}
