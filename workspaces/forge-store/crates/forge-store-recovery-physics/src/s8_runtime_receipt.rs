//! S.8 courtroom input minted from an already-executed recovery facade path.

#[cfg(feature = "certification-test-authority")]
use crate::source_precedence::RecoverySourceReplayBasis;
use crate::FreshRuntimeRecoveryExecution;
#[cfg(feature = "certification-test-authority")]
use crate::{
    PersistedRecoveryArtifactDigest, RecoveredPhysicalState, RecoveryCounterSnapshot,
    RecoveryProfileId, S4CheckpointManifestMaterialization, S4CheckpointPageImageMaterialization,
    S4PersistedRecoveryArtifactMaterialization, S4WalRedoFrameMaterialization,
};
use forge_store_contracts::{
    S8RuntimeCase, S8RuntimeExactCounterEvidence, S8RuntimeExecutionIdentity, S8RuntimeOutcome,
    S8RuntimeOwnerFact, S8RuntimeScanPosture,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RecoveryRuntimeStrategy {
    AppendLog,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8RecoveryRuntimeReceipt {
    execution: FreshRuntimeRecoveryExecution,
    strategy: S8RecoveryRuntimeStrategy,
    fact: S8RuntimeOwnerFact,
}

impl S8RecoveryRuntimeReceipt {
    fn from_fresh_runtime_execution(
        execution: FreshRuntimeRecoveryExecution,
        case: S8RuntimeCase,
        scan_posture: S8RuntimeScanPosture,
    ) -> Self {
        Self {
            execution,
            strategy: S8RecoveryRuntimeStrategy::AppendLog,
            fact: owner_fact(case, scan_posture),
        }
    }

    pub const fn execution(&self) -> &FreshRuntimeRecoveryExecution {
        &self.execution
    }
    pub const fn strategy(&self) -> S8RecoveryRuntimeStrategy {
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

impl FreshRuntimeRecoveryExecution {
    /// Emits the S.8 receipt from the recovery owner's completed runtime path.
    pub fn into_s8_runtime_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::Success,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_unsupported_shape_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::UnsupportedShapeDenial,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_stale_rebind_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::StaleRebind,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_derived_corruption_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::CorruptDerived,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_authority_corruption_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::CorruptAuthority,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_rebuild_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::Rebuild,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_migration_rollback_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::MigrationRollback,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_hidden_scan_denial_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::HiddenScanDenial,
            S8RuntimeScanPosture::FullStoreDenied,
        )
    }

    pub fn into_s8_readmission_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::Readmission,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_cost_envelope_receipt(self) -> S8RecoveryRuntimeReceipt {
        S8RecoveryRuntimeReceipt::from_fresh_runtime_execution(
            self,
            S8RuntimeCase::CostEnvelope,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }
}

#[cfg(feature = "certification-test-authority")]
pub fn s8_recovery_runtime_receipt_for_certification_test(
    case: S8RuntimeCase,
) -> S8RecoveryRuntimeReceipt {
    let execution = certification_recovery_execution(case);
    match case {
        S8RuntimeCase::Success => execution.into_s8_runtime_receipt(),
        S8RuntimeCase::UnsupportedShapeDenial => execution.into_s8_unsupported_shape_receipt(),
        S8RuntimeCase::StaleRebind => execution.into_s8_stale_rebind_receipt(),
        S8RuntimeCase::CorruptDerived => execution.into_s8_derived_corruption_receipt(),
        S8RuntimeCase::CorruptAuthority => execution.into_s8_authority_corruption_receipt(),
        S8RuntimeCase::Rebuild => execution.into_s8_rebuild_receipt(),
        S8RuntimeCase::MigrationRollback => execution.into_s8_migration_rollback_receipt(),
        S8RuntimeCase::HiddenScanDenial => execution.into_s8_hidden_scan_denial_receipt(),
        S8RuntimeCase::Readmission => execution.into_s8_readmission_receipt(),
        S8RuntimeCase::CostEnvelope => execution.into_s8_cost_envelope_receipt(),
    }
}

#[cfg(feature = "certification-test-authority")]
fn certification_recovery_execution(case: S8RuntimeCase) -> FreshRuntimeRecoveryExecution {
    let profile = RecoveryProfileId::strict_s4();
    let artifacts = S4PersistedRecoveryArtifactMaterialization::new(
        "s4",
        "phase33-runtime-recovery",
        profile.clone(),
        S4CheckpointManifestMaterialization::new(
            format!("phase33.{case:?}.checkpoint"),
            format!("phase33-root-{case:?}"),
            41,
            "phase33-runtime-source",
            1,
            4096,
            1,
            4096,
            1,
        ),
        S4WalRedoFrameMaterialization::new(
            format!("phase33.{case:?}.wal"),
            41,
            1,
            format!("phase33-op-{case:?}"),
            format!("phase33-idempotence-{case:?}"),
        ),
        S4CheckpointPageImageMaterialization::new(
            format!("phase33.{case:?}.page"),
            1,
            1,
            41,
            format!("phase33-page-digest-{case:?}"),
        ),
    )
    .materialize()
    .expect("phase33 recovery artifacts should materialize");
    let digest = PersistedRecoveryArtifactDigest::from_artifacts(&artifacts);
    let recovered_state = RecoveredPhysicalState::from_projected_parts(
        format!("phase33-recovered-root-{case:?}"),
        None,
        RecoverySourceReplayBasis::empty(),
        format!("phase33-source-decision-{case:?}"),
        1,
        0,
    );
    let counters =
        RecoveryCounterSnapshot::from_offline_verifier(1, 0, 1, 1, 1, 4096, 1, 4096, 1, 0, 0);
    FreshRuntimeRecoveryExecution::from_certification_runtime_evidence(
        digest,
        profile,
        recovered_state,
        counters,
        1,
        0,
    )
}

const fn owner_fact(case: S8RuntimeCase, scan_posture: S8RuntimeScanPosture) -> S8RuntimeOwnerFact {
    S8RuntimeOwnerFact::new(
        S8RuntimeExecutionIdentity::from_owner_seed(0x5088_1001),
        case,
        scan_posture,
        S8RuntimeExactCounterEvidence::new(1, 1),
    )
}
