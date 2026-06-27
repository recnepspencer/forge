#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupIndexDisposalPostureKind {
    DestroyAndRebuildRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexDisposalPosture {
    kind: EvidenceLookupIndexDisposalPostureKind,
}

impl EvidenceLookupIndexDisposalPosture {
    pub(crate) const fn destroy_and_rebuild_required() -> Self {
        Self {
            kind: EvidenceLookupIndexDisposalPostureKind::DestroyAndRebuildRequired,
        }
    }

    pub const fn kind(&self) -> EvidenceLookupIndexDisposalPostureKind {
        self.kind
    }
}
