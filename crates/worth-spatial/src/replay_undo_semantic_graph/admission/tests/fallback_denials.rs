use crate::workload_platform::evidence_lookup_workload_cutover::{
    EvidenceLookupConsumedWorkloadHandoff, EvidenceLookupWorkloadCutoverErrorKind,
};

use super::fixtures::boolean_event_ledger_stage_proof;

#[test]
fn spatial_replay_handoff_denies_raw_evidence_row_scan_fallback() {
    let proof = boolean_event_ledger_stage_proof().with_test_raw_row_scan_count(1);

    let error = EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof)
        .expect_err("raw evidence row scan fallback must be denied");

    assert_eq!(
        error.kind(),
        EvidenceLookupWorkloadCutoverErrorKind::RawEvidenceFallbackDenied
    );
}

#[test]
fn spatial_replay_handoff_denies_broad_receipt_scan_fallback() {
    let proof = boolean_event_ledger_stage_proof().with_test_broad_receipt_scan_count(1);

    let error = EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof)
        .expect_err("broad receipt scan fallback must be denied");

    assert_eq!(
        error.kind(),
        EvidenceLookupWorkloadCutoverErrorKind::RawEvidenceFallbackDenied
    );
}

#[test]
fn spatial_replay_handoff_denies_caller_owned_scan_fallback() {
    let proof = boolean_event_ledger_stage_proof().with_test_caller_owned_scan_count(1);

    let error = EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof)
        .expect_err("caller-owned scan fallback must be denied");

    assert_eq!(
        error.kind(),
        EvidenceLookupWorkloadCutoverErrorKind::ScopeExpansionDenied
    );
}
