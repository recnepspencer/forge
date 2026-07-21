use super::dependency_impact_narrowing_test_support::{
    lower_rust_authored_artifact, query_bound_artifact, query_bound_surface_app,
    surface_and_query_binding_module,
};
use super::query_binding_comparison_test_support::{
    denial_presentation_drift_query_app, lifecycle_drift_query_app, mixed_change_query_apps,
    phase11_pipeline, query_artifact, query_artifact_with_bindings, standard_query_app,
    wide_query_app,
};
use crate::runtime::replacement::query_binding::WorthUiQueryBindingEvidenceIndex;
use crate::runtime::{
    WorthUiNodeReplacementPlan, WorthUiQueryBindingComparisonDenial,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingUiRequirementsDriftFamily,
};

#[test]
fn unchanged_query_binding_is_not_revisited() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query binding comparison succeeds");
    assert!(comparison.entries().is_empty());
    assert_eq!(comparison.counters().active_bindings_indexed(), 0);
    assert_eq!(comparison.counters().candidate_bindings_indexed(), 0);
    assert_eq!(comparison.counters().bindings_compared(), 0);
    assert_eq!(comparison.counters().preserved_meaning_count(), 0);
    assert_eq!(comparison.counters().rebind_required_count(), 0);
}

#[test]
fn ui_lifecycle_requirement_drift_stays_visible_when_query_authority_is_missing() {
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
        binding.ui_requirement_drifts(),
        &[WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration]
    );
    assert_eq!(comparison.counters().ui_requirement_drift_count(), 1);
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
        binding.ui_requirement_drifts(),
        &[WorthUiQueryBindingUiRequirementsDriftFamily::DenialPresentation]
    );
    assert_eq!(
        binding.outcome(),
        WorthUiQueryBindingComparisonOutcome::RebindRequired
    );
    assert_ne!(
        binding
            .active_ui_requirements()
            .expect("active posture")
            .denial_presentation(),
        binding
            .candidate_ui_requirements()
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
            .ui_requirements()
            .drift_families_against(surface_evidence.ui_requirements()),
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

#[test]
fn one_query_binding_change_is_independent_of_unrelated_application_width() {
    const UNRELATED_BINDING_COUNT: usize = 192;
    const CHANGED_BINDING_INDEX: usize = 96;
    let app = wide_query_app(UNRELATED_BINDING_COUNT);
    let active_ids = (0..UNRELATED_BINDING_COUNT)
        .map(|index| format!("workspace.view_binding.item_{index:03}"))
        .collect::<Vec<_>>();
    let mut candidate_ids = active_ids.clone();
    candidate_ids[CHANGED_BINDING_INDEX] = "workspace.view_binding.replacement".to_owned();
    let active_refs = active_ids.iter().map(String::as_str).collect::<Vec<_>>();
    let candidate_refs = candidate_ids.iter().map(String::as_str).collect::<Vec<_>>();
    let active = query_artifact_with_bindings(&app, &active_refs);
    let candidate = query_artifact_with_bindings(&app, &candidate_refs);
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("wide Query binding comparison succeeds");

    assert_eq!(comparison.counters().active_bindings_indexed(), 1);
    assert_eq!(comparison.counters().candidate_bindings_indexed(), 1);
    assert_eq!(comparison.counters().bindings_compared(), 2);
    assert_eq!(comparison.counters().affected_query_invalidation_count(), 8);
    assert_eq!(comparison.entries().len(), 2);
    assert!(comparison
        .binding_for_view_binding_id(&active_ids[CHANGED_BINDING_INDEX])
        .is_some());
    assert!(comparison
        .binding_for_view_binding_id("workspace.view_binding.replacement")
        .is_some());
}

#[test]
fn structural_delta_does_not_hide_same_identity_requirement_drift() {
    let (active_app, candidate_app) = mixed_change_query_apps();
    let active = query_artifact_with_bindings(
        &active_app,
        &[
            "workspace.view_binding.selection",
            "workspace.view_binding.detail",
        ],
    );
    let candidate = query_artifact_with_bindings(
        &candidate_app,
        &[
            "workspace.view_binding.selection",
            "workspace.view_binding.replacement",
        ],
    );
    let active_index = WorthUiQueryBindingEvidenceIndex::from_active_artifact(&active);
    let candidate_index = WorthUiQueryBindingEvidenceIndex::from_active_artifact(&candidate);
    assert_eq!(
        active_index
            .get("workspace.view_binding.selection")
            .expect("active selection evidence")
            .ui_requirements()
            .drift_families_against(
                candidate_index
                    .get("workspace.view_binding.selection")
                    .expect("candidate selection evidence")
                    .ui_requirements(),
            ),
        vec![WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration]
    );
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("mixed structural and requirement drift comparison succeeds");

    let selection = comparison
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("same-identity selection drift remains in the affected union");
    assert_eq!(
        selection.ui_requirement_drifts(),
        &[WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration]
    );
    assert!(comparison
        .binding_for_view_binding_id("workspace.view_binding.detail")
        .is_some());
    assert!(comparison
        .binding_for_view_binding_id("workspace.view_binding.replacement")
        .is_some());
    assert_eq!(comparison.counters().bindings_compared(), 3);
    assert_eq!(
        comparison.counters().affected_query_invalidation_count(),
        12
    );
}
