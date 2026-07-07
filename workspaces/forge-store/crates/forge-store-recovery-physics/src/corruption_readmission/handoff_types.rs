use forge_store_contracts::CorruptionHandoffDamageCase;

use crate::{BlobReplayAdmissionDenial, S4IntegrityHandoffDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryCorruptionReadmissionDenial {
    QuarantineHandoff(S4IntegrityHandoffDenial),
    StoreAuthority(BlobReplayAdmissionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryCorruptionRepairCapability {
    ClassifyGenerationPosture,
    AdmitImportReadmission,
    NoReadAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryCorruptionReadmissionHandoff {
    primary_damage_case: CorruptionHandoffDamageCase,
    repair_capability: RecoveryCorruptionRepairCapability,
}

impl RecoveryCorruptionReadmissionHandoff {
    pub(crate) const fn new(
        primary_damage_case: CorruptionHandoffDamageCase,
        repair_capability: RecoveryCorruptionRepairCapability,
    ) -> Self {
        Self {
            primary_damage_case,
            repair_capability,
        }
    }

    pub const fn primary_damage_case(self) -> CorruptionHandoffDamageCase {
        self.primary_damage_case
    }

    pub const fn repair_capability(self) -> RecoveryCorruptionRepairCapability {
        self.repair_capability
    }
}
