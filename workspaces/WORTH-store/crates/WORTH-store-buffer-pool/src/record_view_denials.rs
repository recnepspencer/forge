use crate::{RecordCopyCounterSnapshot, ResidentFrameDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordViewDenialKind {
    ResidentLeaseDenied,
    PhysicalReferenceMismatch,
    HeaderWitnessMismatch,
    ResidentPayloadLengthMismatch,
    ProfileForbidsMaterialization,
    SemanticDomainMaterializationRejected,
    MutableViewRequiresExclusiveLease,
    AllocationReceiptKindMismatch,
    AllocationReceiptByteMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordViewDenial {
    kind: RecordViewDenialKind,
    counters: RecordCopyCounterSnapshot,
    resident_denial: Option<ResidentFrameDenial>,
}

impl RecordViewDenial {
    pub(crate) const fn new(
        kind: RecordViewDenialKind,
        counters: RecordCopyCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            counters,
            resident_denial: None,
        }
    }

    pub(crate) const fn from_resident(
        denial: ResidentFrameDenial,
        counters: RecordCopyCounterSnapshot,
    ) -> Self {
        Self {
            kind: RecordViewDenialKind::ResidentLeaseDenied,
            counters,
            resident_denial: Some(denial),
        }
    }

    pub const fn kind(self) -> RecordViewDenialKind {
        self.kind
    }

    pub const fn counters(self) -> RecordCopyCounterSnapshot {
        self.counters
    }

    pub const fn resident_denial(self) -> Option<ResidentFrameDenial> {
        self.resident_denial
    }
}
