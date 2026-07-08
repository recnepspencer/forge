use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::allocation_planning;
use super::frame_activation_gate_test_support::ready_activation_fixture;
use super::query_binding_comparison_test_support::{
    denial_presentation_drift_query_app, phase11_pipeline, query_artifact, standard_query_app,
};
use super::reload_failure_test_support::missing_artifact_candidate_denial;
use crate::runtime::{
    WorthUiCandidateAdmissionDenial, WorthUiDiagnosticProjectionHook,
    WorthUiDiagnosticRichnessPolicy, WorthUiDiagnosticRichnessTier, WorthUiDiagnosticSource,
    WorthUiExecutionLane, WorthUiExecutionLaneSupport, WorthUiQueryLiveRebindOutcome,
    WorthUiReloadCheckedStopPosture, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};

#[test]
fn same_reload_failure_produces_same_diagnostic_codes_and_ordering() {
    let fixture = ready_activation_fixture();
    let left = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let right = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());

    let left_report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&left)
        .materialize();
    let right_report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&right)
        .materialize();

    let left_rows = row_codes_and_families(&left_report);
    let right_rows = row_codes_and_families(&right_report);
    assert_eq!(left_rows, right_rows);
    assert_eq!(
        left_report.rows()[0].code(),
        WorthUiRuntimeDiagnosticCode::InvalidCandidateDenied
    );
    assert_eq!(
        left_report.rows()[0].family(),
        WorthUiRuntimeDiagnosticFamily::Reload
    );
}

#[test]
fn diagnostic_richness_does_not_change_active_plan_or_digest() {
    let fixture = ready_activation_fixture();
    let active_before = fixture.runtime.inspect_active();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());

    let minimal = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .with_policy(WorthUiDiagnosticRichnessPolicy::minimal())
        .materialize();
    let full = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .with_policy(WorthUiDiagnosticRichnessPolicy::full())
        .materialize();
    let support = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .with_policy(WorthUiDiagnosticRichnessPolicy::support())
        .materialize();

    assert_eq!(fixture.runtime.inspect_active(), active_before);
    for report in [&minimal, &full, &support] {
        assert_eq!(
            report.active_artifact_digest(),
            active_before.artifact_digest()
        );
        assert_eq!(
            report.active_plan_digest(),
            active_before.active_plan_digest()
        );
        assert_eq!(report.rows()[0].code(), minimal.rows()[0].code());
    }
    assert_eq!(minimal.counters().rich_materialization_count(), 0);
    assert_eq!(full.counters().rich_materialization_count(), 1);
    assert_eq!(support.counters().support_section_count(), 1);
}

#[test]
fn query_diagnostics_preserve_checked_stop_and_recovery_posture() {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let rebind_plan = runtime
        .plan_query_live_rebinds(&comparison, &plan, &narrowing, &admitted)
        .expect("query rebind records denial entry");
    let entry = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding planned");
    let WorthUiQueryLiveRebindOutcome::Deny(denial) = entry.outcome() else {
        panic!("denial-presentation drift must deny to preserve recovery");
    };

    let report = runtime
        .diagnostics()
        .for_query_recovery(denial)
        .with_policy(WorthUiDiagnosticRichnessPolicy::full())
        .materialize();

    assert_eq!(
        report.rows()[0].code(),
        WorthUiRuntimeDiagnosticCode::QueryRecoveryPreserved
    );
    let WorthUiDiagnosticSource::QueryStop {
        checked_stop_posture,
        evidence_digest,
    } = report.rows()[0].source()
    else {
        panic!("Query diagnostic must preserve Query stop posture");
    };
    assert_eq!(
        checked_stop_posture,
        WorthUiReloadCheckedStopPosture::query_recovery_preserved()
    );
    assert_ne!(evidence_digest, 0);
    assert_eq!(report.counters().query_link_count(), 1);
}

#[test]
fn diagnostic_richness_tiers_gate_report_materialization() {
    let fixture = ready_activation_fixture();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());

    let off = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .with_policy(WorthUiDiagnosticRichnessPolicy::off())
        .materialize();
    let standard = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .with_policy(WorthUiDiagnosticRichnessPolicy::standard())
        .materialize();
    let support = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .with_policy(WorthUiDiagnosticRichnessPolicy::support())
        .materialize();

    assert_eq!(
        off.materialization().tier(),
        WorthUiDiagnosticRichnessTier::Off
    );
    assert!(off.rows().is_empty());
    assert_eq!(off.counters().emitted_row_count(), 0);
    assert_eq!(
        standard.materialization().tier(),
        WorthUiDiagnosticRichnessTier::Standard
    );
    assert_eq!(standard.rows().len(), 1);
    assert_eq!(standard.counters().phase_reference_count(), 1);
    assert!(!standard.support_report().is_materialized());
    assert!(support.support_report().is_materialized());
    assert_eq!(support.support_report().section_count(), 1);
}

#[test]
fn diagnostics_never_depend_on_error_message_substrings() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "runtime-diagnostics.lane-admission");
    let support_without_query =
        WorthUiExecutionLaneSupport::without_lane_for_test(WorthUiExecutionLane::QueryBound);
    let denial = runtime
        .admit_execution_lanes(&planning, &support_without_query)
        .expect_err("unsupported Query lane denies");

    let report = runtime
        .diagnostics()
        .for_lane_admission(&denial)
        .materialize();

    assert_eq!(
        report.rows()[0].family(),
        WorthUiRuntimeDiagnosticFamily::LaneAdmission
    );
    assert_eq!(
        report.rows()[0].code(),
        WorthUiRuntimeDiagnosticCode::LaneAdmissionDenied
    );
    let WorthUiDiagnosticSource::LaneAdmission {
        lane,
        evidence_digest,
    } = report.rows()[0].source()
    else {
        panic!("lane diagnostic must preserve lane source");
    };
    assert_eq!(lane, Some(WorthUiExecutionLane::QueryBound));
    assert_ne!(evidence_digest, 0);
}

#[test]
fn diagnostic_projection_hook_cannot_create_runtime_truth() {
    let fixture = ready_activation_fixture();
    let active_before = fixture.runtime.inspect_active();
    let hook = WorthUiDiagnosticProjectionHook::projection("workspace.diagnostics.panel");

    let report = fixture
        .runtime
        .diagnostics()
        .for_projection_hook(&hook)
        .materialize();

    assert_eq!(fixture.runtime.inspect_active(), active_before);
    assert_eq!(
        report.rows()[0].family(),
        WorthUiRuntimeDiagnosticFamily::DiagnosticsProjection
    );
    assert_eq!(
        report.rows()[0].code(),
        WorthUiRuntimeDiagnosticCode::DiagnosticsProjectionAdmitted
    );
    assert_eq!(
        report.rows()[0].source().evidence_digest(),
        Some(hook.projection_digest())
    );
}

#[test]
fn typed_candidate_admission_diagnostic_family_is_not_a_reload_string() {
    let fixture = ready_activation_fixture();
    let denial = WorthUiCandidateAdmissionDenial::SnapshotMismatch {
        candidate_snapshot_digest: 1,
        active_snapshot_digest: 2,
    };

    let report = fixture
        .runtime
        .diagnostics()
        .for_candidate_admission(&denial)
        .materialize();

    assert_eq!(
        report.rows()[0].family(),
        WorthUiRuntimeDiagnosticFamily::CandidateAdmission
    );
    assert_eq!(
        report.rows()[0].code(),
        WorthUiRuntimeDiagnosticCode::CandidateAdmissionDenied
    );
    assert_ne!(report.rows()[0].family().as_str(), "reload");
}

fn row_codes_and_families(
    report: &crate::runtime::WorthUiRuntimeDiagnosticReport,
) -> Vec<(WorthUiRuntimeDiagnosticFamily, WorthUiRuntimeDiagnosticCode)> {
    report
        .rows()
        .iter()
        .map(|row| (row.family(), row.code()))
        .collect()
}
