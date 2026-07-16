#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::{BridgePreviewSessionIdentity, WorthQueryIntentDeclaration, WorthQueryIntentInput, InvariantCatalog};

fn main() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );
    let installation = installed_domain::install("declaration-transcript-golden");
    let domain = installation.contributions();

    let _support = domain
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .supports_capability("graph.face_inner_loop_insertion")
        .because("topology substrate is available")
        .materialize();

    let _advisory = domain
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .advises("arbitration.requires_clarification")
        .because("multiple spatial candidates remain admissible")
        .materialize();

    let _preview = domain
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .inspects_query_preview(
            "topology.preview_conflict",
            BridgePreviewSessionIdentity::from_stable_name("preview-session:42"),
        )
        .because("preview remains read-only while topology is inspected")
        .materialize();

    let invariant_catalog = InvariantCatalog::default();

    let _registration = domain
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .register_invariant_catalog("spatial.non_manifold_edge_split", invariant_catalog)
        .because("geometry kernel must reject non-manifold edge splits")
        .materialize();
}
