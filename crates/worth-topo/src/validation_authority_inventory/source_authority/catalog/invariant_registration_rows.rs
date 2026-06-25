use super::row;
use crate::validation_authority_inventory::authority_kind::WorthValidationAuthorityKind;
use crate::validation_authority_inventory::disposition::WorthValidationAuthorityDisposition;
use crate::validation_authority_inventory::inventory_row::{
    WorthValidationAuthorityInventoryRow, WorthValidationAuthorityInventoryRowInput,
};
use crate::validation_authority_inventory::source_authority::WorthValidationAuthoritySource;

pub(super) fn push_invariant_registration_rows(
    rows: &mut Vec<WorthValidationAuthorityInventoryRow>,
) {
    for name in [
        "ownership.graph_composition",
        "ownership.commit_backstop",
        "loop_wiring.graph_composition",
        "loop_wiring.commit_backstop",
        "radial_rings.graph_composition",
        "radial_rings.commit_backstop",
        "wire_connectivity.graph_composition",
        "wire_connectivity.commit_backstop",
        "vertex_disks.graph_composition",
        "vertex_disks.commit_backstop",
        "shell_closure.graph_composition",
        "shell_closure.commit_backstop",
        "naming.graph_composition",
        "naming.commit_backstop",
    ] {
        rows.push(row(WorthValidationAuthorityInventoryRowInput {
            source: WorthValidationAuthoritySource::MilestoneOneInvariantRegistration(name),
            source_path: "crates/worth-topo/src/validation/reference_integrity/mod.rs",
            source_symbol: "milestone_one_invariant_registrations",
            authority_kind: WorthValidationAuthorityKind::RuntimeInvariantRegistrationPack,
            owner: "worth-topo.validation.reference_integrity",
            disposition: WorthValidationAuthorityDisposition::Migrate,
            removal_trigger: "Phase 5 converts the invariant into a Query-registered invariant family.",
            query_access_dependency: Some("Query invariant registration facade"),
            certification_only_comparison_allowed: true,
            note: "Old static invariant pack entry; may survive only as comparison residue until migrated.",
        }));
    }
}
