use super::super::capability::BackendForbiddenClaimKind;
use super::super::milestones::{
    MilestonePhysicalStatusRow, S0PhysicalStatus, SemanticPhysicalClaimFamily,
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum DeferredPhysicalGuaranteeCategory {
    PageSegmentExtentSubstrate,
    MemoryAllocationBoundedness,
    PageFrameChunkIntegrityAndCorruptionLocalization,
    WalCheckpointLsnRecoveryPhysics,
    PhysicalReadStabilityDuringMaintenance,
    HardwareAwareIoAndForegroundQos,
    NativeBlobObjectChunkStore,
    IndexLayoutAccessPathDiscipline,
    FormalCrashConcurrencyModels,
    BackupPitrRepairAndForensics,
    SecurityTenantBoundariesKeysAndAuditability,
    PhysicalDatabaseCertificationAndPerformance,
}

impl DeferredPhysicalGuaranteeCategory {
    pub(super) fn minimum_required_sequences(self) -> &'static [&'static str] {
        match self {
            Self::PageSegmentExtentSubstrate => &["S1"],
            Self::MemoryAllocationBoundedness => &["S2"],
            Self::PageFrameChunkIntegrityAndCorruptionLocalization => &["S3"],
            Self::WalCheckpointLsnRecoveryPhysics => &["S4"],
            Self::PhysicalReadStabilityDuringMaintenance => &["S5"],
            Self::HardwareAwareIoAndForegroundQos => &["S6"],
            Self::NativeBlobObjectChunkStore => &["S7"],
            Self::IndexLayoutAccessPathDiscipline => &["S8"],
            Self::FormalCrashConcurrencyModels => &["S9"],
            Self::BackupPitrRepairAndForensics => &["S10"],
            Self::SecurityTenantBoundariesKeysAndAuditability => &["S11"],
            Self::PhysicalDatabaseCertificationAndPerformance => &["S12"],
        }
    }

    pub(super) fn missing_proof_summary(self) -> &'static str {
        match self {
            Self::PageSegmentExtentSubstrate => {
                "page/segment/extent substrate proof remains unearned"
            }
            Self::MemoryAllocationBoundedness => {
                "memory and allocation boundedness proof remains unearned"
            }
            Self::PageFrameChunkIntegrityAndCorruptionLocalization => {
                "page/frame/chunk integrity and corruption localization proof remains unearned"
            }
            Self::WalCheckpointLsnRecoveryPhysics => {
                "WAL/checkpoint/LSN recovery physics proof remains unearned"
            }
            Self::PhysicalReadStabilityDuringMaintenance => {
                "physical read stability during maintenance remains unearned"
            }
            Self::HardwareAwareIoAndForegroundQos => {
                "hardware-aware I/O and foreground QoS proof remains unearned"
            }
            Self::NativeBlobObjectChunkStore => {
                "native blob/object chunk store proof remains unearned"
            }
            Self::IndexLayoutAccessPathDiscipline => {
                "index/layout/access-path discipline proof remains unearned"
            }
            Self::FormalCrashConcurrencyModels => {
                "formal crash/concurrency model proof remains unearned"
            }
            Self::BackupPitrRepairAndForensics => {
                "backup, PITR, repair, and forensics proof remains unearned"
            }
            Self::SecurityTenantBoundariesKeysAndAuditability => {
                "security, tenant boundary, key, and auditability proof remains unearned"
            }
            Self::PhysicalDatabaseCertificationAndPerformance => {
                "physical database certification and performance proof remains unearned"
            }
        }
    }
}

pub(super) fn deferred_category_from_claim_family(
    family: SemanticPhysicalClaimFamily,
    row: &MilestonePhysicalStatusRow,
) -> Option<DeferredPhysicalGuaranteeCategory> {
    let status = row.physical_status_for_claim_family(family);
    if matches!(
        status,
        S0PhysicalStatus::FoundationBacked
            | S0PhysicalStatus::PlatformGrade
            | S0PhysicalStatus::NotApplicable
    ) {
        return None;
    }
    match family {
        SemanticPhysicalClaimFamily::PhysicalSubstrate => {
            Some(DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate)
        }
        SemanticPhysicalClaimFamily::PhysicalBoundedness => {
            Some(DeferredPhysicalGuaranteeCategory::MemoryAllocationBoundedness)
        }
        SemanticPhysicalClaimFamily::PhysicalIntegrity => Some(
            DeferredPhysicalGuaranteeCategory::PageFrameChunkIntegrityAndCorruptionLocalization,
        ),
        SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics => {
            Some(DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics)
        }
        SemanticPhysicalClaimFamily::PhysicalIsolation => {
            Some(DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance)
        }
        SemanticPhysicalClaimFamily::PhysicalIo => {
            Some(DeferredPhysicalGuaranteeCategory::HardwareAwareIoAndForegroundQos)
        }
        SemanticPhysicalClaimFamily::PhysicalOperationalSafety => {
            Some(DeferredPhysicalGuaranteeCategory::BackupPitrRepairAndForensics)
        }
        SemanticPhysicalClaimFamily::PhysicalSecurity => {
            Some(DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability)
        }
        _ => None,
    }
}

pub(super) fn supplementary_category_from_forbidden_claim_kind(
    kind: BackendForbiddenClaimKind,
) -> Option<DeferredPhysicalGuaranteeCategory> {
    match kind {
        BackendForbiddenClaimKind::PlatformGradeDurability
        | BackendForbiddenClaimKind::PhysicalQueryPerformance => {
            Some(DeferredPhysicalGuaranteeCategory::PhysicalDatabaseCertificationAndPerformance)
        }
        BackendForbiddenClaimKind::PlatformGradeRecovery => {
            Some(DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics)
        }
        BackendForbiddenClaimKind::PlatformGradeConcurrency => {
            Some(DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance)
        }
        BackendForbiddenClaimKind::PlatformGradeMultiTenantIsolation => {
            Some(DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability)
        }
        BackendForbiddenClaimKind::PhysicalPersistence => {
            Some(DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate)
        }
    }
}

pub(super) fn current_status_for_category(
    row: &MilestonePhysicalStatusRow,
    category: DeferredPhysicalGuaranteeCategory,
) -> S0PhysicalStatus {
    match category {
        DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalSubstrate)
        }
        DeferredPhysicalGuaranteeCategory::MemoryAllocationBoundedness => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalBoundedness)
        }
        DeferredPhysicalGuaranteeCategory::PageFrameChunkIntegrityAndCorruptionLocalization => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalIntegrity)
        }
        DeferredPhysicalGuaranteeCategory::WalCheckpointLsnRecoveryPhysics => row
            .physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics),
        DeferredPhysicalGuaranteeCategory::PhysicalReadStabilityDuringMaintenance => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalIsolation)
        }
        DeferredPhysicalGuaranteeCategory::HardwareAwareIoAndForegroundQos => {
            row.physical_status_for_claim_family(SemanticPhysicalClaimFamily::PhysicalIo)
        }
        DeferredPhysicalGuaranteeCategory::NativeBlobObjectChunkStore => row
            .native_blob_chunk_status()
            .unwrap_or(S0PhysicalStatus::PhysicalDebt),
        DeferredPhysicalGuaranteeCategory::IndexLayoutAccessPathDiscipline => {
            S0PhysicalStatus::PhysicalDebt
        }
        DeferredPhysicalGuaranteeCategory::FormalCrashConcurrencyModels => {
            S0PhysicalStatus::PhysicalDebt
        }
        DeferredPhysicalGuaranteeCategory::BackupPitrRepairAndForensics => row
            .operator_security_status()
            .unwrap_or(S0PhysicalStatus::PhysicalDebt),
        DeferredPhysicalGuaranteeCategory::SecurityTenantBoundariesKeysAndAuditability => row
            .operator_security_status()
            .unwrap_or(S0PhysicalStatus::PhysicalDebt),
        DeferredPhysicalGuaranteeCategory::PhysicalDatabaseCertificationAndPerformance => {
            S0PhysicalStatus::PhysicalDebt
        }
    }
}
