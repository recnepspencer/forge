use crate::{
    PreDecodePhysicalDenial, PreDecodePhysicalDenialKind, QuarantineHandoffPosture,
    QuarantineRecord,
};

/// Physical damage evidence for cross-crate handoff before logical decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalDamageHandoffEvidence {
    denial_kind: PreDecodePhysicalDenialKind,
}

impl PhysicalDamageHandoffEvidence {
    pub const fn denial_kind(self) -> PreDecodePhysicalDenialKind {
        self.denial_kind
    }
}

/// Classify physical damage before logical decode for cross-crate handoff.
pub fn classify_physical_damage_for_handoff(
    denial: &PreDecodePhysicalDenial,
) -> PhysicalDamageHandoffEvidence {
    PhysicalDamageHandoffEvidence {
        denial_kind: denial.kind(),
    }
}

/// Quarantine handoff receipts expose repair posture — not decode authority.
pub const fn quarantine_handoff_posture(record: &QuarantineRecord) -> QuarantineHandoffPosture {
    record.handoff_posture()
}
