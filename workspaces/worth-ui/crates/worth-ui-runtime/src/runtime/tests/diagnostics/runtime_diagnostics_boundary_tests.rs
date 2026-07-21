use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::allocation_planning;
use super::reload_failure_test_support::missing_artifact_candidate_denial;
use crate::runtime::{
    WorthUiCandidateAdmissionDenial, WorthUiDiagnosticProjectionHook,
    WorthUiDiagnosticRichnessPolicy, WorthUiDiagnosticRichnessTier, WorthUiDiagnosticSource,
    WorthUiExecutionLane, WorthUiExecutionLaneSupport, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};

#[test]
fn same_reload_failure_produces_same_diagnostic_codes_and_ordering() {
    let fixture = activation_staging_inputs();
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
    let fixture = activation_staging_inputs();
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
fn diagnostic_richness_tiers_gate_report_materialization() {
    let fixture = activation_staging_inputs();
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
    let plan_input = inputs.reconstructive_plan_input(&[]);
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let planning = allocation_planning(&runtime, &pending, "runtime-diagnostics.lane-admission");
    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);
    let support_without_query =
        WorthUiExecutionLaneSupport::without_lane_for_test(WorthUiExecutionLane::QueryBound);
    let denial = runtime
        .admit_execution_lanes(&facts, &support_without_query)
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
    let fixture = activation_staging_inputs();
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
    let fixture = activation_staging_inputs();
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
