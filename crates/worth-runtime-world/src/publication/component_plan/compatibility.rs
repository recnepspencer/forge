use crate::branch::{ProductBranchComponentPosture, ProductBranchObservation};

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
    relational_plan_is_compatible(
        plan.relational(),
        intent.component_postures().relational(),
        intent.component_intent().changes_relational(),
    ) && signal_plan_is_compatible(
        plan.signal(),
        intent.component_postures().signal(),
        intent.component_intent().changes_signal(),
    )
}

fn relational_plan_is_compatible(
    plan: &RelationalComponentPlan,
    posture: ProductBranchComponentPosture,
    changes: bool,
) -> bool {
    match plan.posture() {
        RelationalComponentPlanPosture::RetainExact => {
            posture == ProductBranchComponentPosture::ReuseExact
                && !changes
                && plan.prepared_candidate().is_none()
                && plan.fork_source().is_none()
        }
        // The frozen upstream candidate does not expose the exact basis
        // affinity needed here, so no Relational mutation route is admissible
        // until the owner binding supplies that proof.
        RelationalComponentPlanPosture::PublishPrepared => false,
        RelationalComponentPlanPosture::ForkThenPublish => false,
    }
}

fn signal_plan_is_compatible(
    plan: &SignalComponentPlan,
    posture: ProductBranchComponentPosture,
    changes: bool,
) -> bool {
    match plan.posture() {
        SignalComponentPlanPosture::RetainExact => {
            posture == ProductBranchComponentPosture::ReuseExact
                && !changes
                && plan.requested_branch_name().is_none()
        }
        SignalComponentPlanPosture::AdvanceExact => {
            posture == ProductBranchComponentPosture::ReuseExact
                && changes
                && plan.requested_branch_name().is_none()
        }
        SignalComponentPlanPosture::ForkThenAdvance => {
            matches!(
                posture,
                ProductBranchComponentPosture::ForkExact
                    | ProductBranchComponentPosture::ForkAndAdvance
            ) && plan.requested_branch_name().is_some()
        }
    }
}
