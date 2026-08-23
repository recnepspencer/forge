#[path = "completion/execution_basis.rs"]
mod execution_basis;
#[path = "completion/plan_cost.rs"]
mod plan_cost;
#[path = "completion/planned_recovery.rs"]
mod planned_recovery;
#[path = "completion/publication_effects.rs"]
mod publication_effects;

use crate::progression::PlannedPhysicalRecovery;

use super::context::PlanningContext;
use super::resolved_basis::ResolvedPlanningBasis;

pub(super) fn complete(
    context: PlanningContext,
    basis: ResolvedPlanningBasis,
) -> Result<PlannedPhysicalRecovery, crate::entry::PhysicalRecoveryOutcome> {
    let (context, execution) = execution_basis::derive(context, &basis)?;
    let (context, plan_cost, planning_counters) =
        plan_cost::admit(context, &basis, &execution.staging)?;
    let context = publication_effects::admit(context, planning_counters, &execution.publication)?;
    Ok(planned_recovery::construct(
        context,
        basis,
        execution,
        plan_cost,
        planning_counters,
    ))
}
