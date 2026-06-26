use super::super::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class, QuerySelectionSurfaceOwner as Owner,
};
use super::super::source_path_catalog::topo_path;
use super::core::row;

pub(in crate::query_obligation_selection::boundary_inventory) fn topo_primitive(
    surface: &'static str,
    file: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    action: Action,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        topo_path(file),
        Some("topology::facade::primitive_construction"),
        surface,
        class,
        posture,
        proof,
        "worth-topo primitive construction touched-basis execution",
        action,
        Owner::WorthTopo,
        None,
        None,
        None,
    )
}
