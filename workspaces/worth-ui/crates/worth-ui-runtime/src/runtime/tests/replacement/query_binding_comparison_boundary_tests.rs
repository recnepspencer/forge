use super::dependency_impact_narrowing_test_support::{
    lower_rust_authored_artifact, query_bound_artifact, query_bound_surface_app,
    surface_and_query_binding_module,
};
use super::query_binding_comparison_test_support::{
    denial_presentation_drift_query_app, lifecycle_drift_query_app, phase11_pipeline,
    query_artifact, standard_query_app,
};
use crate::runtime::replacement::query_binding::WorthUiQueryBindingEvidenceIndex;
use crate::runtime::{
    WorthUiNodeReplacementPlan, WorthUiQueryBindingComparisonDenial,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingPostureDriftFamily,
};

#[test]
fn same_query_owned_binding_identity_preserves_binding_comparison() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query binding comparison succeeds");
    let binding = comparison
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding exists");

    assert_eq!(
        binding.outcome(),
        WorthUiQueryBindingComparisonOutcome::PreserveMeaning
    );
    assert!(binding.posture_drifts().is_empty());
    assert_eq!(comparison.counters().active_bindings_indexed(), 1);
    assert_eq!(comparison.counters().candidate_bindings_indexed(), 1);
    assert_eq!(comparison.counters().bindings_compared(), 1);
    assert_eq!(comparison.counters().preserved_meaning_count(), 1);
}

#[test]
fn query_binding_lifecycle_drift_detected_before_subscription_reuse() {
    let active_app = standard_query_app();
    let candidate_app = lifecycle_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query binding comparison succeeds");
    let binding = comparison
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding exists");

    assert_eq!(
        binding.outcome(),
        WorthUiQueryBindingComparisonOutcome::RebindRequired
    );
    assert_eq!(
        binding.posture_drifts(),
        &[
            WorthUiQueryBindingPostureDriftFamily::SupportAdmission,
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility,
        ]
    );
    assert_eq!(comparison.counters().posture_drift_count(), 2);
    assert_eq!(comparison.counters().rebind_required_count(), 1);
}

#[test]
fn query_binding_comparison_does_not_use_ui_local_status_enums() {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query binding comparison succeeds");
    let binding = comparison
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding exists");

    assert_eq!(
        binding.posture_drifts(),
        &[WorthUiQueryBindingPostureDriftFamily::DenialPresentation]
    );
    assert_eq!(
        binding.outcome(),
        WorthUiQueryBindingComparisonOutcome::RebindRequired
    );
    assert_ne!(
        binding
            .active_posture()
            .expect("active posture")
            .denial_presentation(),
        binding
            .candidate_posture()
            .expect("candidate posture")
            .denial_presentation()
    );
}

#[test]
fn different_query_binding_identity_rebinds_or_retires() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.detail");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query binding comparison succeeds");

    assert_eq!(comparison.counters().bindings_compared(), 2);
    assert_eq!(comparison.counters().missing_candidate_binding_count(), 1);
    assert_eq!(comparison.counters().missing_active_binding_count(), 1);
    assert_eq!(
        comparison
            .binding_for_view_binding_id("workspace.view_binding.selection")
            .expect("active binding")
            .outcome(),
        WorthUiQueryBindingComparisonOutcome::MissingCandidateBinding
    );
    assert_eq!(
        comparison
            .binding_for_view_binding_id("workspace.view_binding.detail")
            .expect("candidate binding")
            .outcome(),
        WorthUiQueryBindingComparisonOutcome::MissingActiveBinding
    );
}

#[test]
fn changed_query_binding_placement_does_not_change_query_meaning() {
    let app = query_bound_surface_app();
    let standalone_binding = query_bound_artifact(&app, "workspace.view_binding.selection");
    let surface_binding = lower_rust_authored_artifact(
        &app,
        [surface_and_query_binding_module(
            "workspace.surface.command_save",
            "workspace.view_binding.selection",
        )],
    );

    let standalone = WorthUiQueryBindingEvidenceIndex::from_active_artifact(&standalone_binding);
    let surface = WorthUiQueryBindingEvidenceIndex::from_active_artifact(&surface_binding);

    let standalone_evidence = standalone
        .get("workspace.view_binding.selection")
        .expect("standalone binding evidence");
    let surface_evidence = surface
        .get("workspace.view_binding.selection")
        .expect("surface binding evidence");

    assert_eq!(standalone_evidence.identity(), surface_evidence.identity());
    assert_eq!(
        standalone_evidence
            .posture()
            .drift_families_against(surface_evidence.posture()),
        Vec::new()
    );
}

#[test]
fn query_binding_comparison_rejects_stale_plan_or_narrowing() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let stale_plan = WorthUiNodeReplacementPlan::new(
        plan.active_artifact_digest(),
        plan.candidate_artifact_digest() + 1,
        plan.classifications().to_vec(),
        plan.counters(),
    );
    let denial = runtime
        .compare_query_bindings(&stale_plan, &narrowing, &admitted)
        .expect_err("stale plan denies");

    match denial {
        WorthUiQueryBindingComparisonDenial::NodePlanDigestMismatch {
            plan_candidate_artifact_digest,
            admitted_candidate_artifact_digest,
            ..
        } => assert_ne!(
            plan_candidate_artifact_digest,
            admitted_candidate_artifact_digest
        ),
        other => panic!("unexpected denial: {other:?}"),
    }
}
