use worth_query::facade::{
    WorthQueryIntentDeclaration, WorthQueryIntentInput, WorthQueryIntentSourceLane,
};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "intent",
        "strategy.intent.reconcile",
        "1.0",
        "input-contract",
        WorthQueryIntentInput::object([("taskId", WorthQueryIntentInput::string("task-1"))]),
    );
    let _ = declaration.with_source_lane(WorthQueryIntentSourceLane::EffectTriggered);
}
