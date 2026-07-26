use crate::runtime::WorthUiDurableStateInventoryCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDurableStateInventoryDenial {
    AmbiguousNodeReplacementPlan {
        counters: WorthUiDurableStateInventoryCounters,
    },
}
