use worth_store_recovery_physics::{
    admit_recovery_plan_cost, RecoveryPlanCost, RecoveryPlanLimits,
};

use crate::entry::PhysicalRecoveryOutcome;
use crate::progression::RecoveryStagingLayoutPlan;

use super::super::context::PlanningContext;
use super::super::denial::plan_cost_limit;
use super::super::resolved_basis::ResolvedPlanningBasis;

pub(super) fn admit(
    context: PlanningContext,
    basis: &ResolvedPlanningBasis,
    staging: &RecoveryStagingLayoutPlan,
) -> Result<
    (
        PlanningContext,
        RecoveryPlanCost,
        worth_store_recovery_physics::RecoveryPlanningCounters,
    ),
    PhysicalRecoveryOutcome,
> {
    let plan_limits = RecoveryPlanLimits::new(
        context.limits.redo_targets,
        context.limits.redo_bytes,
        context.limits.distinct_pages_and_extents,
        context.limits.operation_bindings,
        context.limits.observation_bytes,
        context.limits.staging_bytes,
        context.limits.recovery_memory_bytes,
        context.limits.dirty_frames,
    )
    .expect("admitted runtime limits are nonzero");
    let peak_recovery_bytes = basis
        .observed_pages
        .bytes_read
        .checked_add(staging.allocated_bytes())
        .and_then(|bytes| bytes.checked_add(staging.write_bytes()))
        .expect("admitted recovery memory accounting cannot overflow");
    let planning_counters = basis
        .planning_counters()
        .with_peak_recovery_bytes(peak_recovery_bytes);
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
        peak_recovery_bytes,
        staging.dirty_frames(),
    );
    let plan_cost = match admit_recovery_plan_cost(plan_limits, plan_cost) {
        Ok(cost) => cost,
        Err(denial) => {
            let limit = plan_cost_limit(denial, plan_limits, plan_cost);
            return Err(context.cost_denial_block(planning_counters, denial, limit));
        }
    };
    Ok((context, plan_cost, planning_counters))
}
