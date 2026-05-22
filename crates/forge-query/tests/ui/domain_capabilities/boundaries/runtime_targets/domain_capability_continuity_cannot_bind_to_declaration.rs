use forge_query::facade::runtime::{
    ForgeQueryContinuityContributionAuthoring, ForgeQueryIntentDeclaration,
};
use serde_json::json;

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "continuity",
        "worth.spatial.continuity",
        "1",
        "worth.spatial.intent",
        json!({ "entity": "edge:42" }),
    );
    let _ = ForgeQueryContinuityContributionAuthoring::preserved(
        "continuity.preserved",
        "identity was preserved",
    )
    .for_intent_declaration(&declaration);
}
