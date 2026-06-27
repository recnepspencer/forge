use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, SpatialReplayFamilyIdentityAuthority,
};
use crate::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    prepare_spatial_replay_semantic_graph_request, SpatialReplaySemanticGraphAdmissionError,
    SpatialReplaySemanticGraphPreparationRequest,
};

use super::fixtures::{
    boolean_event_ledger_fixture, projection_receipt_fixture, projection_receipt_request,
};

#[test]
fn spatial_replay_admission_rejects_family_not_covered_by_lookup_handoff() {
    let fixture = boolean_event_ledger_fixture();
    let request = SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::projection_receipt(),
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
    .expect_err("uncovered family should be rejected before planning");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::MissingCoveredFamily { .. }
    ));
}

#[test]
fn lookup_receipt_only_family_rejects_retained_replay_authority_widening() {
    let fixture = projection_receipt_fixture();
    let request = projection_receipt_request(&fixture)
        .with_retained_replay_receipt(&fixture.foreign_retained_replay_receipt);
    let prepared_request =
        prepare_spatial_replay_semantic_graph_request(request).expect("prepared request");

    let error = admit_prepared_spatial_replay_semantic_graph_input(
        &fixture.family_catalog,
        &prepared_request,
    )
    .expect_err("lookup-only family should reject retained replay widening");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::UnexpectedRetainedReplayReceipt { .. }
    ));
}
