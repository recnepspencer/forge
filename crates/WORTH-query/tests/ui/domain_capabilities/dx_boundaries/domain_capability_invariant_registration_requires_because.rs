use worth_query::facade::runtime::{
    worth_query_domain, WorthQueryIntentDeclaration, WorthQueryIntentInput, InvariantCatalog,
};

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
        .register_invariant_catalog("spatial.non_manifold_edge_split", InvariantCatalog::default())
        .materialize();
}
