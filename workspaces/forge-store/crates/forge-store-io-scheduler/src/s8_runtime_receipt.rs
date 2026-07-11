//! S.8 courtroom input minted only after queue execution reaches `Executed`.

#[cfg(feature = "certification-test-authority")]
use crate::queue_execution::{execute_ready_queue_plan, test_support, QueueExecutionOutcome};
use crate::QueueExecutedPlan;
use forge_store_contracts::{
    S8RuntimeCase, S8RuntimeExactCounterEvidence, S8RuntimeExecutionIdentity, S8RuntimeOutcome,
    S8RuntimeOwnerFact, S8RuntimeScanPosture,
};
use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;

#[derive(Debug, PartialEq, Eq)]
pub struct S8MaintenanceIoRuntimeReceipt {
    execution: QueueExecutedPlan,
    strategy: S8LayoutStrategyFamily,
    fact: S8RuntimeOwnerFact,
}

impl S8MaintenanceIoRuntimeReceipt {
    fn from_executed_queue_plan(
        execution: QueueExecutedPlan,
        case: S8RuntimeCase,
        scan_posture: S8RuntimeScanPosture,
    ) -> Self {
        Self {
            execution,
            strategy: S8LayoutStrategyFamily::StreamingCursorIndex,
            fact: owner_fact(case, scan_posture),
        }
    }

    pub const fn execution(&self) -> &QueueExecutedPlan {
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

impl QueueExecutedPlan {
    /// Emits the S.8 receipt only after the scheduler has reached `Executed`.
    pub fn into_s8_runtime_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::Success,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_unsupported_shape_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::UnsupportedShapeDenial,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }

    pub fn into_s8_stale_rebind_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::StaleRebind,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_derived_corruption_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::CorruptDerived,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_authority_corruption_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::CorruptAuthority,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_rebuild_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::Rebuild,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_migration_rollback_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::MigrationRollback,
            S8RuntimeScanPosture::RebuildBounded,
        )
    }

    pub fn into_s8_hidden_scan_denial_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::HiddenScanDenial,
            S8RuntimeScanPosture::FullStoreDenied,
        )
    }

    pub fn into_s8_readmission_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::Readmission,
            S8RuntimeScanPosture::ReadmissionBounded,
        )
    }

    pub fn into_s8_cost_envelope_receipt(self) -> S8MaintenanceIoRuntimeReceipt {
        S8MaintenanceIoRuntimeReceipt::from_executed_queue_plan(
            self,
            S8RuntimeCase::CostEnvelope,
            S8RuntimeScanPosture::OwnerBounded,
        )
    }
}

#[cfg(feature = "certification-test-authority")]
pub fn s8_maintenance_io_runtime_receipt_for_certification_test(
    case: S8RuntimeCase,
) -> S8MaintenanceIoRuntimeReceipt {
    let plan = test_support::admitted_plan();
    let scope = test_support::speculative_scope(&plan);
    let completion = test_support::completion_for_plan(&plan, 1, Some(scope), 0, None).complete();
    let outcome = execute_ready_queue_plan(plan, completion);
    let execution = match outcome {
        QueueExecutionOutcome::Executed(executed) => executed.plan,
        other => panic!("phase33 queue execution should execute: {other:?}"),
    };
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
        S8RuntimeExecutionIdentity::from_owner_seed(0x5088_3001),
        case,
        scan_posture,
        S8RuntimeExactCounterEvidence::new(1, 1),
    )
}
