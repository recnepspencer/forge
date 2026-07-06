use crate::{PreDecodePhysicalDenial, PreDecodePhysicalDenialKind, QuarantineHandoffPosture, QuarantineRecord};

/// Classify physical damage before logical decode for cross-crate handoff.
pub fn classify_physical_damage_for_handoff(
    denial: &PreDecodePhysicalDenial,
) -> PreDecodePhysicalDenialKind {
    denial.kind()
}

/// Quarantine handoff receipts expose repair posture — not decode authority.
pub const fn quarantine_handoff_posture(record: &QuarantineRecord) -> QuarantineHandoffPosture {
    record.handoff_posture()
}