use forge_query::facade::{
    admit_eligible_domain_capability_contribution, admit_runtime_intent_request,
    evaluate_requested_domain_capability_contribution, materialize_query_causal_inspection_artifact,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryExplanationContributionAuthoring, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentDeclaration, ForgeQueryIntentInput, ForgeQueryRawIntentAdmissionRequest,
};

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "test.intent",
        "test.strategy",
        "1",
        "test.contract",
        ForgeQueryIntentInput::object([("entity", ForgeQueryIntentInput::string("edge:42"))]),
    );
    let request = ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
        declaration.clone(),
    )
    .expect("request should build");
    let ForgeQueryIntentAdmissionDecision::Admitted(plan) = admit_runtime_intent_request(request)
    else {
        unreachable!();
    };
    let requested = ForgeQueryExplanationContributionAuthoring::requires_context(
        "explanation.support.only",
        "causal inspection remains admitted-plan scoped only",
    )
    .for_admitted_intent_plan(&plan);
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let admitted = match admit_eligible_domain_capability_contribution(eligible) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let target = admitted.payload().target().clone();
    let ready = match prepare_admitted_domain_capability_contribution_for_materialization(
        admitted,
        target,
    ) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };

    let _ = materialize_query_causal_inspection_artifact(ready);
}
