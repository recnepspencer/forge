use forge_query::facade::runtime::{
    forge_query_domain, BridgePreviewSessionIdentity, ForgeQueryIntentDeclaration, InvariantCatalog,
};
use serde_json::json;

fn main() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        json!({"edge":"e-1"}),
    );

    let _support = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_capability("graph.face_inner_loop_insertion")
        .because("topology substrate is available")
        .materialize();

    let _advisory = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .advises("arbitration.requires_clarification")
        .because("multiple spatial candidates remain admissible")
        .materialize();

    let _preview = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .inspects_query_preview(
            "topology.preview_conflict",
            BridgePreviewSessionIdentity::new("preview-session:42"),
        )
        .because("preview remains read-only while topology is inspected")
        .materialize();

    let invariant_catalog = InvariantCatalog::default();

    let _registration = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .register_invariant_catalog("spatial.non_manifold_edge_split", invariant_catalog)
        .because("geometry kernel must reject non-manifold edge splits")
        .materialize();
}
