use worth_store_recovery_physics::{
    admit_recovery_plan_cost, RecoveryPlanCost, RecoveryPlanCostDenial, RecoveryPlanLimits,
};

use crate::entry::{
    PhysicalRecoveryLimitDimension, PhysicalRecoveryLimitFailure, PhysicalRecoveryOutcome,
};
use crate::handoff::RecoveryOperationFateSet;
use crate::progression::{derive_execution_basis, ExecutionBasisDenial, PlannedPhysicalRecovery};

use super::context::PlanningContext;
use super::denial::plan_cost_limit;
use super::resolved_basis::ResolvedPlanningBasis;

pub(super) fn complete(
    context: PlanningContext,
    basis: ResolvedPlanningBasis,
) -> Result<PlannedPhysicalRecovery, PhysicalRecoveryOutcome> {
    let planning_counters = basis.planning_counters();
    let (staging, publication, quiescence) = match derive_execution_basis(
        context.authority.media.store_identity(),
        &context.selection,
        &basis.sample,
        &basis.fates,
        &basis.redo,
        &basis.observed_pages.selected_source,
        context.limits.staging_bytes,
        context.limits.dirty_frames,
    ) {
        Ok(execution) => execution,
        Err(ExecutionBasisDenial::StagingBytes { observed }) => {
            let admitted = context.limits.staging_bytes;
            return Err(context.cost_denial_block(
                planning_counters,
                RecoveryPlanCostDenial::StagingBytes,
                PhysicalRecoveryLimitFailure {
                    dimension: PhysicalRecoveryLimitDimension::StagingBytes,
                    observed,
                    admitted,
                },
            ));
        }
        Err(ExecutionBasisDenial::DirtyFrames { observed }) => {
            let admitted = context.limits.dirty_frames;
            return Err(context.cost_denial_block(
                planning_counters,
                RecoveryPlanCostDenial::DirtyFrames,
                PhysicalRecoveryLimitFailure {
                    dimension: PhysicalRecoveryLimitDimension::DirtyFrames,
                    observed,
                    admitted,
                },
            ));
        }
        Err(ExecutionBasisDenial::Invalid) => {
            return Err(context.redo_block(planning_counters, None));
        }
    };
    let plan_limits = RecoveryPlanLimits::new(
        context.limits.redo_targets,
        context.limits.redo_bytes,
        context.limits.distinct_pages_and_extents,
        context.limits.operation_bindings,
        context.limits.observation_bytes,
        context.limits.staging_bytes,
        context.limits.dirty_frames,
    )
    .expect("admitted runtime limits are nonzero");
    let plan_cost = RecoveryPlanCost::new(
        basis.targets.len() as u64,
        basis.redo_bytes,
        basis.distinct_targets,
        basis.sample.operations().len() as u64,
        basis.observed_pages.artifact_reads,
        context
            .counters
            .bytes_observed
            .saturating_add(basis.observed_pages.bytes_read),
        staging.allocated_bytes(),
        staging.dirty_frames(),
    );
    let plan_cost = match admit_recovery_plan_cost(plan_limits, plan_cost) {
        Ok(cost) => cost,
        Err(denial) => {
            let limit = plan_cost_limit(denial, plan_limits, plan_cost);
            return Err(context.cost_denial_block(planning_counters, denial, limit));
        }
    };
    if publication.expected_effects() > context.limits.publication_effects {
        let admitted = context.limits.publication_effects;
        return Err(context.redo_block(
            planning_counters,
            Some(PhysicalRecoveryLimitFailure {
                dimension: PhysicalRecoveryLimitDimension::PublicationEffects,
                observed: publication.expected_effects(),
                admitted,
            }),
        ));
    }
    assert_eq!(
        context.effects_before,
        context.authority.media.recovery_effect_count()
    );
    Ok(PlannedPhysicalRecovery::new(
        context.authority,
        context.coordination,
        context.selection,
        context.counters,
        basis.sample,
        RecoveryOperationFateSet::new(basis.fates),
        basis.redo,
        plan_cost,
        planning_counters,
        staging,
        publication,
        quiescence,
    ))
}
