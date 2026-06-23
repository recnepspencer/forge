use forge_query::facade::runtime::{forge_query_domain, ForgeQueryIntentDeclaration, ForgeQueryIntentInput};

fn support_common_lane() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        ForgeQueryIntentInput::object([("edge", ForgeQueryIntentInput::string("e-1"))]),
    );

    let _support = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_traceability("graph.face_inner_loop_insertion")
        .because("topology substrate is available and traceable")
        .materialize();
}

fn main() {}
