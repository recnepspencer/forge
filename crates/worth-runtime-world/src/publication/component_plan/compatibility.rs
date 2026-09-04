use crate::branch::ProductBranchObservation;

use super::{
    LoweredOwnerComponentPlan, RelationalComponentPlan, RelationalComponentPlanPosture,
    SignalComponentPlan, SignalComponentPlanPosture,
};

pub(super) fn plan_is_compatible_with(
    plan: &LoweredOwnerComponentPlan,
    expected: &ProductBranchObservation,
) -> bool {
    let expected_head = plan.expected().expected();
    if expected_head != expected
        || crate::basis::compare_exact(expected_head.basis(), expected.basis()).is_err()
    {
        return false;
    }
    let basis = expected.basis();
    if plan.relational().expected() != basis.relational_basis()
        || plan.signal().expected().admission_identity()
            != basis.signal_basis().admission_identity()
    {
        return false;
    }
    let intent = plan.expected().intent();
    relational_plan_is_compatible(plan.relational(), intent.changes_relational())
        && signal_plan_is_compatible(plan.signal(), intent.changes_signal())
}

fn relational_plan_is_compatible(plan: &RelationalComponentPlan, changes: bool) -> bool {
    match plan.posture() {
        RelationalComponentPlanPosture::RetainExact => {
            !changes && plan.prepared_candidate().is_none()
        }
        RelationalComponentPlanPosture::PublishPrepared => {
            changes
                && plan.prepared_candidate().is_some_and(|candidate| {
                    candidate.branch() == plan.expected().identity().branch_id()
                })
        }
    }
}

fn signal_plan_is_compatible(plan: &SignalComponentPlan, changes: bool) -> bool {
    match plan.posture() {
        SignalComponentPlanPosture::RetainExact => !changes,
        SignalComponentPlanPosture::AdvanceExact => changes,
    }
}
