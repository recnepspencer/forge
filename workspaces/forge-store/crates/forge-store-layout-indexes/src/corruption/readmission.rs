use crate::execution::{S8AccessPathCounterSnapshot, S8AccessPathKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8CorruptionReadmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutReadmissionWitness {
    path_kind: S8AccessPathKind,
    planned: S8AccessPathCounterSnapshot,
}

impl S8LayoutReadmissionWitness {
    pub(crate) const fn new(
        path_kind: S8AccessPathKind,
        planned: S8AccessPathCounterSnapshot,
    ) -> Self {
        Self { path_kind, planned }
    }

    pub const fn path_kind(&self) -> S8AccessPathKind {
        self.path_kind
    }

    pub const fn planned(&self) -> S8AccessPathCounterSnapshot {
        self.planned
    }
}
