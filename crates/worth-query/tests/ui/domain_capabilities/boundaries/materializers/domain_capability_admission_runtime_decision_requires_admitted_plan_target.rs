use worth_query::facade::{
    admit_eligible_domain_capability_contribution, evaluate_requested_domain_capability_contribution,
    materialize_runtime_admission_decision, prepare_admitted_domain_capability_contribution_for_materialization,
    WorthQueryAdmissionContributionAuthoring, WorthQueryIntentDeclaration, WorthQueryIntentInput,
};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "test.intent",
        "test.strategy",
        "1",
        "test.contract",
        WorthQueryIntentInput::object([("entity", WorthQueryIntentInput::string("edge:42"))]),
    );
    let requested = WorthQueryAdmissionContributionAuthoring::advisory(
        "spatial.arbitration.requires_clarification",
        "multiple candidates remain admissible",
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

    let _ = materialize_runtime_admission_decision(ready);
}
