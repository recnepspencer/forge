use forge_store_physical_integrity::QuarantineHandoffPosture;

use forge_store_contracts::CorruptionHandoffDamageCase;

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
            QuarantineHandoffPosture::RecoveryOwnerRequired
            | QuarantineHandoffPosture::RepairOwnerRequired => {
                RecoveryCorruptionRepairCapability::ClassifyGenerationPosture
            }
            QuarantineHandoffPosture::AuditRetentionOwnerRequired
            | QuarantineHandoffPosture::RootChangeRevalidationRequired => {
                RecoveryCorruptionRepairCapability::NoReadAuthority
            }
        },
    }
}
