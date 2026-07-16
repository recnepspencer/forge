#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::{WorthQueryIntentDeclaration, WorthQueryIntentInput};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );

    let installation = installed_domain::install("support-requires-because");
    let _ = installation
        .contributions()
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .supports_traceability("graph.face_inner_loop_insertion")
        .materialize();
}
