use super::super::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class, QuerySelectionSurfaceOwner as Owner,
};
use super::super::source_path_catalog::forge_query_path;
use super::core::row;

pub(in crate::query_obligation_selection::boundary_inventory) fn forge_query(
    surface: &'static str,
    file: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    action: Action,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        forge_query_path(file),
        Some("forge_query::consumer_kit::graph_obligation_adoption"),
        surface,
        class,
        posture,
        proof,
        "forge-query graph-obligation Consumer Kit",
        action,
        Owner::ForgeQuery,
        None,
        None,
        None,
    )
}
