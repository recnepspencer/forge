use super::{S8AccessPathCounterSnapshot, S8AccessPathKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessPathAmplificationReceipt {
    path_kind: S8AccessPathKind,
    observed: S8AccessPathCounterSnapshot,
}

impl S8AccessPathAmplificationReceipt {
    pub(crate) const fn new(
        path_kind: S8AccessPathKind,
        observed: S8AccessPathCounterSnapshot,
    ) -> Self {
        Self {
            path_kind,
            observed,
        }
    }

    pub const fn path_kind(self) -> S8AccessPathKind {
        self.path_kind
    }

    pub const fn observed(self) -> S8AccessPathCounterSnapshot {
        self.observed
    }

    pub const fn page_touches(self) -> u16 {
        self.observed.page_touches()
    }

    pub const fn index_probes(self) -> u16 {
        self.observed.index_probes()
    }

    pub const fn read_amplification(self) -> u16 {
        self.observed.read_amplification()
    }

    pub const fn write_amplification(self) -> u16 {
        self.observed.write_amplification()
    }
}
