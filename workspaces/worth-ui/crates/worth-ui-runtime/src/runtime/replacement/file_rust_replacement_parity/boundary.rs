use crate::runtime::{
    WorthUiCandidateAuthoringLane, WorthUiFileRustReplacementParityCounters,
    WorthUiFileRustReplacementParityDenial, WorthUiFileRustReplacementParityDenialReason,
    WorthUiFileRustReplacementParityReceipt, WorthUiFileRustReplacementPipelineReport,
    WorthUiFileRustReplacementSemanticReceipt,
};

pub struct WorthUiFileRustReplacementParityBoundary;

impl WorthUiFileRustReplacementParityBoundary {
    pub fn compare(
        file_report: WorthUiFileRustReplacementPipelineReport,
        rust_report: WorthUiFileRustReplacementPipelineReport,
    ) -> Result<WorthUiFileRustReplacementParityReceipt, WorthUiFileRustReplacementParityDenial>
    {
        let mut counters = file_report.counters().merge(rust_report.counters());
        counters.record_parity_comparison();

        if file_report.authoring_lane() != WorthUiCandidateAuthoringLane::FileAuthored {
            return Err(denial(
                WorthUiFileRustReplacementParityDenialReason::FileReportWasNotFileAuthored,
                counters,
            ));
        }
        if rust_report.authoring_lane() != WorthUiCandidateAuthoringLane::RustAuthored {
            return Err(denial(
                WorthUiFileRustReplacementParityDenialReason::RustReportWasNotRustAuthored,
                counters,
            ));
        }
        if file_report.candidate_basis() != rust_report.candidate_basis() {
            return Err(denial(
                WorthUiFileRustReplacementParityDenialReason::CandidateBasisMismatch,
                counters,
            ));
        }
        if file_report.artifact_comparison_outcome() != rust_report.artifact_comparison_outcome()
            || file_report.active_artifact_digest() != rust_report.active_artifact_digest()
            || file_report.candidate_artifact_digest() != rust_report.candidate_artifact_digest()
        {
            return Err(denial(
                WorthUiFileRustReplacementParityDenialReason::ArtifactComparisonMismatch,
                counters,
            ));
        }

        let semantic =
            WorthUiFileRustReplacementSemanticReceipt::from_reports(&file_report, &rust_report);
        if !semantic.artifact_digests_match() || !semantic.plan_digests_match() {
            return Err(denial(
                WorthUiFileRustReplacementParityDenialReason::ExecutionPlanParityMismatch,
                counters,
            ));
        }
        if !semantic.lane_receipts_match() {
            return Err(denial(
                WorthUiFileRustReplacementParityDenialReason::LaneParityMismatch,
                counters,
            ));
        }
        if !semantic.activation_receipts_match() {
            return Err(denial(
                WorthUiFileRustReplacementParityDenialReason::ActivationReceiptMismatch,
                counters,
            ));
        }

        Ok(WorthUiFileRustReplacementParityReceipt::new(
            file_report,
            rust_report,
            semantic,
            counters,
        ))
    }
}

fn denial(
    reason: WorthUiFileRustReplacementParityDenialReason,
    counters: WorthUiFileRustReplacementParityCounters,
) -> WorthUiFileRustReplacementParityDenial {
    WorthUiFileRustReplacementParityDenial::new(reason, counters)
}
