use super::{UiGraphFactIndexBasis, UiGraphFactIndexEntry};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphFactLookupCost {
    index_probes: usize,
    contract_checks: usize,
    selected_consumers: usize,
}

impl UiGraphFactLookupCost {
    pub(crate) const fn exact(selected_consumers: usize) -> Self {
        Self {
            index_probes: 1,
            contract_checks: selected_consumers,
            selected_consumers,
        }
    }

    pub const fn index_probes(self) -> usize {
        self.index_probes
    }

    pub const fn contract_checks(self) -> usize {
        self.contract_checks
    }

    pub const fn selected_consumers(self) -> usize {
        self.selected_consumers
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphFactLookupReceipt {
    basis: UiGraphFactIndexBasis,
    entries: Box<[UiGraphFactIndexEntry]>,
    cost: UiGraphFactLookupCost,
}

impl UiGraphFactLookupReceipt {
    pub(crate) fn new(basis: UiGraphFactIndexBasis, entries: Box<[UiGraphFactIndexEntry]>) -> Self {
        let cost = UiGraphFactLookupCost::exact(entries.len());
        Self {
            basis,
            entries,
            cost,
        }
    }

    pub const fn basis(&self) -> UiGraphFactIndexBasis {
        self.basis
    }

    pub fn entries(&self) -> &[UiGraphFactIndexEntry] {
        &self.entries
    }

    pub const fn cost(&self) -> UiGraphFactLookupCost {
        self.cost
    }
}
