use super::lane_meaning_parity_test_support::{
    plan_with_command_semantics_changed, query_preserving_lane_change_fixture,
};
use super::query_binding_comparison_test_support::{
    denial_presentation_drift_query_app, lifecycle_drift_query_app, phase11_pipeline,
    query_artifact, standard_query_app,
};
use crate::runtime::{
    WorthUiCrossLaneSemanticAuthority, WorthUiCrossLaneSemanticFamily,
    WorthUiLaneParityDenialReason, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters,
    WorthUiNodeReplacementPlan, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonCounters, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingPosture,
    WorthUiQueryLiveRebindOutcome, WorthUiQuerySupportStatus,
};

#[test]
fn same_artifact_meaning_preserved_across_admitted_lane_transition() {
    let fixture = query_preserving_lane_change_fixture();

    let report = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &fixture.query_comparison,
            Some(&fixture.query_rebind_plan),
        )
        .expect("lane meaning parity certifies");

    assert!(report.certifies_activation());
    assert_eq!(report.transitions().len(), 1);
    assert!(report.transitions()[0].mechanics_changed());
    assert_eq!(report.transitions()[0].active_lane(), None);
    assert_eq!(report.transitions()[0].candidate_lane(), None);
    assert!(report.transitions()[0]
        .meaning_parity()
        .iter()
        .all(|parity| parity.reference().preserves_meaning()));
    assert!(report.transitions()[0]
        .meaning_parity()
        .iter()
        .any(|parity| parity.reference().family()
            == WorthUiCrossLaneSemanticFamily::LaneChangeIdentity));
    assert_eq!(report.counters().semantic_mismatch_count(), 0);
    assert_eq!(report.counters().source_parse_count(), 0);
    assert_eq!(report.counters().registry_lookup_count(), 0);
    assert_eq!(report.counters().frame_execution_count(), 0);
}

#[test]
fn lane_report_does_not_guess_transition_lane_from_plan_partition_shape() {
    let fixture = query_preserving_lane_change_fixture();

    let report = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &fixture.query_comparison,
            Some(&fixture.query_rebind_plan),
        )
        .expect("lane meaning parity certifies");
    let transition = &report.transitions()[0];

    assert!(transition.mechanics_changed());
    assert_eq!(transition.active_lane(), None);
    assert_eq!(transition.candidate_lane(), None);
}

#[test]
fn query_rebind_receipt_certifies_drift_without_hiding_digest_difference() {
    let active_app = standard_query_app();
    let candidate_app = lifecycle_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let query_comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind_plan = runtime
        .plan_query_live_rebinds(&query_comparison, &plan, &narrowing, &admitted)
        .expect("query rebind plan succeeds");
    let fixture = query_preserving_lane_change_fixture();
    let lane_plan = all_lane_change_plan(&plan);

    let report = runtime
        .certify_lane_meaning_parity(
            &lane_plan,
            &narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &query_comparison,
            Some(&query_rebind_plan),
        )
        .expect("Query-owned rebind certifies lane parity");
    let query_reference = report.transitions()[0]
        .meaning_parity()
        .iter()
        .map(|parity| parity.reference())
        .find(|reference| {
            reference.family() == WorthUiCrossLaneSemanticFamily::QueryBindingMeaning
                && reference.identity() == "workspace.view_binding.selection"
        })
        .expect("query semantic reference is reported");

    assert_eq!(
        query_reference.authority(),
        WorthUiCrossLaneSemanticAuthority::QueryOwnedRebindReceipt
    );
    assert_ne!(
        query_reference.active_digest(),
        query_reference.candidate_digest()
    );
    assert!(query_reference.preserves_meaning());
}

#[test]
fn lane_specific_command_or_query_semantics_rejected() {
    let fixture = query_preserving_lane_change_fixture();
    let mut changed_query_comparison = fixture.query_comparison.clone();
    let entry = changed_query_comparison
        .entries()
        .first()
        .expect("fixture has query entry");
    let bogus_query = fixture
        .query_rebind_plan
        .binding_for_view_binding_id(entry.identity().view_binding_id())
        .expect("fixture has query rebind entry");

    assert!(matches!(
        bogus_query.outcome(),
        WorthUiQueryLiveRebindOutcome::Preserve(_)
    ));
    changed_query_comparison =
        comparison_with_deferred_candidate_support(&changed_query_comparison);

    let denial = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &changed_query_comparison,
            None,
        )
        .expect_err("lane-local query semantics cannot certify");

    assert_eq!(
        denial.reason(),
        WorthUiLaneParityDenialReason::QueryBindingChangedWithoutRebind
    );
}

#[test]
fn visual_similarity_without_semantic_parity_does_not_certify_lane_transition() {
    let fixture = query_preserving_lane_change_fixture();
    let candidate_plan = plan_with_command_semantics_changed(&fixture.active_plan);

    let denial = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &candidate_plan,
            &fixture.query_comparison,
            Some(&fixture.query_rebind_plan),
        )
        .expect_err("visual similarity cannot replace semantic parity");

    assert_eq!(
        denial.reason(),
        WorthUiLaneParityDenialReason::VisualSimilarityWithoutSemanticParity
    );
    assert_eq!(denial.counters().visual_only_evidence_rejected_count(), 1);
}

#[test]
fn lane_transition_with_changed_query_binding_denied_without_query_rebind() {
    let active_app = standard_query_app();
    let candidate_app = lifecycle_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let query_comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let fixture = query_preserving_lane_change_fixture();
    let lane_plan = all_lane_change_plan(&plan);

    let denial = runtime
        .certify_lane_meaning_parity(
            &lane_plan,
            &narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &query_comparison,
            None,
        )
        .expect_err("changed query binding requires Query-owned rebind");

    assert_eq!(
        denial.reason(),
        WorthUiLaneParityDenialReason::QueryBindingChangedWithoutRebind
    );
}

#[test]
fn wrong_query_rebind_outcome_cannot_certify_changed_binding() {
    let fixture = query_preserving_lane_change_fixture();
    let changed_query_comparison =
        comparison_with_deferred_candidate_support(&fixture.query_comparison);

    let denial = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &changed_query_comparison,
            Some(&fixture.query_rebind_plan),
        )
        .expect_err("preserve receipt cannot satisfy rebind-required drift");

    assert_eq!(
        denial.reason(),
        WorthUiLaneParityDenialReason::QueryRebindOutcomeMismatch
    );
}

#[test]
fn denied_query_rebind_receipt_cannot_certify_lane_parity() {
    let active_app = standard_query_app();
    let candidate_app = denial_presentation_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);
    let query_comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind_plan = runtime
        .plan_query_live_rebinds(&query_comparison, &plan, &narrowing, &admitted)
        .expect("query rebind plan succeeds");
    let fixture = query_preserving_lane_change_fixture();
    let lane_plan = all_lane_change_plan(&plan);

    let denial = runtime
        .certify_lane_meaning_parity(
            &lane_plan,
            &narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &query_comparison,
            Some(&query_rebind_plan),
        )
        .expect_err("Query denial receipt cannot certify lane parity");

    assert_eq!(
        denial.reason(),
        WorthUiLaneParityDenialReason::QueryRebindDenied
    );
}

fn comparison_with_deferred_candidate_support(
    comparison: &WorthUiQueryBindingComparison,
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
            let candidate_posture = entry.candidate_posture().map(posture_with_deferred_support);
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

fn posture_with_deferred_support(
    posture: &WorthUiQueryBindingPosture,
) -> WorthUiQueryBindingPosture {
    posture.with_query_support_status_for_test(WorthUiQuerySupportStatus::Deferred)
}

fn all_lane_change_plan(plan: &WorthUiNodeReplacementPlan) -> WorthUiNodeReplacementPlan {
    let mut counters = WorthUiNodeReplacementCounters::default();
    let classifications = plan
        .classifications()
        .iter()
        .map(|classification| {
            counters.record_transition(WorthUiNodeLifecycleTransition::LaneChange);
            WorthUiNodeReplacementClassification::new(
                crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassificationInput {
                    identity_basis: classification.identity_basis().to_owned(),
                    authored_provenance_digest: classification.authored_provenance_digest(),
                    transition: WorthUiNodeLifecycleTransition::LaneChange,
                    active_kind: classification.active_kind(),
                    candidate_kind: classification.candidate_kind(),
                    active_durable_state_eligible: classification.active_durable_state_eligible(),
                    candidate_durable_state_eligible: classification.candidate_durable_state_eligible(),
                    active_resize_contract_id: classification.active_resize_contract_id().cloned(),
                    candidate_resize_contract_id: classification.candidate_resize_contract_id().cloned(),
                    active_resize_permission: classification.active_resize_permission().cloned(),
                    candidate_resize_permission: classification.candidate_resize_permission().cloned(),
                    active_resize_shape_digest: classification.active_resize_shape_digest(),
                    candidate_resize_shape_digest: classification.candidate_resize_shape_digest(),
                },
            )
        })
        .collect();
    WorthUiNodeReplacementPlan::new(
        plan.active_artifact_digest(),
        plan.candidate_artifact_digest(),
        classifications,
        counters,
    )
}
