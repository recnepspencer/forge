use worth_query::facade::{
    WorthQueryAuthorityLane, WorthQueryIntentDeclaration, WorthQueryIntentInput,
};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "intent",
        "strategy.intent.reconcile",
        "1.0",
        "input-contract",
        WorthQueryIntentInput::object([("taskId", WorthQueryIntentInput::string("task-1"))]),
    );
    let _ = declaration.with_target_lane(WorthQueryAuthorityLane::PreviewTruth);
}
