use super::{
    certify_topology_query_boundary_cleanup_closeout, TopologyQueryBoundaryCleanupArea,
    TopologyQueryBoundaryCleanupStatus,
};

#[test]
fn query_boundary_cleanup_closeout_certifies_all_acceptance_areas() {
    let report = certify_topology_query_boundary_cleanup_closeout()
        .expect("worth-topo query boundary cleanup closeout should certify");

    assert!(report.cleanup_complete());
    assert_eq!(
        report.rows().len(),
        TopologyQueryBoundaryCleanupArea::ALL.len()
    );
    for area in TopologyQueryBoundaryCleanupArea::ALL {
        assert_eq!(
            report.status(area),
            TopologyQueryBoundaryCleanupStatus::Closed
        );
    }
    assert!(report.closeout_digest().row_count > 0);
}

#[test]
fn query_boundary_cleanup_closeout_names_designated_survivors_for_every_area() {
    let report = certify_topology_query_boundary_cleanup_closeout()
        .expect("worth-topo query boundary cleanup closeout should certify");

    for row in report.rows() {
        assert!(!row.reason().is_empty());
        assert!(!row.row_digest().digest_hex.is_empty());
        assert!(
            row.designated_survivor().is_some(),
            "cleanup closeout row `{}` should name its designated survivor seam",
            row.area().as_str()
        );
    }
}

#[test]
fn query_boundary_cleanup_closeout_proves_phase_six_public_surface_completion_rule() {
    let report = certify_topology_query_boundary_cleanup_closeout()
        .expect("worth-topo query boundary cleanup closeout should certify");

    let public_surface_row = report
        .rows()
        .iter()
        .find(|row| row.area() == TopologyQueryBoundaryCleanupArea::PublicFacade)
        .expect("cleanup closeout should cover the public facade area");

    assert_eq!(
        public_surface_row.status(),
        TopologyQueryBoundaryCleanupStatus::Closed
    );
    assert_eq!(
        public_surface_row.designated_survivor(),
        Some("src/query_domain.rs")
    );
    assert!(
        public_surface_row
            .reason()
            .contains("topology-facing public surface is limited to query-domain entry"),
        "public-surface closeout reason should state the surviving query-domain entry boundary",
    );
}

#[test]
fn query_boundary_cleanup_closeout_names_phase_three_graph_authority_ledger_resolution() {
    let registry =
        include_str!("../../../../worth-kernel/src/query_graph_authority_gate/registry.rs");

    assert!(registry.contains("delete.ceremony-audit"));
    assert!(registry.contains(
        "operator catalog lowers to Query descriptor, registration, support pin, selection proof, and residue manifest"
    ));
    assert!(registry.contains("delete.handoff-only-helper"));
    assert!(registry.contains("topology.edge-split-blueprint"));
    assert!(registry.contains("residue.edge-split-blueprint-proof-obligation"));
    assert!(registry.contains("topology.loop-reconstruction-blueprint"));
    assert!(registry.contains("residue.loop-reconstruction-blueprint-proof-obligation"));
    assert!(registry.contains(
        "TopologyPrimitiveConstructionBirthGraphAuthorityProof is derived only from Query-backed compose execution"
    ));
    assert!(registry.contains(
        "raw proof-obligation vocabulary is crate-private; public facade exposes classification and Query surface posture only"
    ));
    assert!(registry.contains(
        "Phase 3 keeps catalog lowering as Query adoption/status and denies promotion to execution proof"
    ));
    assert!(registry.contains(
        "Phase 3 replaces handoff-only helpers with execution-derived graph authority proof"
    ));
}
