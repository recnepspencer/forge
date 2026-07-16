#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::{WorthQueryIntentDeclaration, WorthQueryIntentInput};

fn support_common_lane() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );

    let installation = installed_domain::install("support-golden");
    let _support = installation
        .contributions()
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .supports_traceability("graph.face_inner_loop_insertion")
        .because("topology substrate is available and traceable")
        .materialize();
}

fn main() {}
