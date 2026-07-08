#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PlannedVsObservedCounterReceipt {
    path_kind: crate::execution::S8AccessPathKind,
    planned: crate::execution::S8AccessPathCounterSnapshot,
    observed: crate::execution::S8AccessPathCounterSnapshot,
}

impl S8PlannedVsObservedCounterReceipt {
    pub(crate) const fn new(
        path_kind: crate::execution::S8AccessPathKind,
        planned: crate::execution::S8AccessPathCounterSnapshot,
        observed: crate::execution::S8AccessPathCounterSnapshot,
    ) -> Self {
        Self {
            path_kind,
            planned,
            observed,
        }
    }

    pub const fn planned(self) -> crate::execution::S8AccessPathCounterSnapshot {
        self.planned
    }

    pub const fn observed(self) -> crate::execution::S8AccessPathCounterSnapshot {
        self.observed
    }

    pub const fn path_kind(self) -> crate::execution::S8AccessPathKind {
        self.path_kind
    }

    pub fn parity_holds(self) -> bool {
        self.planned() == self.observed()
    }
}
