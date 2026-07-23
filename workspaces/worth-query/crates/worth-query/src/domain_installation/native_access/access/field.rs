use crate::projection_consumption::{ConsumedFieldValueFact, ConsumedNativeValueView};

use super::WorthQueryNativeAccessCounters;

#[derive(Debug)]
pub struct WorthQueryNativeFieldAccess<'a> {
    pub(super) fact: &'a ConsumedFieldValueFact,
    pub(super) counters: WorthQueryNativeAccessCounters,
}

impl<'a> WorthQueryNativeFieldAccess<'a> {
    pub fn fact(&self) -> &'a ConsumedFieldValueFact {
        self.fact
    }

    pub fn value(&self) -> ConsumedNativeValueView<'a> {
        self.fact.native_value()
    }

    pub fn counters(&self) -> WorthQueryNativeAccessCounters {
        self.counters
    }
}
