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




