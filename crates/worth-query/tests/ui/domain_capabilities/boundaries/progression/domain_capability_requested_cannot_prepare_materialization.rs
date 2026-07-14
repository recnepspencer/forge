use worth_query::facade::runtime::{prepare_admitted_domain_capability_contribution_for_materialization, WorthQueryAdmissionContributionAuthoring, WorthQueryDeclarationBoundContributionTarget, WorthQueryIntentDeclaration, WorthQueryIntentInput};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "offset",
        "worth.spatial.offset",
        "1",
        "worth.spatial.intent",
        WorthQueryIntentInput::object([("entity", WorthQueryIntentInput::string("edge:42"))]),
    );
    let requested = WorthQueryAdmissionContributionAuthoring::violation(
        "spatial.binding.changed",
        "binding no longer matches",
    )
    .for_intent_declaration(&declaration);
    let target = WorthQueryDeclarationBoundContributionTarget::for_intent_declaration(&declaration);
    let _ = prepare_admitted_domain_capability_contribution_for_materialization(requested, target);
}
