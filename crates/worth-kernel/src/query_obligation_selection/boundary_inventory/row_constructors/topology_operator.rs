use super::super::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class, QuerySelectionSurfaceOwner as Owner,
};
use super::super::source_path_catalog::{topo_operator_path, topo_operator_surface_path};
use super::core::row;

pub(in crate::query_obligation_selection::boundary_inventory) fn topo_operator(
    surface: &'static str,
    file: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    action: Action,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        topo_operator_path(file),
        Some("topology::facade::topology_operators"),
        surface,
        class,
        posture,
        proof,
        "worth-topo topology operator graph-obligation adoption",
        action,
        Owner::WorthTopo,
        None,
        None,
        None,
    )
}

pub(in crate::query_obligation_selection::boundary_inventory) fn topo_operator_residue(
    surface: &'static str,
    cap: &'static str,
    blocker: &'static str,
    trigger: &'static str,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        topo_operator_path("residue/residue_manifest.rs"),
        Some("topology::facade::topology_operators"),
        surface,
        Class::CappedResidue,
        Posture::ResidueManifest,
        Proof::ResidueOnly,
        "worth-topo topology operator graph-obligation adoption",
        Action::CappedResidue,
        Owner::WorthTopo,
        Some(cap),
        Some(blocker),
        Some(trigger),
    )
}

pub(in crate::query_obligation_selection::boundary_inventory) fn topo_operator_application(
    surface: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    action: Action,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        "crates/worth-topo/src/topology_operators/application/declared_mutation_artifact/mutation_evidence.rs",
        None,
        surface,
        class,
        posture,
        proof,
        "worth-topo topology operator application evidence",
        action,
        Owner::WorthTopo,
        None,
        None,
        None,
    )
}

pub(in crate::query_obligation_selection::boundary_inventory) fn topo_operator_surface(
    surface: &'static str,
    file: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    action: Action,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        topo_operator_surface_path(file),
        None,
        surface,
        class,
        posture,
        proof,
        "worth-topo topology operator runtime projection surface",
        action,
        Owner::WorthTopo,
        None,
        None,
        None,
    )
}
