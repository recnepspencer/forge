use crate::fact_contract::UiProducedFactFamily;
use crate::graph::UiGraphFactLookupReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAffectedFactLookup {
    fact_ordinal: usize,
    fact_family: UiProducedFactFamily,
    predecessor: UiGraphFactLookupReceipt,
    candidate: UiGraphFactLookupReceipt,
}

impl UiAffectedFactLookup {
    pub(crate) const fn new(
        fact_ordinal: usize,
        fact_family: UiProducedFactFamily,
        predecessor: UiGraphFactLookupReceipt,
        candidate: UiGraphFactLookupReceipt,
    ) -> Self {
        Self {
            fact_ordinal,
            fact_family,
            predecessor,
            candidate,
        }
    }

    pub const fn fact_ordinal(&self) -> usize {
        self.fact_ordinal
    }

    pub const fn fact_family(&self) -> UiProducedFactFamily {
        self.fact_family
    }

    pub const fn predecessor(&self) -> &UiGraphFactLookupReceipt {
        &self.predecessor
    }

    pub const fn candidate(&self) -> &UiGraphFactLookupReceipt {
        &self.candidate
    }
}
