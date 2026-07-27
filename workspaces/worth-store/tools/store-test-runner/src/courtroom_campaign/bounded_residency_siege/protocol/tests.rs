use super::parse_dirty;

const VALID_DIRTY_MARKER: &str = "\
BOUNDED_RESIDENCY_DIRTY 275 1 274 274 283 WriteCompleted ContinueSettlement \
ReconciledFromPhysicalTruth 1 0 1 1 1 1 0 true true 1 1 0 0 0";

#[test]
fn cancelled_writeback_accepts_only_reconciliation_from_physical_truth() {
    let parsed = parse_dirty(&[VALID_DIRTY_MARKER.to_owned()]).unwrap();

    assert_eq!(
        parsed.settlement.signal_evidence(),
        "reconciled-from-physical-truth"
    );
    for (forbidden, replacement) in [
        ("WriteCompleted", "NoEffect"),
        ("ContinueSettlement", "RetryPermitted"),
        ("ReconciledFromPhysicalTruth", "Committed"),
    ] {
        let marker = VALID_DIRTY_MARKER.replacen(forbidden, replacement, 1);
        let denial = parse_dirty(&[marker]).unwrap_err();
        assert!(
            denial.contains("cancelled writeback settlement"),
            "{forbidden} substitution produced `{denial}`"
        );
    }
}
