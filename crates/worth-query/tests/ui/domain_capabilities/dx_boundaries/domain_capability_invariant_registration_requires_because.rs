#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::{WorthQueryIntentDeclaration, WorthQueryIntentInput, InvariantCatalog};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );

    let installation = installed_domain::install("invariant-requires-because");
    let _ = installation
        .contributions()
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .register_invariant_catalog("spatial.non_manifold_edge_split", InvariantCatalog::default())
        .materialize();
}
