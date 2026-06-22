use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryIntentDeclaration, ForgeQueryIntentInput,
};

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "intent",
        "strategy.intent.reconcile",
        "1.0",
        "input-contract",
        ForgeQueryIntentInput::object([("taskId", ForgeQueryIntentInput::string("task-1"))]),
    );
    let _ = declaration.with_target_lane(ForgeQueryAuthorityLane::PreviewTruth);
}
