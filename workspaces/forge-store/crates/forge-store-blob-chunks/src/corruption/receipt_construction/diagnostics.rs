use crate::{BlobChunkQuarantine, BlobDamageCase};

use super::repair_capability::{
    classify_repair_capability_from_quarantine, BlobQuarantineRepairCapability,
};

/// Diagnostics and repair/readmission capability — not ordinary blob read authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobQuarantineDiagnostics {
    damage_case: BlobDamageCase,
    quarantine: BlobChunkQuarantine,
    repair_capability: BlobQuarantineRepairCapability,
}

pub(crate) fn construct_quarantine_diagnostics(
    quarantine: BlobChunkQuarantine,
    damage_case: BlobDamageCase,
) -> BlobQuarantineDiagnostics {
    let repair_capability = classify_repair_capability_from_quarantine(&quarantine);
    BlobQuarantineDiagnostics {
        damage_case,
        quarantine,
        repair_capability,
    }
}

impl BlobQuarantineDiagnostics {
    pub const fn damage_case(&self) -> BlobDamageCase {
        self.damage_case
    }

    pub const fn quarantine(&self) -> &BlobChunkQuarantine {
        &self.quarantine
    }

    pub const fn repair_capability(&self) -> BlobQuarantineRepairCapability {
        self.repair_capability
    }
}
