use worth_query::facade::runtime::{worth_query_domain, WorthQueryIntentDeclaration, WorthQueryIntentInput};

fn support_common_lane() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );

    let _support = worth_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_traceability("graph.face_inner_loop_insertion")
        .because("topology substrate is available and traceable")
        .materialize();
}

fn main() {}
