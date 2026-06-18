#[path = "public_api_planar_boolean_edge_splitting_workload_evidence_support.rs"]
mod workload_evidence_support;
use super::metaboss_support::MetabossEventExtractionSubject;
use super::reduced_pair_support;
use workload_evidence_support::{
    assert_split_ledger_rejects_manual_or_counterless_evidence,
    assert_split_ledger_satisfies_workload_requirement_for_7_4_consumption,
    assert_split_stage_requirement_maps_only_to_split_ledger_receipts,
};

#[test]
fn worth_workload_requires_boolean_split_receipt_for_7_4_consumption() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify(
            "phase7.3 completed split ledger workload evidence",
        );
        assert_split_ledger_satisfies_workload_requirement_for_7_4_consumption(&subject);
    });
}

#[test]
fn boolean_split_evidence_rejects_manual_or_counterless_rows() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify(
            "phase7.3 hostile split ledger workload evidence",
        );
        assert_split_ledger_rejects_manual_or_counterless_evidence(&subject);
    });
}

#[test]
fn boolean_split_stage_requirement_maps_only_to_split_receipts() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 split ledger only workload evidence");
        assert_split_stage_requirement_maps_only_to_split_ledger_receipts(&subject);
    });
}
