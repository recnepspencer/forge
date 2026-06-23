use super::query_binding_comparison_test_support::standard_query_app;
use super::replacement_impact_test_support::{
    artifact_from_modules, impact_test_app, token_module,
};
use super::source_ingress_test_support::{
    file_import_provider, runtime_from_artifact, rust_import_artifact, rust_import_provider,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::atomic_plan_swap::WorthUiPlanSwapReceiptParts;
use crate::runtime::candidate::rust_authored_replacement_candidate;
use crate::runtime::file_rust_replacement_parity::WorthUiFileRustReplacementPipelineReportParts;
use crate::runtime::{
    WorthUiFileRustReplacementParityBoundary, WorthUiFileRustReplacementParityReceipt,
    WorthUiFileRustReplacementPipelineReport, WorthUiReplacementCandidate, WorthUiReplacementCause,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeHost, WorthUiRuntimeLaunch,
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
    runtime: &mut WorthUiRuntimeHost,
    candidate: WorthUiReplacementCandidate,
) -> WorthUiFileRustReplacementPipelineReport {
    runtime
        .activate_replacement_for_file_rust_parity_report(candidate)
        .expect("replacement parity pipeline completes")
}

pub(super) fn candidate_from_provider(
    runtime: &WorthUiRuntimeHost,
    provider: WorthUiSourceProvider,
) -> WorthUiReplacementCandidate {
    let app = WorthUi::app().freeze();
    candidate_from_provider_for_app(runtime, provider, &app)
}

fn candidate_from_provider_for_app(
    runtime: &WorthUiRuntimeHost,
    provider: WorthUiSourceProvider,
    app: &WorthUiApp,
) -> WorthUiReplacementCandidate {
    let provider_id = provider.id().to_owned();
    let mut session = runtime.source_ingress(provider).start();
    let batch = session
        .ingest([WorthUiWatcherEvent::provider_revision(provider_id)])
        .expect("source ingress debounces");
    batch
        .lower_to_candidate_submission(app.capabilities(), None)
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
    report: &WorthUiFileRustReplacementPipelineReport,
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
    report: &WorthUiFileRustReplacementPipelineReport,
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
    report: &WorthUiFileRustReplacementPipelineReport,
) -> WorthUiFileRustReplacementPipelineReport {
    let swap = report.swap_receipt();
    let drifted_swap = crate::runtime::WorthUiPlanSwapReceipt::new(WorthUiPlanSwapReceiptParts {
        previous_active_artifact_digest: swap.previous_active_artifact_digest() + 1,
        previous_active_plan_digest: swap.previous_active_plan_digest(),
        previous_active_snapshot_digest: swap.previous_active_snapshot_digest(),
        next_active_artifact_digest: swap.next_active_artifact_digest(),
        next_active_plan_digest: swap.next_active_plan_digest(),
        next_active_snapshot_digest: swap.next_active_snapshot_digest(),
        activation_gate_receipt: swap.activation_gate_receipt(),
        prior_valid_plan: swap.prior_valid_plan(),
        counters: swap.counters(),
    });
    pipeline_report_with_test_overrides(
        report,
        PipelineReportTestOverride {
            swap_receipt: Some(drifted_swap),
            ..PipelineReportTestOverride::default()
        },
    )
}

#[derive(Default)]
struct PipelineReportTestOverride {
    artifact_comparison_outcome: Option<WorthUiRuntimeArtifactComparisonOutcome>,
    lane_support_digest: Option<u64>,
    swap_receipt: Option<crate::runtime::WorthUiPlanSwapReceipt>,
}

fn pipeline_report_with_test_overrides(
    report: &WorthUiFileRustReplacementPipelineReport,
    override_values: PipelineReportTestOverride,
) -> WorthUiFileRustReplacementPipelineReport {
    WorthUiFileRustReplacementPipelineReport::new(WorthUiFileRustReplacementPipelineReportParts {
        authoring_lane: report.authoring_lane(),
        candidate_basis: report.candidate_basis(),
        provenance_handle: report.provenance_handle(),
        active_artifact_digest: report.active_artifact_digest(),
        candidate_artifact_digest: report.candidate_artifact_digest(),
        artifact_comparison_outcome: override_values
            .artifact_comparison_outcome
            .unwrap_or_else(|| report.artifact_comparison_outcome()),
        candidate_plan_digest: report.candidate_plan_digest(),
        lane_support_digest: override_values
            .lane_support_digest
            .unwrap_or_else(|| report.lane_support_digest()),
        plan_node_count: report.plan_node_count(),
        swap_receipt: override_values
            .swap_receipt
            .unwrap_or_else(|| report.swap_receipt()),
        counters: report.counters(),
    })
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
