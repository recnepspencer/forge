use super::{
    PlanarBooleanEdgeSplitReplayParityDenial, PlanarBooleanEdgeSplitReplayParityDenialKind,
    PlanarBooleanEdgeSplitReplayParityRow, PlanarBooleanEdgeSplitReplayParityRowKind,
};

#[test]
fn replay_parity_rows_certify_only_matching_identities() {
    let row = PlanarBooleanEdgeSplitReplayParityRow::new(
        PlanarBooleanEdgeSplitReplayParityRowKind::DecisionLogReceipt,
        "decision-log",
        "decision-log",
    );

    assert!(row.certifies_match());
    assert_eq!(
        row.kind(),
        PlanarBooleanEdgeSplitReplayParityRowKind::DecisionLogReceipt
    );
}

#[test]
fn replay_parity_denial_is_typed_and_machine_readable() {
    let denial = PlanarBooleanEdgeSplitReplayParityDenial::new(
        PlanarBooleanEdgeSplitReplayParityDenialKind::ReplayOperationalTruthMismatch,
        "operational-truth",
        "expected",
        "observed",
        "operational truth must match across replay",
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanEdgeSplitReplayParityDenialKind::ReplayOperationalTruthMismatch
    );
    assert_eq!(denial.rejected_identity(), "operational-truth");
    assert_eq!(denial.expected_identity(), "expected");
    assert_eq!(denial.observed_identity(), "observed");
}
