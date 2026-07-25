use super::assert_sources_exclude;

#[test]
fn ordinary_publication_planning_cannot_reconstruct_bootstrap_readers() {
    let forbidden = [
        "ManifestReader::with_loader(",
        "SegmentMembershipReader::with_loader(",
        "FreeSpaceReader::with_loader(",
        "ServingRecordArtifacts::new(",
    ];
    for root in [
        "src/physical_runtime/record_serving/publication",
        "src/physical_runtime/record_serving/planning",
        "src/physical_runtime/record_serving/access/segment_membership",
    ] {
        assert_sources_exclude(root, "canonical-planning-read-route", &forbidden);
    }
}
