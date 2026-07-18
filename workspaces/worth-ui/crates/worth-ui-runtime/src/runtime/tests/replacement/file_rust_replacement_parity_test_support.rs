use super::query_binding_comparison_test_support::standard_query_app;
use super::replacement_impact_test_support::{
    artifact_from_modules, impact_test_app, token_module,
};
use super::source_ingress_test_support::{runtime_from_artifact, rust_import_artifact};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::candidate::{
    file_authored_replacement_candidate, rust_authored_replacement_candidate,
};
use crate::runtime::{
    WorthUiCandidateAuthoringLane, WorthUiFileRustReplacementParityBoundary,
    WorthUiFileRustReplacementParityReceipt, WorthUiFileRustReplacementPipelineReport,
    WorthUiReplacementCandidate, WorthUiReplacementCause, WorthUiRuntime,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeLaunch,
};
use crate::source::WorthUiArtifact;

pub(super) fn parity_receipt() -> WorthUiFileRustReplacementParityReceipt {
    let file = replacement_report_for_lane(WorthUiCandidateAuthoringLane::FileAuthored);
    let rust = replacement_report_for_lane(WorthUiCandidateAuthoringLane::RustAuthored);
    WorthUiFileRustReplacementParityBoundary::compare(file, rust)
        .expect("file and rust reports prove parity")
}

pub(super) fn meaningful_token_parity_reports() -> (
    WorthUiFileRustReplacementPipelineReport,
    WorthUiFileRustReplacementPipelineReport,
) {
    let app = impact_test_app();
    let active = artifact_from_modules(&app, [token_module("theme.text.primary")]);
    let candidate_token = "theme.text.secondary";
    let file_report = replacement_report_for_artifact_against_active(
        &app,
        WorthUiCandidateAuthoringLane::FileAuthored,
        artifact_from_modules(&app, [token_module(candidate_token)]),
        active.clone(),
    );
    let rust_report = replacement_report_for_artifact_against_active(
        &app,
        WorthUiCandidateAuthoringLane::RustAuthored,
        artifact_from_modules(&app, [token_module(candidate_token)]),
        active,
    );
    (file_report, rust_report)
}

pub(super) fn replacement_report_for_lane(
    lane: WorthUiCandidateAuthoringLane,
) -> WorthUiFileRustReplacementPipelineReport {
    let mut runtime = runtime_from_artifact(rust_import_artifact());
    let candidate = candidate_for_lane(&runtime, lane);
    activate_replacement_for_report(&mut runtime, candidate)
}

fn replacement_report_for_artifact_against_active(
    app: &WorthUiApp,
    lane: WorthUiCandidateAuthoringLane,
    candidate_artifact: WorthUiArtifact,
    active_artifact: WorthUiArtifact,
) -> WorthUiFileRustReplacementPipelineReport {
    let mut runtime = app
        .launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(
            active_artifact,
        ))
        .expect("runtime launches");
    let candidate = candidate_for_artifact(app, lane, candidate_artifact);
    activate_replacement_for_report(&mut runtime, candidate)
}

fn activate_replacement_for_report(
    runtime: &mut WorthUiRuntime,
    candidate: WorthUiReplacementCandidate,
) -> WorthUiFileRustReplacementPipelineReport {
    runtime
        .activate_replacement_for_file_rust_parity_report(candidate)
        .expect("replacement parity pipeline completes")
}

pub(super) fn candidate_for_lane(
    _runtime: &WorthUiRuntime,
    lane: WorthUiCandidateAuthoringLane,
) -> WorthUiReplacementCandidate {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    candidate_for_artifact(&app, lane, rust_import_artifact())
}

fn candidate_for_artifact(
    app: &WorthUiApp,
    lane: WorthUiCandidateAuthoringLane,
    artifact: WorthUiArtifact,
) -> WorthUiReplacementCandidate {
    match lane {
        WorthUiCandidateAuthoringLane::FileAuthored => file_authored_replacement_candidate(
            artifact,
            app.capabilities().digest(),
            WorthUiReplacementCause::file_source_change(
                crate::source::WorthUiSourceModuleId::from_relative_path(std::path::Path::new(
                    "app/main.wui",
                ))
                .expect("test source module identity should be valid"),
                1,
            ),
        ),
        WorthUiCandidateAuthoringLane::RustAuthored => rust_authored_replacement_candidate(
            artifact,
            app.capabilities().digest(),
            WorthUiReplacementCause::rust_authored_input_change(1),
        ),
    }
    .expect("replacement candidate should seal")
}

pub(super) fn stale_snapshot_rust_candidate() -> WorthUiReplacementCandidate {
    rust_authored_replacement_candidate(
        rust_import_artifact(),
        standard_query_app().capabilities().digest(),
        WorthUiReplacementCause::rust_authored_input_change(5),
    )
    .expect("stale snapshot candidate seals before admission")
}

pub(super) fn report_with_artifact_comparison_outcome(
    report: WorthUiFileRustReplacementPipelineReport,
    outcome: WorthUiRuntimeArtifactComparisonOutcome,
) -> WorthUiFileRustReplacementPipelineReport {
    pipeline_report_with_test_overrides(
        report,
        PipelineReportTestOverride {
            artifact_comparison_outcome: Some(outcome),
            ..PipelineReportTestOverride::default()
        },
    )
}

pub(super) fn report_with_lane_support_digest(
    report: WorthUiFileRustReplacementPipelineReport,
    lane_support_digest: u64,
) -> WorthUiFileRustReplacementPipelineReport {
    pipeline_report_with_test_overrides(
        report,
        PipelineReportTestOverride {
            lane_support_digest: Some(lane_support_digest),
            ..PipelineReportTestOverride::default()
        },
    )
}

pub(super) fn report_with_previous_active_artifact_receipt_drift(
    report: WorthUiFileRustReplacementPipelineReport,
) -> WorthUiFileRustReplacementPipelineReport {
    let mut report_parts = report.into_parts();
    report_parts.swap_receipt = report_parts
        .swap_receipt
        .with_corrupted_previous_artifact_digest_for_test();
    WorthUiFileRustReplacementPipelineReport::new(report_parts)
}

fn pipeline_report_with_test_overrides(
    report: WorthUiFileRustReplacementPipelineReport,
    override_values: PipelineReportTestOverride,
) -> WorthUiFileRustReplacementPipelineReport {
    let mut parts = report.into_parts();
    if let Some(outcome) = override_values.artifact_comparison_outcome {
        parts.artifact_comparison_outcome = outcome;
    }
    if let Some(lane_support_digest) = override_values.lane_support_digest {
        parts.lane_support_digest = lane_support_digest;
    }
    WorthUiFileRustReplacementPipelineReport::new(parts)
}

#[derive(Default)]
struct PipelineReportTestOverride {
    artifact_comparison_outcome: Option<WorthUiRuntimeArtifactComparisonOutcome>,
    lane_support_digest: Option<u64>,
}
