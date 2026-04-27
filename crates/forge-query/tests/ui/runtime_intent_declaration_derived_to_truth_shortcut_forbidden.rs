use forge_query::facade::{ForgeQueryIntentDeclaration, ForgeQueryIntentSourceLane};
use serde_json::json;

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "derive-to-truth",
        "commit-derived-output",
        "v1",
        "contract",
        json!({}),
    );

    let _ = declaration.with_source_lane(ForgeQueryIntentSourceLane::DerivedRuntime);
}
