use super::file_rust_replacement_parity_test_support::{
    candidate_for_lane, meaningful_token_parity_reports, parity_receipt,
    replacement_report_for_lane, report_with_artifact_comparison_outcome,
    report_with_lane_support_digest, report_with_previous_active_artifact_receipt_drift,
    stale_snapshot_rust_candidate,
};
use super::source_ingress_test_support::{
    empty_artifact, runtime_from_artifact, rust_import_artifact,
};
use crate::runtime::{
    WorthUiCandidateAuthoringLane, WorthUiFileRustReplacementParityBoundary,
    WorthUiFileRustReplacementParityDenialReason, WorthUiRuntimeArtifactComparisonOutcome,
};

#[test]
fn file_and_rust_replacements_with_same_meaning_activate_equivalent_plans() {
    let receipt = parity_receipt();
    let semantic = receipt.semantic_receipt();

    assert_eq!(
        semantic.file_next_artifact_digest(),
        semantic.rust_next_artifact_digest()
    );
    assert_eq!(
        semantic.file_next_plan_digest(),
        semantic.rust_next_plan_digest()
    );
    assert_eq!(
        semantic.file_reconciliation_basis_digest(),
        semantic.rust_reconciliation_basis_digest()
    );
    assert_eq!(
        semantic.file_query_rebind_basis_digest(),
        semantic.rust_query_rebind_basis_digest()
    );
    assert!(semantic.activation_receipts_match());
    assert_eq!(receipt.counters().file_candidate_count(), 1);
    assert_eq!(receipt.counters().rust_candidate_count(), 1);
    assert_eq!(receipt.counters().candidate_admission_count(), 2);
    assert_eq!(receipt.counters().artifact_comparison_count(), 2);
    assert_eq!(receipt.counters().plan_lowering_count(), 2);
    assert_eq!(receipt.counters().lane_admission_count(), 2);
    assert_eq!(receipt.counters().plan_swap_count(), 2);
    assert_eq!(receipt.counters().source_reparse_on_swap_count(), 0);
    assert_eq!(receipt.counters().registry_rebuild_on_swap_count(), 0);
}

#[test]
fn file_and_rust_meaningful_replacements_activate_equivalent_swap_receipts() {
    let (file_report, rust_report) = meaningful_token_parity_reports();

    assert_ne!(
        file_report.active_artifact_digest(),
        file_report.candidate_artifact_digest()
    );
    let receipt = WorthUiFileRustReplacementParityBoundary::compare(file_report, rust_report)
        .expect("meaningful file and rust replacements have parity");

    assert!(receipt.semantic_receipt().activation_receipts_match());
}

#[test]
fn rust_replacement_cannot_bypass_candidate_admission_or_snapshot_support() {
    let mut runtime = runtime_from_artifact(empty_artifact());
    let before = runtime.inspect_active();

    let denial = runtime
        .activate_replacement_for_file_rust_parity_report(stale_snapshot_rust_candidate())
        .expect_err("stale snapshot candidate is denied before replacement work");

    assert_eq!(
        denial.reason(),
        WorthUiFileRustReplacementParityDenialReason::CandidateAdmissionDenied
    );
    assert_eq!(runtime.inspect_active(), before);
    assert_eq!(denial.counters().candidate_admission_count(), 1);
    assert_eq!(denial.counters().artifact_comparison_count(), 0);
    assert_eq!(denial.counters().plan_lowering_count(), 0);
    assert_eq!(denial.counters().plan_swap_count(), 0);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn authoring_lane_difference_preserved_only_as_diagnostic_provenance() {
    let mut file_runtime = runtime_from_artifact(rust_import_artifact());
    let file_candidate =
        candidate_for_lane(&file_runtime, WorthUiCandidateAuthoringLane::FileAuthored);
    let file_lane = file_candidate.authoring_lane();
    let file_provenance = file_candidate.provenance_handle();
    let file_basis = file_candidate.basis();
    let file_report = file_runtime
        .activate_replacement_for_file_rust_parity_report(file_candidate)
        .expect("file replacement activates");

    let mut rust_runtime = runtime_from_artifact(rust_import_artifact());
    let rust_candidate =
        candidate_for_lane(&rust_runtime, WorthUiCandidateAuthoringLane::RustAuthored);
    let rust_lane = rust_candidate.authoring_lane();
    let rust_provenance = rust_candidate.provenance_handle();
    let rust_basis = rust_candidate.basis();
    let rust_report = rust_runtime
        .activate_replacement_for_file_rust_parity_report(rust_candidate)
        .expect("rust replacement activates");

    assert_eq!(file_lane, WorthUiCandidateAuthoringLane::FileAuthored);
    assert_eq!(rust_lane, WorthUiCandidateAuthoringLane::RustAuthored);
    assert_ne!(file_provenance, rust_provenance);
    assert_eq!(file_basis, rust_basis);

    let receipt = WorthUiFileRustReplacementParityBoundary::compare(file_report, rust_report)
        .expect("different provenance still has semantic parity");
    assert_eq!(
        receipt.semantic_receipt().file_next_plan_digest(),
        receipt.semantic_receipt().rust_next_plan_digest()
    );
}

#[test]
fn rust_authored_candidate_cannot_inject_active_plan_nodes_directly() {
    let rust_report = replacement_report_for_lane(WorthUiCandidateAuthoringLane::RustAuthored);

    assert_eq!(
        rust_report.authoring_lane(),
        WorthUiCandidateAuthoringLane::RustAuthored
    );
    assert_eq!(rust_report.plan_node_count(), 0);
    assert_eq!(rust_report.counters().rust_active_plan_injection_count(), 0);
    assert_eq!(
        rust_report.counters().rust_direct_handle_injection_count(),
        0
    );
    assert_eq!(
        rust_report.counters().canonical_constructor_bypass_count(),
        0
    );
}

#[test]
fn parity_denies_when_a_rust_report_is_used_as_the_file_side() {
    let rust_left = replacement_report_for_lane(WorthUiCandidateAuthoringLane::RustAuthored);
    let rust_right = replacement_report_for_lane(WorthUiCandidateAuthoringLane::RustAuthored);

    let denial = WorthUiFileRustReplacementParityBoundary::compare(rust_left, rust_right)
        .expect_err("file side must be file-authored");

    assert_eq!(
        denial.reason(),
        WorthUiFileRustReplacementParityDenialReason::FileReportWasNotFileAuthored
    );
    assert_eq!(denial.counters().parity_comparison_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn parity_denies_when_a_file_report_is_used_as_the_rust_side() {
    let file_left = replacement_report_for_lane(WorthUiCandidateAuthoringLane::FileAuthored);
    let file_right = replacement_report_for_lane(WorthUiCandidateAuthoringLane::FileAuthored);

    let denial = WorthUiFileRustReplacementParityBoundary::compare(file_left, file_right)
        .expect_err("rust side must be rust-authored");

    assert_eq!(
        denial.reason(),
        WorthUiFileRustReplacementParityDenialReason::RustReportWasNotRustAuthored
    );
    assert_eq!(denial.counters().parity_comparison_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn parity_denies_candidate_basis_drift_before_comparing_artifacts() {
    let file_report = replacement_report_for_lane(WorthUiCandidateAuthoringLane::FileAuthored);
    let (_, rust_report_with_different_basis) = meaningful_token_parity_reports();

    let denial = WorthUiFileRustReplacementParityBoundary::compare(
        file_report,
        rust_report_with_different_basis,
    )
    .expect_err("different candidate basis must deny parity");

    assert_eq!(
        denial.reason(),
        WorthUiFileRustReplacementParityDenialReason::CandidateBasisMismatch
    );
    assert_eq!(denial.counters().parity_comparison_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn parity_denies_artifact_comparison_outcome_drift() {
    let file_report = replacement_report_for_lane(WorthUiCandidateAuthoringLane::FileAuthored);
    let rust_report = report_with_artifact_comparison_outcome(
        replacement_report_for_lane(WorthUiCandidateAuthoringLane::RustAuthored),
        WorthUiRuntimeArtifactComparisonOutcome::MeaningfullyDifferent,
    );

    let denial = WorthUiFileRustReplacementParityBoundary::compare(file_report, rust_report)
        .expect_err("comparison outcome drift must deny parity");

    assert_eq!(
        denial.reason(),
        WorthUiFileRustReplacementParityDenialReason::ArtifactComparisonMismatch
    );
    assert_eq!(denial.counters().parity_comparison_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn parity_denies_lane_receipt_drift_after_plan_equivalence_holds() {
    let (file_report, rust_report) = meaningful_token_parity_reports();
    let drifted_digest = rust_report.lane_support_digest().wrapping_add(1);
    let rust_report = report_with_lane_support_digest(rust_report, drifted_digest);

    let denial = WorthUiFileRustReplacementParityBoundary::compare(file_report, rust_report)
        .expect_err("lane receipt drift must deny parity");

    assert_eq!(
        denial.reason(),
        WorthUiFileRustReplacementParityDenialReason::LaneParityMismatch
    );
    assert_eq!(denial.counters().parity_comparison_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn parity_denies_activation_receipt_drift_beyond_next_plan_digest() {
    let (file_report, rust_report) = meaningful_token_parity_reports();
    let rust_report = report_with_previous_active_artifact_receipt_drift(rust_report);

    let denial = WorthUiFileRustReplacementParityBoundary::compare(file_report, rust_report)
        .expect_err("full swap receipt drift must deny parity");

    assert_eq!(
        denial.reason(),
        WorthUiFileRustReplacementParityDenialReason::ActivationReceiptMismatch
    );
    assert_eq!(denial.counters().parity_comparison_count(), 1);
    assert_eq!(denial.counters().denial_count(), 1);
}
