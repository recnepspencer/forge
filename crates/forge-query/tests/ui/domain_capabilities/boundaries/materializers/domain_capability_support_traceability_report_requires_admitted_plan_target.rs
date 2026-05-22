use forge_query::facade::{
    admit_eligible_domain_capability_contribution, evaluate_requested_domain_capability_contribution,
    materialize_intent_admission_support_traceability_report,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryIntentDeclaration, ForgeQuerySupportContributionAuthoring,
};

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "test.intent",
        "test.strategy",
        "1",
        "test.contract",
        serde_json::json!({ "entity": "edge:42" }),
    );
    let requested = ForgeQuerySupportContributionAuthoring::declaration_support(
        "spatial.support",
        "support remains declaration-scoped",
    )
    .for_intent_declaration(&declaration);
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

    let _ = materialize_intent_admission_support_traceability_report(ready);
}
