use std::collections::BTreeSet;

use crate::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionErrorKind;

use super::super::source_firewall::{
    forbidden_pattern_audit_rows, scan_source, scan_source_for_region,
};
use super::{production_phase_seven_closeout, production_phase_seven_seed, TempWorkspace};

#[test]
fn source_firewall_rejects_local_loop_cache_and_receipt_residue() {
    for needle in [
        "local_graph_read_loop",
        "local_adjacency_map",
        "local_graph_cache",
        "fabricated_graph_read_receipt",
        "fabricated_access_plan_receipt",
        "old_helper_to_query_receipt_adapter",
    ] {
        let err = scan_source(
            "hostile.rs",
            &format!("fn relapse() {{ let _ = \"{needle}\"; }}"),
        )
        .expect_err("hard-deletion firewall should reject local execution residue");
        assert_eq!(err.forbidden_pattern(), needle);
    }
}

#[test]
fn source_firewall_rejects_every_registered_forbidden_pattern_in_each_declared_region() {
    for row in forbidden_pattern_audit_rows() {
        let err = scan_source_for_region(
            "hostile.rs",
            row.region(),
            &format!("fn relapse() {{ let _ = \"{}\"; }}", row.needle()),
        )
        .expect_err("registered hard-deletion residue should be rejected in its declared region");

        assert_eq!(err.forbidden_pattern(), row.label());
    }
}

#[test]
fn source_firewall_scans_production_regions() {
    let closeout = production_phase_seven_closeout();

    assert_eq!(closeout.source_firewall_report().violation_count(), 0);
    assert_eq!(closeout.source_firewall_report().scanned_region_count(), 4);
    assert!(closeout.source_firewall_report().scanned_source_count() > 0);
    assert_eq!(closeout.source_firewall_report().region_rows().len(), 4);
    let regions = closeout
        .source_firewall_report()
        .region_rows()
        .iter()
        .map(|row| row.region())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        regions,
        BTreeSet::from([
            "kernel_graph_read_helpers",
            "plan_adoption_authority",
            "spatial_read_consumers",
            "topology_read_consumers",
        ])
    );
    assert!(closeout
        .source_firewall_report()
        .region_rows()
        .iter()
        .all(|row| !row.root_identity().is_empty()));
    assert_eq!(
        closeout.source_firewall_report().forbidden_pattern_count(),
        forbidden_pattern_audit_rows()
            .iter()
            .map(|row| row.label())
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    );
}

#[test]
fn source_firewall_violation_under_each_workspace_region_fails_closeout() {
    for (region_path, forbidden_source) in [
        (
            "crates/worth-kernel/src/graph_read_access_plan_adoption/relapse.rs",
            "fn relapse() { local_graph_read_loop(); }",
        ),
        (
            "crates/worth-topo/src/relapse.rs",
            "fn relapse() { local_graph_traversal(); }",
        ),
        (
            "crates/worth-spatial/src/relapse.rs",
            "const RESIDUE: &str = \"local_spatial_evidence_graph_read_fallback\";",
        ),
        (
            "crates/worth-kernel/src/construction/relapse.rs",
            "const RESIDUE: &str = \"fabricated_graph_read_receipt\";",
        ),
    ] {
        let workspace = TempWorkspace::new("firewall_violation");
        workspace.write_source(region_path, forbidden_source);

        let err = super::super::closeout::closeout_for_workspace_root(
            &production_phase_seven_seed(),
            workspace.root(),
        )
        .expect_err("source firewall residue in a scanned region must fail closeout");

        assert_eq!(
            WorthGraphReadAccessHardDeletionErrorKind::SourceFirewallViolation,
            err.kind()
        );
    }
}
