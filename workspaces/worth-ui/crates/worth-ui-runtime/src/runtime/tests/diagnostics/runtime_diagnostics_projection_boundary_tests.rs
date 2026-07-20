use super::activation_staging_test_support::activation_staging_inputs;
use super::reload_failure_test_support::missing_artifact_candidate_denial;
use super::runtime_diagnostics_projection_test_support::{
    foundational_frame_report, runtime_from_import_target,
};
use crate::runtime::{
    WorthUiDiagnosticRichnessPolicy, WorthUiDiagnosticSource,
    WorthUiDiagnosticsProjectionDenialReason, WorthUiDiagnosticsProjectionHook,
    WorthUiFrameCostSurfaceKind, WorthUiReplacementCandidateDenial, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};

#[test]
fn diagnostics_projection_preserves_runtime_diagnostic_identity() {
    let fixture = activation_staging_inputs();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .materialize();

    let projection = fixture
        .runtime
        .diagnostics_projection()
        .from_report(&report)
        .project()
        .expect("typed runtime report projects");

    assert_eq!(projection.rows(), report.rows());
    assert_eq!(
        projection.rows()[0].family(),
        WorthUiRuntimeDiagnosticFamily::Reload
    );
    assert_eq!(
        projection.rows()[0].code(),
        WorthUiRuntimeDiagnosticCode::InvalidCandidateDenied
    );
    assert_eq!(
        projection.rows()[0].source().evidence_digest(),
        report.rows()[0].source().evidence_digest()
    );
    assert_eq!(projection.counters().runtime_rows_consumed(), 1);
}

#[test]
fn diagnostics_projection_cannot_mutate_active_plan() {
    let fixture = activation_staging_inputs();
    let active_before = fixture.runtime.inspect_active();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .materialize();

    let projection = fixture
        .runtime
        .diagnostics_projection()
        .from_report(&report)
        .with_hook(WorthUiDiagnosticsProjectionHook::surface(
            "workspace.diagnostics.panel",
        ))
        .project()
        .expect("presentation-only hook admits");

    assert_eq!(fixture.runtime.inspect_active(), active_before);
    assert_eq!(
        projection.active_plan_digest(),
        active_before.active_plan_digest()
    );
    assert_eq!(projection.counters().authority_mutations(), 0);
    assert_eq!(projection.counters().hooks_applied(), 1);
}

#[test]
fn failed_reload_visible_without_blank_active_app() {
    let fixture = activation_staging_inputs();
    let active_before = fixture.runtime.inspect_active();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .materialize();

    let projection = fixture
        .runtime
        .diagnostics_projection()
        .from_report(&report)
        .project()
        .expect("failed reload projects over previous active app");

    assert_eq!(
        projection.reload_status().active_artifact_digest(),
        active_before.artifact_digest()
    );
    assert_eq!(
        projection.reload_status().active_plan_digest(),
        active_before.active_plan_digest()
    );
    let latest = projection
        .reload_status()
        .latest_failure()
        .expect("reload failure remains visible");
    assert_eq!(latest, &report.rows()[0]);
    assert!(matches!(
        latest.source(),
        WorthUiDiagnosticSource::ReloadFailure { .. }
    ));
    assert_eq!(fixture.runtime.inspect_active(), active_before);
}

#[test]
fn diagnostics_projection_rejects_hook_identity_rewrite() {
    let fixture = activation_staging_inputs();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .materialize();
    let hook = WorthUiDiagnosticsProjectionHook::identity_rewrite_attempt_for_test(
        "workspace.diagnostics.panel",
        "reload.hidden",
    );

    let denial = fixture
        .runtime
        .diagnostics_projection()
        .from_report(&report)
        .with_hook(hook)
        .project()
        .expect_err("identity rewrite hook must deny");

    assert_eq!(
        denial.reason(),
        WorthUiDiagnosticsProjectionDenialReason::HookAttemptedIdentityRewrite
    );
}

#[test]
fn frame_cost_surface_consumes_foundational_materialized_report() {
    let fixture = activation_staging_inputs();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .materialize();
    let frame_report = foundational_frame_report(2);

    let projection = fixture
        .runtime
        .diagnostics_projection()
        .from_report(&report)
        .with_frame_costs(&frame_report)
        .project()
        .expect("Foundational frame report projects");

    let rows = projection.frame_costs().rows();
    assert!(rows
        .iter()
        .any(|row| row.kind() == WorthUiFrameCostSurfaceKind::FoundationalCounter));
    assert!(rows
        .iter()
        .any(|row| row.kind() == WorthUiFrameCostSurfaceKind::FoundationalEvidence));
    assert_eq!(projection.counters().frame_cost_rows(), rows.len());
    assert!(rows.iter().all(|row| row.evidence_digest() != 0));
}

#[test]
fn projection_digest_changes_when_typed_frame_cost_input_changes() {
    let fixture = activation_staging_inputs();
    let failure = fixture
        .runtime
        .preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let report = fixture
        .runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .materialize();
    let low_cost_report = foundational_frame_report(2);
    let higher_cost_report = foundational_frame_report(5);

    let low_cost_projection = fixture
        .runtime
        .diagnostics_projection()
        .from_report(&report)
        .with_frame_costs(&low_cost_report)
        .project()
        .expect("low-cost frame report projects");
    let higher_cost_projection = fixture
        .runtime
        .diagnostics_projection()
        .from_report(&report)
        .with_frame_costs(&higher_cost_report)
        .project()
        .expect("higher-cost frame report projects");

    assert_ne!(
        low_cost_projection.frame_costs().source_digest(),
        higher_cost_projection.frame_costs().source_digest()
    );
    assert_ne!(
        low_cost_projection.projection_digest(),
        higher_cost_projection.projection_digest()
    );
}

#[test]
fn diagnostics_projection_rejects_report_from_different_runtime() {
    let runtime = runtime_from_import_target("app/panels/inspector.wui");
    let other_runtime = runtime_from_import_target("app/panels/settings.wui");
    let report = other_runtime
        .diagnostics()
        .for_invalid_candidate(WorthUiReplacementCandidateDenial::MissingArtifactDigest)
        .with_policy(WorthUiDiagnosticRichnessPolicy::standard())
        .materialize();

    let denial = runtime
        .diagnostics_projection()
        .from_report(&report)
        .project()
        .expect_err("cross-runtime report cannot project as current runtime truth");

    assert_eq!(
        denial.reason(),
        WorthUiDiagnosticsProjectionDenialReason::RuntimeReportDigestMismatch
    );
    assert_eq!(
        denial.active_plan_digest(),
        runtime.inspect_active().active_plan_digest()
    );
    assert_ne!(
        report.active_artifact_digest(),
        runtime.inspect_active().artifact_digest()
    );
}

#[test]
fn diagnostics_projection_rejects_plan_inspection_from_different_plan() {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    let failure = runtime.preserve_invalid_candidate_reload(missing_artifact_candidate_denial());
    let report = runtime
        .diagnostics()
        .for_reload_failure(&failure)
        .materialize();
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "runtime-diagnostics-projection.candidate-plan",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits complete diagnostic catalog");
    let boundary = runtime.traversal_frame_boundary_for_test();
    let mut candidate_inspection = None;
    runtime
        .activate_admitted_allocation_catalog_with_boundary_source(
            pending,
            admitted,
            |runtime, _, candidate_plan, lowering_facts| {
                candidate_inspection = Some(
                    runtime
                        .inspect_execution_plan(candidate_plan, lowering_facts)
                        .map_err(|_| crate::runtime::WorthUiAllocationCatalogActivationDenial::CertificationBoundary("plan inspection"))?,
                );
                Ok((boundary, None))
            },
        )
        .expect_err("unsafe boundary preserves active plan");
    let candidate_inspection =
        candidate_inspection.expect("canonical route exposes read-only plan inspection");

    let denial = runtime
        .diagnostics_projection()
        .from_report(&report)
        .with_plan_inspection(&candidate_inspection)
        .project()
        .expect_err("candidate plan inspection cannot describe active report");

    assert_eq!(
        denial.reason(),
        WorthUiDiagnosticsProjectionDenialReason::PlanInspectionAuthorityMismatch
    );
    assert_eq!(
        candidate_inspection.plan_digest().raw(),
        report.active_plan_digest()
    );
    assert_eq!(
        candidate_inspection.handle_arena_identity(),
        runtime.active_handle_arena_identity()
    );
    assert!(!runtime
        .active_plan_shares_lowering_identity_with(candidate_inspection.lowering_identity()));
}
