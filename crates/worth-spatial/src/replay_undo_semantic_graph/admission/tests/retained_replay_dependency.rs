use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, SpatialReplayFamilyIdentityAuthority,
};
use crate::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    prepare_spatial_replay_semantic_graph_request, SpatialReplaySemanticGraphAdmissionError,
    SpatialReplaySemanticGraphPreparationRequest,
};

use super::fixtures::{boolean_event_ledger_fixture, boolean_event_ledger_request};

#[test]
fn retained_replay_required_family_rejects_missing_retained_replay_receipt() {
    let fixture = boolean_event_ledger_fixture();
    let request = SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        &fixture.authority,
        &fixture.execution_receipt,
        &fixture.workload_handoff,
    );
    let prepared_request =
        prepare_spatial_replay_semantic_graph_request(request).expect("prepared request");

    let error = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &prepared_request,
    )
    .expect_err("retained replay family should require retained replay receipt");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::MissingRequiredRetainedReplayReceipt { .. }
    ));
}

#[test]
fn retained_replay_required_family_accepts_matching_retained_replay_receipt() {
    let fixture = boolean_event_ledger_fixture();
    let prepared_request =
        prepare_spatial_replay_semantic_graph_request(boolean_event_ledger_request(&fixture))
            .expect("prepared request");

    let admitted_input = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &prepared_request,
    )
    .expect("matching retained replay should admit");

    assert!(admitted_input.retained_replay_receipt().is_some());
}

#[test]
fn retained_replay_required_family_rejects_foreign_retained_replay_receipt() {
    let fixture = boolean_event_ledger_fixture();
    let request = SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        &fixture.authority,
        &fixture.execution_receipt,
        &fixture.workload_handoff,
    )
    .with_retained_replay_receipt(&fixture.foreign_retained_replay_receipt);
    let prepared_request =
        prepare_spatial_replay_semantic_graph_request(request).expect("prepared request");

    let error = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &prepared_request,
    )
    .expect_err("foreign retained replay authority should be rejected");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::RetainedReplayReceiptMismatch { .. }
    ));
}
