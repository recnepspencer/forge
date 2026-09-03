use crate::branch::{ProductBranchComponentPosture, ProductBranchObservation};

use super::{
    LoweredOwnerComponentPlan, RelationalComponentPlan, RelationalComponentPlanPosture,
    RelationalForkPlanInput, SignalComponentPlan, SignalComponentPlanPosture,
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
                && plan.fork_input().is_none()
        }
        RelationalComponentPlanPosture::PublishPrepared => plan
            .prepared_candidate()
            .is_some_and(|candidate| candidate.branch() == plan.expected().identity().branch_id()),
        RelationalComponentPlanPosture::ForkExact => plan.fork_input().is_some_and(|input| {
            matches!(input, RelationalForkPlanInput::ForkExact { .. })
                && fork_source_matches(input, plan)
        }),
        RelationalComponentPlanPosture::ForkAndAdvance => plan.fork_input().is_some_and(|input| {
            matches!(input, RelationalForkPlanInput::ForkAndAdvance { .. })
                && fork_source_matches(input, plan)
        }),
    }
}

fn fork_source_matches(input: &RelationalForkPlanInput, plan: &RelationalComponentPlan) -> bool {
    let source = input.source().descriptor();
    let expected = plan.expected().descriptor();
    source.runtime_instance_id() == expected.runtime_instance_id()
        && source.source_branch() == expected.branch_id()
        && source.observation() == expected.reference()
        && source.truth_version() == expected.truth_version()
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
        SignalComponentPlanPosture::ForkExact => {
            posture == ProductBranchComponentPosture::ForkExact
                && changes
                && plan.requested_branch_name().is_some()
        }
        SignalComponentPlanPosture::ForkAndAdvance => {
            posture == ProductBranchComponentPosture::ForkAndAdvance
                && changes
                && plan.requested_branch_name().is_some()
        }
    }
}
