use super::query_binding_comparison_test_support::{
    denial_presentation_drift_query_app, lifecycle_drift_query_app, phase11_pipeline,
    query_artifact, standard_query_app,
};
use crate::runtime::{
    WorthUiNodeReplacementPlan, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirementReason, WorthUiQueryBindingUiRequirementsDriftFamily,
    WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlanDenial,
};

#[test]
fn unchanged_query_binding_requires_no_rebind_work() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let rebind_plan = query_live_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    assert!(rebind_plan.entries().is_empty());
    assert_eq!(rebind_plan.counters().preserved_binding_count(), 0);
    assert_eq!(rebind_plan.counters().rebound_binding_count(), 0);
    assert_eq!(rebind_plan.counters().denied_binding_count(), 0);
}

#[test]
fn lifecycle_presentation_drift_stays_visible_when_query_authority_is_missing() {
    let active_app = standard_query_app();
    let candidate_app = lifecycle_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);

    let rebind_plan = query_live_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    let entry = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding planned");

    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Rebind(rebind) => {
            assert_eq!(
                rebind.reason(),
                WorthUiQueryBindingRebindReason::QueryAuthorityChanged
            );
            assert_eq!(
                rebind.drift_families(),
                &[WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration]
            );
        }
        other => panic!("missing Query authority must rebind: {other:?}"),
    }
    assert_eq!(rebind_plan.counters().rebound_binding_count(), 1);
}

#[test]
fn ui_local_denial_presentation_does_not_replace_query_authority() {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);

    let rebind_plan = query_live_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    let entry = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding planned");

    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Rebind(rebind) => {
            assert_eq!(
                rebind.reason(),
                WorthUiQueryBindingRebindReason::QueryAuthorityChanged
            );
            assert_eq!(
                rebind.drift_families(),
                &[WorthUiQueryBindingUiRequirementsDriftFamily::DenialPresentation]
            );
        }
        other => panic!("missing Query authority must rebind: {other:?}"),
    }
    assert_eq!(rebind_plan.counters().denied_binding_count(), 0);
}

#[test]
fn ui_lifecycle_drift_does_not_hide_missing_query_authority() {
    let active_app = standard_query_app();
    let candidate_app = lifecycle_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("comparison succeeds");
    let rebind_plan = runtime
        .plan_query_live_rebinds(&comparison, &plan, &narrowing, &admitted)
        .expect("live rebind plan succeeds");

    let entry = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding planned");
    assert!(
        matches!(entry.outcome(), WorthUiQueryLiveRebindOutcome::Rebind(rebind) if rebind.reason() == WorthUiQueryBindingRebindReason::QueryAuthorityChanged)
    );
}

#[test]
fn candidate_and_active_only_bindings_are_not_silent_preservations() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.detail");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let rebind_plan = query_live_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    let fresh = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.detail")
        .expect("candidate-only binding planned");
    let retired = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("active-only binding planned");

    match fresh.outcome() {
        WorthUiQueryLiveRebindOutcome::Rebind(rebind) => assert_eq!(
            rebind.reason(),
            WorthUiQueryBindingRebindReason::FreshCandidateBinding
        ),
        other => panic!("candidate-only binding should fresh-rebind: {other:?}"),
    }
    match retired.outcome() {
        WorthUiQueryLiveRebindOutcome::Retire(retirement) => assert_eq!(
            retirement.reason(),
            WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding
        ),
        other => panic!("active-only binding should retire: {other:?}"),
    }
}

#[test]
fn stale_comparison_cannot_drive_live_rebind_planning() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);
    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("comparison succeeds");
    let stale_plan = WorthUiNodeReplacementPlan::new(
        plan.active_artifact_digest(),
        plan.candidate_artifact_digest() + 1,
        plan.classifications().to_vec(),
        plan.counters(),
    );

    let denial = runtime
        .plan_query_live_rebinds(&comparison, &stale_plan, &narrowing, &admitted)
        .expect_err("stale plan denies");

    match denial {
        WorthUiQueryLiveRebindPlanDenial::ComparisonDigestMismatch {
            comparison_candidate_artifact_digest,
            plan_candidate_artifact_digest,
            ..
        } => assert_ne!(
            comparison_candidate_artifact_digest,
            plan_candidate_artifact_digest
        ),
        other => panic!("unexpected denial: {other:?}"),
    }
}

fn query_live_rebind_plan(
    runtime: &crate::runtime::WorthUiRuntime,
    plan: &crate::runtime::WorthUiNodeReplacementPlan,
    narrowing: &crate::runtime::WorthUiRuntimeImpactNarrowing,
    admitted: &crate::runtime::WorthUiAdmittedReplacementCandidate,
) -> crate::runtime::WorthUiQueryLiveRebindPlan {
    let comparison = runtime
        .compare_query_bindings(plan, narrowing, admitted)
        .expect("comparison succeeds");
    runtime
        .plan_query_live_rebinds(&comparison, plan, narrowing, admitted)
        .expect("live rebind plan succeeds")
}
