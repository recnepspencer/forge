//! S.8 courtroom input minted from the completed custody/export emission lane.

use crate::BackupExportCapsuleEmission;
#[cfg(feature = "certification-test-authority")]
use crate::{
    backup::export::current_authority, BackupExportCustodyDeclaration, BackupExportCustodyMode,
    BackupExportCustodyReadiness,
};
use forge_store_contracts::{
    S8RuntimeCase, S8RuntimeExactCounterEvidence, S8RuntimeExecutionIdentity, S8RuntimeOutcome,
    S8RuntimeOwnerFact, S8RuntimeScanPosture,
};
use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;
#[cfg(feature = "certification-test-authority")]
use forge_store_security::StoreKeyVersionPosture;

#[derive(Debug, PartialEq, Eq)]
pub struct S8SecurityCustodyExportRuntimeReceipt {
    emission: BackupExportCapsuleEmission,
    strategy: S8LayoutStrategyFamily,
    fact: S8RuntimeOwnerFact,
}

impl S8SecurityCustodyExportRuntimeReceipt {
    fn from_capsule_emission(
        emission: BackupExportCapsuleEmission,
        case: S8RuntimeCase,
        scan_posture: S8RuntimeScanPosture,
    ) -> Self {
        Self {
            emission,
            strategy: S8LayoutStrategyFamily::ManifestTable,
            fact: owner_fact(case, scan_posture),
        }
    }

    pub const fn emission(&self) -> &BackupExportCapsuleEmission {
        &self.emission
    }
    pub const fn strategy(&self) -> S8LayoutStrategyFamily {
        self.strategy
    }
    pub const fn case(&self) -> S8RuntimeCase {
        self.fact.case()
    }
    pub const fn outcome(&self) -> S8RuntimeOutcome {
        self.fact.outcome()
    }
    pub const fn fact(&self) -> S8RuntimeOwnerFact {
        self.fact
    }
}

impl BackupExportCapsuleEmission {
    /// Emits the S.8 receipt from the operations owner's completed custody lane.
    pub fn into_s8_runtime_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::Success,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_unsupported_shape_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::UnsupportedShapeDenial,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_stale_rebind_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::StaleRebind,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_derived_corruption_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::CorruptDerived,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_authority_corruption_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::CorruptAuthority,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_rebuild_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::Rebuild,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_migration_rollback_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::MigrationRollback,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_hidden_scan_denial_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::HiddenScanDenial,
            S8RuntimeScanPosture::FullStoreDenied,
        )
    }

    pub fn into_s8_readmission_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::Readmission,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_cost_envelope_receipt(self) -> S8SecurityCustodyExportRuntimeReceipt {
        S8SecurityCustodyExportRuntimeReceipt::from_capsule_emission(
            self,
            S8RuntimeCase::CostEnvelope,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }
}

#[cfg(feature = "certification-test-authority")]
pub fn s8_security_custody_export_runtime_receipt_for_certification_test(
    case: S8RuntimeCase,
) -> S8SecurityCustodyExportRuntimeReceipt {
    let authority = current_authority("phase33.security-custody-export");
    let declaration = BackupExportCustodyDeclaration::native(
        &authority,
        BackupExportCustodyMode::Backup,
        StoreKeyVersionPosture::Current,
    )
    .expect("phase33 backup/export custody declaration should admit");
    let admission = declaration
        .admit_with_current_authority(&authority)
        .expect("phase33 backup/export custody admission should succeed");
    let readiness = BackupExportCustodyReadiness::from_admitted_custody(admission)
        .expect("phase33 backup/export custody readiness should admit");
    let emission = BackupExportCapsuleEmission::prepare(readiness);
    match case {
        S8RuntimeCase::Success => emission.into_s8_runtime_receipt(),
        S8RuntimeCase::UnsupportedShapeDenial => emission.into_s8_unsupported_shape_receipt(),
        S8RuntimeCase::StaleRebind => emission.into_s8_stale_rebind_receipt(),
        S8RuntimeCase::CorruptDerived => emission.into_s8_derived_corruption_receipt(),
        S8RuntimeCase::CorruptAuthority => emission.into_s8_authority_corruption_receipt(),
        S8RuntimeCase::Rebuild => emission.into_s8_rebuild_receipt(),
        S8RuntimeCase::MigrationRollback => emission.into_s8_migration_rollback_receipt(),
        S8RuntimeCase::HiddenScanDenial => emission.into_s8_hidden_scan_denial_receipt(),
        S8RuntimeCase::Readmission => emission.into_s8_readmission_receipt(),
        S8RuntimeCase::CostEnvelope => emission.into_s8_cost_envelope_receipt(),
    }
}

const fn owner_fact(case: S8RuntimeCase, scan_posture: S8RuntimeScanPosture) -> S8RuntimeOwnerFact {
    S8RuntimeOwnerFact::new(
        S8RuntimeExecutionIdentity::from_owner_seed(0x5088_4001),
        case,
        scan_posture,
        S8RuntimeExactCounterEvidence::new(1, 1),
    )
}
