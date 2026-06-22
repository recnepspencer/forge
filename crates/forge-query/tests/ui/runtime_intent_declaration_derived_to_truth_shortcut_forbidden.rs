use forge_query::facade::{
    ForgeQueryIntentDeclaration, ForgeQueryIntentInput, ForgeQueryIntentSourceLane,
};

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "derive-to-truth",
        "commit-derived-output",
        "v1",
        "contract",
        ForgeQueryIntentInput::object([]),
    );

    let _ = declaration.with_source_lane(ForgeQueryIntentSourceLane::DerivedRuntime);
}
