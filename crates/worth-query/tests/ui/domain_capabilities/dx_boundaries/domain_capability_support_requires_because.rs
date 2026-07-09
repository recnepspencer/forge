use worth_query::facade::runtime::{worth_query_domain, WorthQueryIntentDeclaration, WorthQueryIntentInput};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );

    let _ = worth_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_traceability("graph.face_inner_loop_insertion")
        .materialize();
}
