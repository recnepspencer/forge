use worth_query::facade::certification::{
    WorthQueryPublicBridgeProjectionConsumptionEvidence, WorthQueryPublicBridgeReaderLanePosture,
    WorthQueryPublicBridgeReaderLaneSabotageOutcome,
};

use crate::support;

use support::public_bridge_runtime::{
    certify_public_bridge_hostile_schedule, direct_materialization_read_count,
    public_bridge_certification_inventory, public_bridge_certification_inventory_paths,
    public_graph_support_profile, reset_public_bridge_runtime_bootstrap_invocations,
    sabotaged_public_bridge_certification_inventory, PublicBridgeRuntimeBootstrapPath,
    PublicBridgeRuntimeHarness,
};

#[test]
fn public_bridge_reader_lane_honesty_closure_test() {
    reset_public_bridge_runtime_bootstrap_invocations();

    let common = certify_public_bridge_hostile_schedule(
        &PublicBridgeRuntimeHarness::new(),
        PublicBridgeRuntimeBootstrapPath::Common,
        public_graph_support_profile(),
    );
    let common_replay = certify_public_bridge_hostile_schedule(
        &PublicBridgeRuntimeHarness::new(),
        PublicBridgeRuntimeBootstrapPath::Common,
        public_graph_support_profile(),
    );
    let builder = certify_public_bridge_hostile_schedule(
        &PublicBridgeRuntimeHarness::new(),
        PublicBridgeRuntimeBootstrapPath::Builder,
        public_graph_support_profile(),
    );
    let builder_replay = certify_public_bridge_hostile_schedule(
        &PublicBridgeRuntimeHarness::new(),
        PublicBridgeRuntimeBootstrapPath::Builder,
        public_graph_support_profile(),
    );

    assert_eq!(common.digest(), builder.digest());
    assert_eq!(common.digest(), common_replay.digest());
    assert_eq!(builder.digest(), builder_replay.digest());
    assert_eq!(
        common.reader_lane().digest(),
        builder.reader_lane().digest()
    );
    assert_eq!(
        common.reader_lane().digest(),
        common_replay.reader_lane().digest()
    );
    assert_eq!(
        builder.reader_lane().digest(),
        builder_replay.reader_lane().digest()
    );
    assert_eq!(
        common.posture(),
        WorthQueryPublicBridgeReaderLanePosture::Closed
    );
    assert_eq!(
        builder.posture(),
        WorthQueryPublicBridgeReaderLanePosture::Closed
    );
    assert_eq!(common.reader_lane().direct_materialization_read_count(), 0);
    assert_eq!(builder.reader_lane().direct_materialization_read_count(), 0);
    assert_eq!(common.reader_lane().projection_receipt_digests().len(), 3);
    assert_eq!(builder.reader_lane().projection_receipt_digests().len(), 3);
    assert_projection_reads_are_real(common.reader_lane().projection_reads());
    assert_eq!(
        common.reader_lane().projection_reads(),
        builder.reader_lane().projection_reads()
    );
    assert_eq!(common.reader_lane().published_artifact_digests().len(), 4);
    assert_eq!(builder.reader_lane().published_artifact_digests().len(), 4);
    assert!(common.reader_lane().sabotage().rejected());
    assert!(builder.reader_lane().sabotage().rejected());
    assert!(matches!(
        common.reader_lane().sabotage().outcome(),
        WorthQueryPublicBridgeReaderLaneSabotageOutcome::Rejected(_)
    ));
}

#[test]
fn public_bridge_reader_lane_inventory_rejects_direct_materialization_reads() {
    let sabotaged_source = r#"
        artifact
            .published_binding()
            .unwrap()
            .materialization_by_name(view)
            .unwrap()
            .rows()
    "#;

    assert_eq!(
        public_bridge_certification_inventory_paths(),
        vec![
            "tests/support/public_bridge_runtime/hostile_certification.rs".to_string(),
            "tests/support/public_bridge_runtime/reader_lane_honesty/projection_reader.rs"
                .to_string(),
        ]
    );
    assert!(public_bridge_certification_inventory()
        .forbidden_findings()
        .is_empty());
    assert!(sabotaged_public_bridge_certification_inventory()
        .forbidden_findings()
        .iter()
        .any(|finding| finding.path().contains("projection_reader.rs")
            && finding.line() > 0
            && !finding.matched_text().is_empty()));
    assert!(direct_materialization_read_count(sabotaged_source) > 0);
}

fn assert_projection_reads_are_real(reads: &[WorthQueryPublicBridgeProjectionConsumptionEvidence]) {
    let titles = reads
        .iter()
        .map(|read| read.consumed_title())
        .collect::<Vec<_>>();
    assert_eq!(titles, vec!["Task One", "Task Two", "Task Three"]);
    for read in reads {
        assert!(!read.receipt_digest().is_empty());
        assert!(!read.fact_set_digest().is_empty());
        assert!(!read.source_identity().is_empty());
        assert_eq!(read.requested_field(), "title.value");
        assert_eq!(read.extracted_fact_count(), 1);
    }
}
