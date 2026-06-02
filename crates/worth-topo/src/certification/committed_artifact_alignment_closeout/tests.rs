use crate::facade::{
    certify_topology_committed_artifact_alignment_closeout, TopologyCommittedArtifactAlignmentArea,
    TopologyCommittedArtifactAlignmentRow, TopologyCommittedArtifactAlignmentStatus,
};

#[test]
fn committed_artifact_alignment_closeout_certifies_all_phase_nine_areas() {
    let report = certify_topology_committed_artifact_alignment_closeout()
        .expect("worth-topo committed artifact alignment closeout should certify");

    for area in TopologyCommittedArtifactAlignmentArea::ALL {
        assert_eq!(
            report.status(area),
            TopologyCommittedArtifactAlignmentStatus::Closed,
            "phase nine closeout should mark `{}` closed",
            area.as_str()
        );
    }
    assert!(report.phase_nine_ready());
    assert!(report.closeout_digest().row_count > 0);
}

#[test]
fn committed_artifact_alignment_closeout_names_designated_survivors_for_every_area() {
    let report = certify_topology_committed_artifact_alignment_closeout()
        .expect("worth-topo committed artifact alignment closeout should certify");

    for row in report.rows() {
        let row: &TopologyCommittedArtifactAlignmentRow = row;
        assert!(
            !row.designated_survivor().is_empty(),
            "phase nine closeout should name a survivor for {:?}",
            row.area()
        );
    }
}

#[test]
fn committed_artifact_alignment_closeout_proves_phase_nine_completion_rule() {
    let report = certify_topology_committed_artifact_alignment_closeout()
        .expect("worth-topo committed artifact alignment closeout should certify");

    let live_artifact_row = report
        .rows()
        .iter()
        .find(|row: &&TopologyCommittedArtifactAlignmentRow| {
            row.area() == TopologyCommittedArtifactAlignmentArea::LiveArtifactContract
        })
        .expect("committed artifact alignment closeout should cover live artifact contract");

    assert!(
        live_artifact_row
            .reason()
            .contains("one accepted mutation projection plus topology-specific materialized aftermath"),
        "phase nine closeout should state that downstream topology workflows now follow one authoritative committed-artifact progression",
    );
}
