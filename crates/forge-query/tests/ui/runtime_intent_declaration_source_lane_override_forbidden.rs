use forge_query::facade::{
    ForgeQueryIntentDeclaration, ForgeQueryIntentSourceLane,
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
    let _ = declaration.with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered);
}
