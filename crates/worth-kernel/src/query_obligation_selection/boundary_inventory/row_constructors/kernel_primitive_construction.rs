use super::super::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class, QuerySelectionSurfaceOwner as Owner,
};
use super::super::source_path_catalog::kernel_path;
use super::core::row;

pub(in crate::query_obligation_selection::boundary_inventory) fn primitive(
    surface: &'static str,
    file: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    action: Action,
) -> QuerySelectionBoundaryInventoryRow {
    primitive_with_caller(
        surface,
        file,
        class,
        posture,
        proof,
        "worth-kernel primitive construction graph-obligation adoption",
        action,
    )
}

pub(in crate::query_obligation_selection::boundary_inventory) fn primitive_with_caller(
    surface: &'static str,
    file: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    caller: &'static str,
    action: Action,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        kernel_path(file),
        None,
        surface,
        class,
        posture,
        proof,
        caller,
        action,
        Owner::WorthKernel,
        None,
        None,
        None,
    )
}

pub(in crate::query_obligation_selection::boundary_inventory) fn primitive_residue(
    surface: &'static str,
    cap: &'static str,
    blocker: &'static str,
    trigger: &'static str,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        kernel_path("residue.rs"),
        None,
        surface,
        Class::CappedResidue,
        Posture::ResidueManifest,
        Proof::ResidueOnly,
        "worth-kernel primitive construction graph-obligation adoption",
        Action::CappedResidue,
        Owner::WorthKernel,
        Some(cap),
        Some(blocker),
        Some(trigger),
    )
}
