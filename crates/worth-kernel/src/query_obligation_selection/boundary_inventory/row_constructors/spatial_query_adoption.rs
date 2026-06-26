use super::super::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class, QuerySelectionSurfaceOwner as Owner,
};
use super::super::source_path_catalog::spatial_path;
use super::super::spatial_support_projection_residue_policy::{
    spatial_support_projection_residue_blocker, spatial_support_projection_residue_cap,
    spatial_support_projection_residue_trigger,
};
use super::core::row;

pub(in crate::query_obligation_selection::boundary_inventory) fn spatial(
    surface: &'static str,
    file: &'static str,
    class: Class,
    posture: Posture,
    proof: Proof,
    action: Action,
    facade: Option<&'static str>,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        spatial_path(file),
        facade,
        surface,
        class,
        posture,
        proof,
        "worth-spatial query adoption Consumer Kit",
        action,
        Owner::WorthSpatial,
        spatial_support_projection_residue_cap(surface, class),
        spatial_support_projection_residue_blocker(surface, class),
        spatial_support_projection_residue_trigger(surface, class),
    )
}

pub(in crate::query_obligation_selection::boundary_inventory) fn spatial_residue(
    surface: &'static str,
    file: &'static str,
    cap: &'static str,
    blocker: &'static str,
    trigger: &'static str,
) -> QuerySelectionBoundaryInventoryRow {
    row(
        spatial_path(file),
        Some("worth_spatial::facade::query_adoption"),
        surface,
        Class::CappedResidue,
        Posture::ResidueManifest,
        Proof::ResidueOnly,
        "worth-spatial query adoption Consumer Kit",
        Action::CappedResidue,
        Owner::WorthSpatial,
        Some(cap),
        Some(blocker),
        Some(trigger),
    )
}
