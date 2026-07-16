use super::query_binding_comparison_test_support::standard_query_app;
use super::replacement_impact_test_support::{
    artifact_from_modules, impact_test_app, token_module,
};
use super::source_ingress_test_support::{
    file_import_provider, runtime_from_artifact, rust_import_artifact, rust_import_provider,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::candidate::rust_authored_replacement_candidate;
use crate::runtime::{
    WorthUiFileRustReplacementParityBoundary, WorthUiFileRustReplacementParityReceipt,
    WorthUiFileRustReplacementPipelineReport, WorthUiReplacementCandidate, WorthUiReplacementCause,
    WorthUiRuntime, WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeLaunch,
    WorthUiSourceProvider, WorthUiWatchedArtifactInput, WorthUiWatcherEvent,
};
use crate::source::WorthUiArtifact;

pub(super) fn parity_receipt() -> WorthUiFileRustReplacementParityReceipt {
    let file = replacement_report_from_provider(file_import_provider());
    let rust = replacement_report_from_provider(rust_import_provider());
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
    let file_report = replacement_report_from_provider_against_active_for_app(
        &app,
        file_token_provider(candidate_token),
        active.clone(),
    );
    let rust_report = replacement_report_from_provider_against_active_for_app(
        &app,
        rust_token_provider(candidate_token),
        active,
    );
    (file_report, rust_report)
}

pub(super) fn replacement_report_from_provider(
    provider: WorthUiSourceProvider,
) -> WorthUiFileRustReplacementPipelineReport {
    let mut runtime = runtime_from_artifact(rust_import_artifact());
    let candidate = candidate_from_provider(&runtime, provider);
    activate_replacement_for_report(&mut runtime, candidate)
}

pub(super) fn replacement_report_from_provider_against_active_for_app(
    app: &WorthUiApp,
    provider: WorthUiSourceProvider,
    active_artifact: WorthUiArtifact,
) -> WorthUiFileRustReplacementPipelineReport {
    let mut runtime = app
        .launch_runtime(WorthUiRuntimeLaunch::from_canonical_artifact(
            active_artifact,
        ))
        .expect("runtime launches");
    let candidate = candidate_from_provider_for_app(&runtime, provider, app);
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

pub(super) fn candidate_from_provider(
    runtime: &WorthUiRuntime,
    provider: WorthUiSourceProvider,
) -> WorthUiReplacementCandidate {
    let app = WorthUi::app().freeze();
    candidate_from_provider_for_app(runtime, provider, &app)
}

fn candidate_from_provider_for_app(
    runtime: &WorthUiRuntime,
    provider: WorthUiSourceProvider,
    app: &WorthUiApp,
) -> WorthUiReplacementCandidate {
    let provider_id = provider.id().to_owned();
    let mut session = runtime.source_ingress(provider).start();
    let batch = session
        .ingest([WorthUiWatcherEvent::provider_revision(provider_id)])
        .expect("source ingress debounces");
    batch
        .lower_to_candidate_submission(app.capabilities())
        .expect("candidate submission lowers")
        .into_candidate()
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

fn file_token_provider(token_id: &str) -> WorthUiSourceProvider {
    WorthUiSourceProvider::filesystem_root(r"C:\workspace").with_file(
        "app/main.wui",
        format!(r#"token {token_id} = "{token_id}";"#),
    )
}

fn rust_token_provider(token_id: &str) -> WorthUiSourceProvider {
    let app = impact_test_app();
    let artifact = artifact_from_modules(&app, [token_module(token_id)]);
    WorthUiSourceProvider::rust_authored_artifact("rust-authored-token").with_artifact_input(
        WorthUiWatchedArtifactInput::from_rust_authored_artifact("token-provider", artifact),
    )
}
