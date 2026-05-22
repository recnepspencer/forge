use forge_query::facade::runtime::{
    admit_eligible_domain_capability_contribution, ForgeQueryAdmissionContributionAuthoring,
    ForgeQueryIntentDeclaration,
};
use serde_json::json;

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "rotate",
        "worth.spatial.rotate",
        "1",
        "worth.spatial.intent",
        json!({ "entity": "edge:42" }),
    );
    let requested = ForgeQueryAdmissionContributionAuthoring::advisory(
        "arbitration.requires_clarification",
        "multiple candidates remain",
    )
    .for_intent_declaration(&declaration);
    let _ = admit_eligible_domain_capability_contribution(requested);
}
