use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryIntentDeclaration,
};
use serde_json::json;

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "intent",
        "strategy.intent.reconcile",
        "1.0",
        "input-contract",
        json!({"taskId": "task-1"}),
    );
    let _ = declaration.with_target_lane(ForgeQueryAuthorityLane::PreviewTruth);
}
