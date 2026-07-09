use worth_query::facade::{
    WorthQueryIntentDeclaration, WorthQueryIntentInput, WorthQueryIntentSourceLane,
};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "derive-to-truth",
        "commit-derived-output",
        "v1",
        "contract",
        WorthQueryIntentInput::object([]),
    );

    let _ = declaration.with_source_lane(WorthQueryIntentSourceLane::DerivedRuntime);
}
