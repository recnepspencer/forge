use crate::{BlobChunkQuarantine, BlobCorruptionImportReadmission, BlobQuarantineLifecycleState};

/// Next repair/readmission capability — not ordinary blob read authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobQuarantineRepairCapability {
    ClassifyGenerationPosture,
    AdmitImportReadmission(BlobCorruptionImportReadmission),
    NoReadAuthority,
}

pub(crate) fn classify_repair_capability_from_quarantine(
    quarantine: &BlobChunkQuarantine,
) -> BlobQuarantineRepairCapability {
    match quarantine.state() {
        BlobQuarantineLifecycleState::ImportCorrupt => {
            BlobQuarantineRepairCapability::AdmitImportReadmission(BlobCorruptionImportReadmission)
        }
        BlobQuarantineLifecycleState::Quarantined
        | BlobQuarantineLifecycleState::RebuildableDerived
        | BlobQuarantineLifecycleState::RepairRequiredAuthoritative
        | BlobQuarantineLifecycleState::RestoreRequiredAuthoritative
        | BlobQuarantineLifecycleState::DegradedTruthAuthoritative => {
            BlobQuarantineRepairCapability::ClassifyGenerationPosture
        }
        _ => BlobQuarantineRepairCapability::NoReadAuthority,
    }
}
