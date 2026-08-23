use worth_store::physical_runtime::StoreRecoveryBindingSampleDenial;
use worth_store_recovery_physics::{
    PhysicalRedoPlanningDenial, RecoveryPlanCost, RecoveryPlanCostDenial, RecoveryPlanLimits,
    RecoveryPlanningCounters,
};

use crate::entry::{
    AdmittedPlatformAuthority, PhysicalRecoveryBlockEvidence, PhysicalRecoveryBlockKind,
    PhysicalRecoveryLimitDimension, PhysicalRecoveryLimitFailure, PhysicalRecoveryOutcome,
    PhysicalRecoveryPlanningDenial,
};
use crate::handoff::block_unsupported_scope;
use crate::progression::PhysicalRecoveryDiscoveryCounters;

use super::super::RecoveryCoordination;

pub(super) fn redo_block(
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
    counters: PhysicalRecoveryDiscoveryCounters,
    planning_counters: RecoveryPlanningCounters,
    limit: Option<PhysicalRecoveryLimitFailure>,
) -> PhysicalRecoveryOutcome {
    block_with_planning_attempt(
        authority,
        coordination,
        PhysicalRecoveryBlockKind::RedoPlanning,
        counters,
        planning_counters,
        "canonical-redo-plan",
        limit,
        None,
    )
}

pub(super) fn redo_denial_block(
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
    counters: PhysicalRecoveryDiscoveryCounters,
    planning_counters: RecoveryPlanningCounters,
    limit: Option<PhysicalRecoveryLimitFailure>,
    denial: PhysicalRedoPlanningDenial,
) -> PhysicalRecoveryOutcome {
    block_with_planning_attempt(
        authority,
        coordination,
        PhysicalRecoveryBlockKind::RedoPlanning,
        counters,
        planning_counters,
        "canonical-redo-plan",
        limit,
        Some(PhysicalRecoveryPlanningDenial::Redo(denial)),
    )
}

pub(super) fn cost_denial_block(
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
    counters: PhysicalRecoveryDiscoveryCounters,
    planning_counters: RecoveryPlanningCounters,
    denial: RecoveryPlanCostDenial,
    limit: PhysicalRecoveryLimitFailure,
) -> PhysicalRecoveryOutcome {
    block_with_planning_attempt(
        authority,
        coordination,
        PhysicalRecoveryBlockKind::RedoPlanning,
        counters,
        planning_counters,
        "recovery-plan-cost",
        Some(limit),
        Some(PhysicalRecoveryPlanningDenial::Cost(denial)),
    )
}

pub(super) fn plan_cost_limit(
    denial: RecoveryPlanCostDenial,
    limits: RecoveryPlanLimits,
    cost: RecoveryPlanCost,
) -> PhysicalRecoveryLimitFailure {
    let (dimension, observed, admitted) = match denial {
        RecoveryPlanCostDenial::RedoTargets => (
            PhysicalRecoveryLimitDimension::RedoTargets,
            cost.redo_targets(),
            limits.redo_targets(),
        ),
        RecoveryPlanCostDenial::RedoBytes => (
            PhysicalRecoveryLimitDimension::RedoBytes,
            cost.redo_bytes(),
            limits.redo_bytes(),
        ),
        RecoveryPlanCostDenial::DistinctTargets => (
            PhysicalRecoveryLimitDimension::DistinctPagesAndExtents,
            cost.distinct_targets(),
            limits.distinct_targets(),
        ),
        RecoveryPlanCostDenial::OperationBindings => (
            PhysicalRecoveryLimitDimension::OperationBindings,
            cost.operation_bindings(),
            limits.operation_bindings(),
        ),
        RecoveryPlanCostDenial::ObservationBytes => (
            PhysicalRecoveryLimitDimension::ObservationBytes,
            cost.observation_bytes(),
            limits.observation_bytes(),
        ),
        RecoveryPlanCostDenial::StagingBytes => (
            PhysicalRecoveryLimitDimension::StagingBytes,
            cost.staging_bytes(),
            limits.staging_bytes(),
        ),
        RecoveryPlanCostDenial::RecoveryMemoryBytes => (
            PhysicalRecoveryLimitDimension::RecoveryMemoryBytes,
            cost.peak_recovery_bytes(),
            limits.recovery_memory_bytes(),
        ),
        RecoveryPlanCostDenial::DirtyFrames => (
            PhysicalRecoveryLimitDimension::DirtyFrames,
            cost.dirty_frames(),
            limits.dirty_frames(),
        ),
    };
    PhysicalRecoveryLimitFailure {
        dimension,
        observed,
        admitted,
    }
}

pub(super) fn block(
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
    kind: PhysicalRecoveryBlockKind,
    counters: PhysicalRecoveryDiscoveryCounters,
    artifact: &str,
    limit: Option<PhysicalRecoveryLimitFailure>,
) -> PhysicalRecoveryOutcome {
    block_unsupported_scope(
        authority,
        coordination,
        kind,
        PhysicalRecoveryBlockEvidence {
            counters,
            planning_counters: Some(RecoveryPlanningCounters::default()),
            limit,
            artifact: Some(artifact.to_owned()),
            ..Default::default()
        },
    )
}

pub(super) fn block_with_planning_attempt_denial(
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
    kind: PhysicalRecoveryBlockKind,
    counters: PhysicalRecoveryDiscoveryCounters,
    planning_counters: RecoveryPlanningCounters,
    artifact: &str,
    limit: Option<PhysicalRecoveryLimitFailure>,
    planning_denial: PhysicalRecoveryPlanningDenial,
) -> PhysicalRecoveryOutcome {
    block_with_planning_attempt(
        authority,
        coordination,
        kind,
        counters,
        planning_counters,
        artifact,
        limit,
        Some(planning_denial),
    )
}

fn block_with_planning_attempt(
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
    kind: PhysicalRecoveryBlockKind,
    counters: PhysicalRecoveryDiscoveryCounters,
    planning_counters: RecoveryPlanningCounters,
    artifact: &str,
    limit: Option<PhysicalRecoveryLimitFailure>,
    planning_denial: Option<PhysicalRecoveryPlanningDenial>,
) -> PhysicalRecoveryOutcome {
    block_unsupported_scope(
        authority,
        coordination,
        kind,
        PhysicalRecoveryBlockEvidence {
            counters,
            planning_counters: Some(planning_counters),
            limit,
            artifact: Some(artifact.to_owned()),
            planning_denial,
            ..Default::default()
        },
    )
}

pub(super) fn sample_limit(
    failure: worth_store::physical_runtime::StoreRecoveryBindingSampleFailure,
    operation_bindings: u64,
    redo_bytes: u64,
) -> Option<PhysicalRecoveryLimitFailure> {
    match failure.denial() {
        StoreRecoveryBindingSampleDenial::OperationBindingLimit => {
            Some(PhysicalRecoveryLimitFailure {
                dimension: PhysicalRecoveryLimitDimension::OperationBindings,
                observed: failure.operation_bindings_observed(),
                admitted: operation_bindings,
            })
        }
        StoreRecoveryBindingSampleDenial::RedoByteLimit => Some(PhysicalRecoveryLimitFailure {
            dimension: PhysicalRecoveryLimitDimension::RedoBytes,
            observed: failure.redo_bytes_observed(),
            admitted: redo_bytes,
        }),
        _ => None,
    }
}
