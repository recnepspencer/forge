use crate::replay_undo_semantic_graph::{
    admit_spatial_undo_semantic_graph_input, SpatialReplaySemanticGraphAdmissionError,
    SpatialUndoSemanticGraphAdmissionRequest,
};
use crate::undo_family_catalog::SpatialUndoFamilyIdentityAuthority;

use super::fixtures::{
    boolean_event_ledger_fixture, event_ledger_stage_index_product, projection_receipt_fixture,
};

#[test]
fn undo_family_requiring_lookup_handoff_rejects_missing_handoff() {
    let fixture = boolean_event_ledger_fixture();
    let stage_index_product = event_ledger_stage_index_product(&fixture.authority);

    let error =
        admit_spatial_undo_semantic_graph_input(SpatialUndoSemanticGraphAdmissionRequest::new(
            SpatialUndoFamilyIdentityAuthority::boolean_event_ledger().identity(),
            &fixture.authority,
            &fixture.execution_receipt,
            &stage_index_product,
        ))
        .expect_err("boolean event ledger rollback requires lookup-consumed workload handoff");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::MissingRequiredLookupConsumedWorkload { .. }
    ));
}

#[test]
fn lookup_receipt_only_undo_family_rejects_unnecessary_lookup_handoff() {
    let fixture = projection_receipt_fixture();
    let stage_index_product = event_ledger_stage_index_product(&fixture.authority);

    let error = admit_spatial_undo_semantic_graph_input(
        SpatialUndoSemanticGraphAdmissionRequest::new(
            SpatialUndoFamilyIdentityAuthority::projection_receipt().identity(),
            &fixture.authority,
            &fixture.execution_receipt,
            &stage_index_product,
        )
        .with_lookup_consumed_workload_handoff(&fixture.workload_handoff),
    )
    .expect_err("projection receipt rollback should reject widened lookup-consumed workload proof");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::UnexpectedLookupConsumedWorkload { .. }
    ));
}
