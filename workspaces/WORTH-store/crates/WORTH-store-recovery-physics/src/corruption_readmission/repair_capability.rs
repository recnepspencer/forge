use worth_store_physical_integrity::QuarantineHandoffPosture;

use worth_store_contracts::CorruptionHandoffDamageCase;

use crate::RecoveryCorruptionRepairCapability;

pub fn classify_recovery_repair_capability(
    damage_case: CorruptionHandoffDamageCase,
    handoff_posture: QuarantineHandoffPosture,
) -> RecoveryCorruptionRepairCapability {
    match damage_case {
        CorruptionHandoffDamageCase::CrossScopeImport => {
            RecoveryCorruptionRepairCapability::AdmitImportReadmission
        }
        CorruptionHandoffDamageCase::ChecksumMismatch
        | CorruptionHandoffDamageCase::AuthenticityFailure
        | CorruptionHandoffDamageCase::MissingChunk
        | CorruptionHandoffDamageCase::StaleGeneration => match handoff_posture {
            QuarantineHandoffPosture::S4RecoveryOwnerRequired
            | QuarantineHandoffPosture::S10RepairOwnerRequired => {
                RecoveryCorruptionRepairCapability::ClassifyGenerationPosture
            }
            QuarantineHandoffPosture::AuditRetentionOwnerRequired
            | QuarantineHandoffPosture::RootChangeRevalidationRequired => {
                RecoveryCorruptionRepairCapability::NoReadAuthority
            }
        },
    }
}
