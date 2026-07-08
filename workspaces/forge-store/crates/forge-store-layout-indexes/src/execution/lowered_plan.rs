use crate::execution::{S8AccessPathCounterSnapshot, S8AccessPathKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8LoweredAccessPlan {
    path_kind: S8AccessPathKind,
    planned: S8AccessPathCounterSnapshot,
}

impl S8LoweredAccessPlan {
    pub(crate) const fn new(
        path_kind: S8AccessPathKind,
        planned: S8AccessPathCounterSnapshot,
    ) -> Self {
        Self { path_kind, planned }
    }

    pub(crate) const fn exact_foreground_read() -> Self {
        Self::new(
            S8AccessPathKind::ExactForegroundRead,
            S8AccessPathCounterSnapshot::new(1, 0, 0, 0, 0),
        )
    }

    pub(crate) const fn readmission_boundary() -> Self {
        Self::new(
            S8AccessPathKind::ReadmissionBoundary,
            S8AccessPathCounterSnapshot::new(0, 0, 0, 0, 1),
        )
    }

    pub(crate) const fn path_kind(self) -> S8AccessPathKind {
        self.path_kind
    }

    pub(crate) const fn planned(self) -> S8AccessPathCounterSnapshot {
        self.planned
    }
}
