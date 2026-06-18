use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_reconstruction_subject, LoopFixtureEntryOrder,
};

use super::{PlanarBooleanLoopReconstructionRequest, PlanarBooleanLoopReconstructionRequestInput};

#[test]
fn loop_reconstruction_request_preserves_split_receipt_and_request_lineage() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = PlanarBooleanLoopReconstructionRequest::admit(
        PlanarBooleanLoopReconstructionRequestInput::from_split_consumption(
            &subject.loop_split_consumption,
        ),
    )
    .expect("loop reconstruction request should admit from loop split consumption");

    assert_eq!(
        request.loop_split_consumption_identity(),
        subject.loop_split_consumption.consumption_identity()
    );
    assert_eq!(
        request.split_ledger_receipt_identity(),
        subject.split_ledger_result.receipt().receipt_identity()
    );
    assert_eq!(
        request.split_request_identity(),
        subject.request_subject.request.split_request_identity()
    );
    assert_eq!(
        request.workload_stage_index_identity(),
        subject
            .loop_split_consumption
            .workload_stage_index_identity()
    );
    assert_eq!(request.counters().split_consumption_products_consumed(), 1);
    assert_eq!(
        request.counters().split_chain_rows_bound(),
        subject
            .loop_split_consumption
            .counters()
            .receipts_consumed()
    );
    assert_eq!(request.counters().missing_authority_rejected(), 0);
    assert!(request.certifies_loop_reconstruction_request());
}

#[test]
fn loop_reconstruction_request_identity_is_replay_stable_for_equivalent_split_lineage() {
    let canonical = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Replayed);

    let canonical_request = canonical.admit_loop_request();
    let replayed_request = replayed.admit_loop_request();

    assert_eq!(
        canonical_request.request_identity(),
        replayed_request.request_identity()
    );
    assert_eq!(
        canonical_request.split_ledger_receipt_identity(),
        replayed_request.split_ledger_receipt_identity()
    );
    assert_eq!(
        canonical_request.split_request_identity(),
        replayed_request.split_request_identity()
    );
    assert_eq!(canonical_request.counters(), replayed_request.counters());
}
