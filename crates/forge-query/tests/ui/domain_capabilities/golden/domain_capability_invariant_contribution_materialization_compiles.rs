use forge_query::facade::runtime::{
    forge_query_domain, ForgeQueryIntentDeclaration, InvariantCatalog, InvariantRegistration,
    InvariantRule,
};
use serde_json::json;

fn invariant_common_lane() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        json!({"edge":"e-1"}),
    );
    let invariant_catalog = InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(7),
        )],
    };

    let _artifact = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .register_invariant_catalog("spatial.non_manifold_edge_split", invariant_catalog)
        .because("geometry kernel must reject non-manifold edge splits")
        .materialize();
}

fn main() {}
