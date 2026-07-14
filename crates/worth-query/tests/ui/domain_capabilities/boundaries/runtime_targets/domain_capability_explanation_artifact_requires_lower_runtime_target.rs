use worth_query::facade::runtime::{admit_eligible_domain_capability_contribution, evaluate_requested_domain_capability_contribution, materialize_query_causal_inspection_artifact, prepare_admitted_domain_capability_contribution_for_materialization, WorthQueryAdmittedIntentPlan, WorthQueryExplanationContributionAuthoring};

fn wrong_target(plan: &WorthQueryAdmittedIntentPlan) {
    let requested = WorthQueryExplanationContributionAuthoring::requires_context(
        "explanation.support.only",
        "causal inspection remains admitted-plan scoped only",
    )
    .for_admitted_intent_plan(plan);
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        worth_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let admitted = match admit_eligible_domain_capability_contribution(eligible) {
        worth_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let target = admitted.payload().target().clone();
    let ready = match prepare_admitted_domain_capability_contribution_for_materialization(
        admitted,
        target,
    ) {
        worth_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };

    let _ = materialize_query_causal_inspection_artifact(ready);
}

fn main() {}
