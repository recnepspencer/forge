#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::{BridgePreviewSessionIdentity, WorthQueryIntentDeclaration, WorthQueryIntentInput};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );

    let installation = installed_domain::install("workflow-draft-no-declaration");
    let _ = installation
        .contributions()
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .plans_preview_mutation(
            "topology.preview_mutation",
            BridgePreviewSessionIdentity::from_stable_name("preview-session:77"),
        )
        .workflow_declaration();
}
