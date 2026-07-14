use super::query_binding::{
    WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonCounters,
    WorthUiQueryBindingComparisonEntry, WorthUiQueryBindingComparisonOutcome,
    WorthUiQueryBindingPosture,
};
use super::query_binding_comparison_test_support::{
    basis_drift_query_app, denial_presentation_drift_query_app, phase11_pipeline, query_artifact,
    standard_query_app,
};
use crate::runtime::{
    WorthUiNodeReplacementPlan, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirementReason, WorthUiQueryLiveRebindOutcome,
    WorthUiQueryLiveRebindPlanDenial, WorthUiQueryRebindRequiredSurface, WorthUiQuerySupportStatus,
};

#[test]
fn same_query_binding_basis_preserves_live_binding() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);

    let rebind_plan = query_live_rebind_plan(&runtime, &plan, &narrowing, &admitted);
    let entry = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding planned");

    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Preserve(preservation) => {
            assert_eq!(
                preservation.identity().view_binding_id(),
                "workspace.view_binding.selection"
            );
            assert!(preservation
                .preservation_receipt()
                .contains("query-live-preserve"));
        }
        other => panic!("same basis should preserve live binding, got {other:?}"),
    }
    assert_eq!(rebind_plan.counters().preserved_binding_count(), 1);
    assert_eq!(rebind_plan.counters().rebound_binding_count(), 0);
    assert_eq!(rebind_plan.counters().denied_binding_count(), 0);
}

#[test]
fn query_basis_drift_requires_rebind_or_denial() {
    let active_app = standard_query_app();
    let candidate_app = basis_drift_query_app();
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
                WorthUiQueryBindingRebindReason::QueryOwnedPostureDrift
            );
            assert!(rebind
                .required_query_surfaces()
                .contains(&WorthUiQueryRebindRequiredSurface::BasisCapabilityLifecycle));
        }
        WorthUiQueryLiveRebindOutcome::Deny(denial) => {
            assert!(!denial.drift_families().is_empty());
        }
        other => panic!("basis drift cannot preserve stale live binding: {other:?}"),
    }
    assert_eq!(rebind_plan.counters().preserved_binding_count(), 0);
}

#[test]
fn ui_local_subscription_recovery_path_rejected() {
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
        WorthUiQueryLiveRebindOutcome::Deny(denial) => {
            assert_eq!(
                denial.reason(),
                WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery
            );
            assert!(denial.active_posture().is_some());
            assert!(denial.candidate_posture().is_some());
        }
        other => panic!("UI-local recovery substitution should deny: {other:?}"),
    }
    assert_eq!(rebind_plan.counters().denied_binding_count(), 1);
}

#[test]
fn stale_query_subscription_handle_cannot_be_preserved_after_basis_drift() {
    let active_app = standard_query_app();
    let candidate_app = basis_drift_query_app();
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
    assert!(!matches!(
        entry.outcome(),
        WorthUiQueryLiveRebindOutcome::Preserve(_)
    ));
}

#[test]
fn deferred_query_support_posture_denies_live_rebind_activation() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);
    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("comparison succeeds");
    let deferred_comparison =
        comparison_with_candidate_support_status(&comparison, WorthUiQuerySupportStatus::Deferred);

    let rebind_plan = runtime
        .plan_query_live_rebinds(&deferred_comparison, &plan, &narrowing, &admitted)
        .expect("live rebind planning succeeds");
    let entry = rebind_plan
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding planned");

    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Deny(denial) => {
            assert_eq!(
                denial.reason(),
                WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted
            );
            assert_eq!(
                denial
                    .candidate_posture()
                    .expect("candidate posture")
                    .query_support_status(),
                WorthUiQuerySupportStatus::Deferred
            );
            assert_eq!(
                denial.drift_families(),
                &[WorthUiQueryBindingPostureDriftFamily::SupportAdmission]
            );
        }
        other => panic!("deferred Query support cannot produce activation rebind: {other:?}"),
    }
    assert_eq!(rebind_plan.counters().denied_binding_count(), 1);
    assert_eq!(rebind_plan.counters().rebound_binding_count(), 0);
}

fn comparison_with_candidate_support_status(
    comparison: &WorthUiQueryBindingComparison,
    support_status: WorthUiQuerySupportStatus,
) -> WorthUiQueryBindingComparison {
    let original_counters = comparison.counters();
    let mut counters = WorthUiQueryBindingComparisonCounters::default();
    counters.record_active_bindings_indexed(original_counters.active_bindings_indexed());
    counters.record_candidate_bindings_indexed(original_counters.candidate_bindings_indexed());
    counters
        .record_affected_query_invalidations(original_counters.affected_query_invalidation_count());
    let entries = comparison
        .entries()
        .iter()
        .map(|entry| {
            let candidate_posture = entry
                .candidate_posture()
                .map(|posture| posture_with_support_status(posture, support_status));
            let posture_drifts = match (entry.active_posture(), candidate_posture.as_ref()) {
                (Some(active), Some(candidate)) => active.drift_families_against(candidate),
                _ => entry.posture_drifts().to_vec(),
            };
            let outcome = if posture_drifts.is_empty() {
                entry.outcome()
            } else {
                WorthUiQueryBindingComparisonOutcome::RebindRequired
            };
            counters.record_entry(outcome, posture_drifts.len());
            WorthUiQueryBindingComparisonEntry::new(
                entry.identity().clone(),
                entry.active_posture().cloned(),
                candidate_posture,
                outcome,
                posture_drifts,
            )
        })
        .collect();
    WorthUiQueryBindingComparison::new(
        comparison.active_artifact_digest(),
        comparison.candidate_artifact_digest(),
        entries,
        counters,
    )
}

fn posture_with_support_status(
    posture: &WorthUiQueryBindingPosture,
    support_status: WorthUiQuerySupportStatus,
) -> WorthUiQueryBindingPosture {
    WorthUiQueryBindingPosture::new(
        support_status,
        posture.support_admission_digest().to_owned(),
        posture.basis_capability_digest().to_owned(),
        posture.live_compatibility_digest().to_owned(),
        posture.async_result_state_digest().to_owned(),
        posture.recovery_digest().to_owned(),
        posture.inspection_digest().to_owned(),
        posture.projection_consumption_digest().to_owned(),
        posture.denial_presentation_digest().to_owned(),
    )
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

#[test]
fn changed_admitted_query_support_receipt_cannot_drive_live_rebind_planning() {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&app, active, candidate);
    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("comparison succeeds");
    let stale_admitted = admitted.with_admitted_query_support_receipt_digest_for_test(u64::MAX);

    let denial = runtime
        .plan_query_live_rebinds(&comparison, &plan, &narrowing, &stale_admitted)
        .expect_err("changed admitted receipt denies");

    match denial {
        WorthUiQueryLiveRebindPlanDenial::AdmittedQuerySupportReceiptChanged {
            admitted_receipt_digest,
            current_receipt_digest,
        } => assert_ne!(admitted_receipt_digest, current_receipt_digest),
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
