//! S.8 courtroom input minted from observed heavy-blob execution evidence.

#[cfg(feature = "certification-test-authority")]
use crate::certification_test_authority::{
    execute_blob_harness, BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessExecutionInput, BlobHarnessFailurePoint,
    BlobHarnessPlacementClass, BlobHarnessSecurityScopeClass, BlobHarnessSizeClass,
};
use crate::HeavyBlobFixtureExecutionEvidence;
#[cfg(feature = "certification-test-authority")]
use forge_store_budgets::BlobHarnessEnvelopeProfile;
use forge_store_contracts::{
    S8RuntimeCase, S8RuntimeExactCounterEvidence, S8RuntimeExecutionIdentity, S8RuntimeOutcome,
    S8RuntimeOwnerFact, S8RuntimeScanPosture,
};
use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8BlobRuntimeReceipt {
    execution: HeavyBlobFixtureExecutionEvidence,
    strategy: S8LayoutStrategyFamily,
    fact: S8RuntimeOwnerFact,
}

impl S8BlobRuntimeReceipt {
    fn from_heavy_fixture_execution(
        execution: HeavyBlobFixtureExecutionEvidence,
        case: S8RuntimeCase,
        scan_posture: S8RuntimeScanPosture,
    ) -> Self {
        Self {
            execution,
            strategy: S8LayoutStrategyFamily::ChunkTree,
            fact: owner_fact(case, scan_posture),
        }
    }

    pub const fn execution(&self) -> &HeavyBlobFixtureExecutionEvidence {
        &self.execution
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

impl HeavyBlobFixtureExecutionEvidence {
    /// Emits the S.8 receipt from observed heavy-fixture execution evidence.
    pub fn into_s8_runtime_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::Success,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_unsupported_shape_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::UnsupportedShapeDenial,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_stale_rebind_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::StaleRebind,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_derived_corruption_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::CorruptDerived,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_authority_corruption_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::CorruptAuthority,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_rebuild_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::Rebuild,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_hidden_scan_denial_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::HiddenScanDenial,
            S8RuntimeScanPosture::FullStoreDenied,
        )
    }

    pub fn into_s8_migration_rollback_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::MigrationRollback,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_readmission_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::Readmission,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_cost_envelope_receipt(self) -> S8BlobRuntimeReceipt {
        S8BlobRuntimeReceipt::from_heavy_fixture_execution(
            self,
            S8RuntimeCase::CostEnvelope,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }
}

#[cfg(feature = "certification-test-authority")]
pub fn s8_blob_runtime_receipt_for_certification_test(case: S8RuntimeCase) -> S8BlobRuntimeReceipt {
    let topology = BlobHarnessChunkTopology::from_classes(
        BlobHarnessSizeClass::LocalDeterministic,
        BlobHarnessChunkSizeClass::Fixed1MiB,
    )
    .expect("phase33 blob topology should admit");
    let witness = execute_blob_harness(
        BlobHarnessExecutionInput::new(
            BlobHarnessEnvelopeProfile::Local,
            BlobHarnessSizeClass::LocalDeterministic,
            BlobHarnessPlacementClass::StoreLocal,
            BlobHarnessSecurityScopeClass::ScopePreserving,
            BlobHarnessAccessMode::ReadOnlyReplay,
            BlobHarnessFailurePoint::NoFaultSeed,
            BlobHarnessActorMix::IngestReadVerify,
            topology,
        )
        .with_heavy_temp_file_materialization(),
    );
    let execution = witness
        .heavy_fixture_evidence()
        .cloned()
        .expect("phase33 blob execution should include heavy fixture evidence");
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

const fn owner_fact(case: S8RuntimeCase, scan_posture: S8RuntimeScanPosture) -> S8RuntimeOwnerFact {
    S8RuntimeOwnerFact::new(
        S8RuntimeExecutionIdentity::from_owner_seed(0x5088_2001),
        case,
        scan_posture,
        S8RuntimeExactCounterEvidence::new(1, 1),
    )
}
