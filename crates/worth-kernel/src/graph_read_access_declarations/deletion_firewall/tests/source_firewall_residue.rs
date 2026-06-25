use std::collections::BTreeSet;

use super::phase_chain_fixture::production_phase_six_seed;
use super::temp_workspace_fixture::{
    temp_workspace_with_forbidden_pattern, temp_workspace_with_old_graph_read_adoption_residue,
    temp_workspace_with_topology_declaration_helper_residue,
};
use crate::graph_read_access_declarations::{
    current_worth_graph_read_declaration_deletion_firewall_closeout,
    WorthGraphReadDeclarationSourceFirewallRegionReport,
};

use super::super::closeout::closeout_for_workspace_root;
use super::super::errors::WorthGraphReadDeclarationDeletionFirewallErrorKind;
use super::super::source_firewall::{forbidden_pattern_audit_rows, SourceFirewallRegion};

#[test]
fn old_local_declaration_shims_are_deleted_or_capped() {
    let phase_six_seed = production_phase_six_seed();
    let closeout = current_worth_graph_read_declaration_deletion_firewall_closeout(&phase_six_seed)
        .expect("production declaration residue should be deleted or capped");

    assert_eq!(closeout.capped_residue_report().residue_count(), 0);
    assert_eq!(
        source_firewall_regions(closeout.source_firewall_report().region_reports()),
        BTreeSet::from([
            SourceFirewallRegion::DeclarationAuthority,
            SourceFirewallRegion::WorthKernelAdoptionAuthority,
            SourceFirewallRegion::TopologySpatialReadHelpers,
        ])
    );
    assert!(closeout
        .source_firewall_report()
        .region_reports()
        .iter()
        .all(|region| region.audited_pattern_count() > 0));
    assert_eq!(closeout.source_firewall_report().violation_count(), 0);
    assert!(closeout.source_firewall_report().scanned_source_count() > 0);
}

#[test]
fn phase_six_closeout_rejects_uncapped_residue_growth() {
    let phase_six_seed = production_phase_six_seed();
    let workspace_root = temp_workspace_with_old_graph_read_adoption_residue();

    let error = closeout_for_workspace_root(&phase_six_seed, &workspace_root)
        .expect_err("remaining old graph-read adoption path must exceed zero residue cap");

    assert_eq!(
        error.kind(),
        WorthGraphReadDeclarationDeletionFirewallErrorKind::CappedResidueCapExceeded
    );
}

#[test]
fn phase_six_source_firewall_rejects_topology_helper_residue() {
    let phase_six_seed = production_phase_six_seed();
    let workspace_root = temp_workspace_with_topology_declaration_helper_residue();

    let error = closeout_for_workspace_root(&phase_six_seed, &workspace_root)
        .expect_err("topology fallback traversal residue must fail the source firewall");

    assert_eq!(
        error.kind(),
        WorthGraphReadDeclarationDeletionFirewallErrorKind::SourceFirewallViolation
    );
}

#[test]
fn source_firewall_rejects_every_registered_forbidden_pattern_region() {
    let phase_six_seed = production_phase_six_seed();

    for audit_row in forbidden_pattern_audit_rows() {
        let workspace_root =
            temp_workspace_with_forbidden_pattern(audit_row.region(), audit_row.text());
        let error = closeout_for_workspace_root(&phase_six_seed, &workspace_root).expect_err(
            "each registered forbidden behavior pattern should fail in its audited region",
        );

        assert_eq!(
            error.kind(),
            WorthGraphReadDeclarationDeletionFirewallErrorKind::SourceFirewallViolation,
            "pattern `{}` in region `{}` should be caught by the source firewall",
            audit_row.text(),
            audit_row.region().digest_part()
        );
    }
}

fn source_firewall_regions(
    rows: &[WorthGraphReadDeclarationSourceFirewallRegionReport],
) -> BTreeSet<SourceFirewallRegion> {
    rows.iter().map(|row| row.region()).collect()
}
