use forge_relational::facade::runtime::RelationalRuntimeApi;
use forge_relational::facade::transactions::CommitLog;

use crate::data::bootstrap::worth_bootstrap_schema_registry;
use crate::data::seed::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};
use crate::data::tracing::{
    WorthAuthorityTraceAnchor, WorthAuthorityTraceEvidence, WorthBoundaryEnvelope,
    WorthBoundaryFailure, WorthDecisionTrace, WorthIntegrityMarkers, WorthNamedCounter,
    WorthPerformanceAccounting,
};

#[test]
fn authority_trace_evidence_summarizes_commit_logs() {
    let mut published = CommitLog::new();
    published.begin_phase(forge_relational::facade::transactions::CommitPhase::DraftPreparation);
    published.record_commit_published(
        forge_relational::facade::history::CommitId(1),
        "main",
    );
    let mut rejected = CommitLog::new();
    rejected.begin_phase(forge_relational::facade::transactions::CommitPhase::DraftPreparation);
    rejected.record_rejection(
        forge_relational::facade::transactions::CommitPhase::DraftPreparation,
        None,
        None,
        "blocked",
    );

    let evidence = WorthAuthorityTraceEvidence::from_commit_logs(
        forge_relational::facade::history::BranchId("main".to_string()),
        vec![published, rejected],
    );

    assert_eq!(evidence.commit_count, 2);
    assert_eq!(evidence.published_commit_count, 1);
    assert_eq!(evidence.total_phase_count, 2);
}

#[test]
fn authority_trace_anchor_tracks_runtime_coordinates() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(
            worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
        )
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "trace-anchor",
        &WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 3 },
    )
    .expect("seed worth topology");
    let anchor = WorthAuthorityTraceAnchor::from_commit_results(
        verified.branch_id.clone(),
        &verified.commits,
    );

    assert_eq!(anchor.branch_id, verified.branch_id);
    assert_eq!(anchor.transaction_ids.len(), verified.commits.len());
    assert_eq!(anchor.runtime_instance_ids.len(), verified.commits.len());
    assert_eq!(
        anchor.commit_ids.last(),
        verified.commits.last().map(|commit| &commit.commit.commit_id)
    );
    assert_eq!(
        anchor.snapshot_ids.last(),
        verified.commits.last().map(|commit| &commit.snapshot.snapshot_id)
    );
    let read_view = anchor
        .open_latest_snapshot(&runtime)
        .expect("anchor should reopen latest snapshot");
    assert!(!read_view.entities().is_empty());
}

#[test]
fn boundary_envelope_helpers_preserve_trace_metadata_while_mapping_primary_result() {
    let envelope = WorthBoundaryEnvelope::success(
        7usize,
        Vec::new(),
        WorthDecisionTrace::default(),
        WorthIntegrityMarkers::default(),
        WorthPerformanceAccounting::new([WorthNamedCounter::new("test.counter", 3)]),
    );

    let mapped = envelope
        .map_primary_result(|value| value.to_string())
        .with_performance_accounting(WorthPerformanceAccounting::new([WorthNamedCounter::new(
            "test.counter",
            5,
        )]));

    assert_eq!(mapped.primary_result(), "7");
    assert_eq!(mapped.performance_accounting().counters[0].value, 5);
    assert!(mapped.warnings().is_empty());
}

#[test]
fn boundary_failure_helpers_preserve_trace_metadata_while_mapping_error() {
    let failure = WorthBoundaryFailure::failure(
        "boom",
        Vec::new(),
        WorthDecisionTrace::default(),
        WorthIntegrityMarkers::default(),
        WorthPerformanceAccounting::default(),
    );

    let mapped = failure.map_error(|error| format!("wrapped:{error}"));

    assert_eq!(mapped.error(), "wrapped:boom");
    assert!(mapped.warnings().is_empty());
}
