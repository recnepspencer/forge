use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, SpatialReplayFamilyIdentityAuthority,
};
use crate::replay_undo_semantic_graph::{
    prepare_spatial_replay_semantic_graph_request, SpatialReplaySemanticGraphAdmissionError,
    SpatialReplaySemanticGraphPreparationRequest,
};

use super::fixtures::{boolean_event_ledger_fixture, projection_receipt_fixture};

#[test]
fn spatial_replay_preparation_rejects_stage_index_identity_drift() {
    let fixture = boolean_event_ledger_fixture();
    let workload_handoff = fixture
        .workload_handoff
        .clone()
        .with_test_workload_stage_index_identity("foreign-stage-index");
    let request = SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        &fixture.authority,
        &fixture.execution_receipt,
        &workload_handoff,
    )
    .with_retained_replay_receipt(&fixture.matching_retained_replay_receipt);

    let error = prepare_spatial_replay_semantic_graph_request(request)
        .expect_err("foreign stage index should be rejected");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::StageIndexIdentityMismatch { .. }
    ));
}

#[test]
fn spatial_replay_preparation_rejects_stage_receipt_identity_drift() {
    let fixture = boolean_event_ledger_fixture();
    let workload_handoff = fixture
        .workload_handoff
        .clone()
        .with_test_stage_receipt_identity("foreign-stage-receipt");
    let request = SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        &fixture.authority,
        &fixture.execution_receipt,
        &workload_handoff,
    )
    .with_retained_replay_receipt(&fixture.matching_retained_replay_receipt);

    let error = prepare_spatial_replay_semantic_graph_request(request)
        .expect_err("foreign stage receipt should be rejected");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::StageReceiptIdentityMismatch { .. }
    ));
}

#[test]
fn spatial_replay_preparation_rejects_lookup_execution_receipt_drift() {
    let fixture = boolean_event_ledger_fixture();
    let foreign_fixture = projection_receipt_fixture();
    let request = SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        &fixture.authority,
        &foreign_fixture.execution_receipt,
        &fixture.workload_handoff,
    )
    .with_retained_replay_receipt(&fixture.matching_retained_replay_receipt);

    let error = prepare_spatial_replay_semantic_graph_request(request)
        .expect_err("foreign lookup execution receipt should be rejected");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::LookupExecutionReceiptMismatch { .. }
    ));
}
