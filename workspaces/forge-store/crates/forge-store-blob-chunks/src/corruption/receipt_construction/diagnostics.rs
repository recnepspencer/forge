use crate::{BlobChunkQuarantine, BlobDamageCase};

/// Diagnostics and repair/readmission capability — not ordinary blob read authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobQuarantineDiagnostics {
    damage_case: BlobDamageCase,
    quarantine: BlobChunkQuarantine,
}

pub(crate) fn construct_quarantine_diagnostics(
    quarantine: BlobChunkQuarantine,
    damage_case: BlobDamageCase,
) -> BlobQuarantineDiagnostics {
    BlobQuarantineDiagnostics {
        damage_case,
        quarantine,
    }
}

impl BlobQuarantineDiagnostics {
    pub const fn damage_case(&self) -> BlobDamageCase {
        self.damage_case
    }

    pub const fn quarantine(&self) -> &BlobChunkQuarantine {
        &self.quarantine
    }
}