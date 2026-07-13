use worth_query::facade::runtime::{admit_eligible_domain_capability_contribution, evaluate_requested_domain_capability_contribution, materialize_intent_admission_support_traceability_report, prepare_admitted_domain_capability_contribution_for_materialization, WorthQueryIntentDeclaration, WorthQueryIntentInput, WorthQuerySupportContributionAuthoring};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "test.intent",
        "test.strategy",
        "1",
        "test.contract",
        WorthQueryIntentInput::object([("entity", WorthQueryIntentInput::string("edge:42"))]),
    );
    let requested = WorthQuerySupportContributionAuthoring::declaration_support(
        "spatial.support",
        "support remains declaration-scoped",
    )
    .for_intent_declaration(&declaration);
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

    let _ = materialize_intent_admission_support_traceability_report(ready);
}
