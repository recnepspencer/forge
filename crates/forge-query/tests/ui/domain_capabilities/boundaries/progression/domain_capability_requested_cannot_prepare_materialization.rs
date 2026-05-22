use forge_query::facade::runtime::{
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryAdmissionContributionAuthoring, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryIntentDeclaration,
};
use serde_json::json;

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "offset",
        "worth.spatial.offset",
        "1",
        "worth.spatial.intent",
        json!({ "entity": "edge:42" }),
    );
    let requested = ForgeQueryAdmissionContributionAuthoring::violation(
        "spatial.binding.changed",
        "binding no longer matches",
    )
    .for_intent_declaration(&declaration);
    let target = ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(&declaration);
    let _ = prepare_admitted_domain_capability_contribution_for_materialization(requested, target);
}
