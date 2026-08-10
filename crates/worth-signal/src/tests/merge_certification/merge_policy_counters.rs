use super::merge_certification_world::{
    build_aspect_policy_runtime, build_shared_state_conflict_runtime,
};
use crate::facade::AspectMergeDecisionOutcome;
use crate::logic::transaction::{merge_plan_proof_report, merge_result_proof_report};
use crate::tests::support::ASPECT_A;

#[test]
fn merge_execution_counters_obey_bounded_shared_conflict_contract() {
    let (mut runtime, feature, main) = build_shared_state_conflict_runtime();
    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .conflict_isolation_policy_named("signal.conflict-isolation.per-aspect")
        .run()
        .unwrap();

    assert!(
        result.counters.source_slice_breadth >= result.counters.final_candidate_breadth,
        "source slice breadth must dominate the final candidate set"
    );
    assert_eq!(
        result.counters.final_candidate_breadth,
        result.planned_candidates.nodes.len() as u64,
        "final candidate breadth should match the lowered candidate node set"
    );
    assert!(
        result.counters.reconciliation_breadth <= result.counters.final_candidate_breadth,
        "reconciliation breadth must stay within the admitted candidate set"
    );
    assert_eq!(
        result.counters.conflict_isolation_record_count,
        result.conflict_isolation_plan.records.len() as u64,
        "conflict isolation record count should mirror the lowered isolation plan"
    );
    assert_eq!(
        result.counters.conflict_isolation_expansion_breadth,
        result.conflict_isolation_plan.expansion_breadth,
        "conflict isolation breadth counters must mirror the lowered isolation plan"
    );
    assert_eq!(
        result.counters.conflict_isolation_expansion_breadth, 0,
        "current conflict isolation lowering must not widen candidate admission"
    );
    assert_eq!(
        result.counters.identity_ambiguous_match_count, 0,
        "shared-state conflict certification case should not introduce identity ambiguity"
    );
    assert_eq!(
        result.counters.identity_rejected_admissibility_count, 0,
        "shared-state conflict certification case should not rely on rejected identity admissibility"
    );
}

#[test]
fn aspect_policy_and_decision_lowering_remain_consistent() {
    let (mut runtime, feature, main) = build_aspect_policy_runtime();
    let planned = runtime.merge().from(feature).into(main).plan().unwrap();

    let aspect_policy_plan = planned.plan().aspect_policy_plan();
    let aspect_decision_plan = planned.plan().aspect_decision_plan();

    assert_eq!(aspect_policy_plan.records.len(), 1);
    assert_eq!(aspect_decision_plan.records.len(), 1);
    assert_eq!(aspect_policy_plan.records[0].aspect, ASPECT_A);
    assert_eq!(aspect_decision_plan.records[0].aspect, ASPECT_A);
    assert_eq!(
        aspect_policy_plan.records[0].selected_policy_name.as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_decision_plan.records[0]
            .selected_policy_name
            .as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_policy_plan.records[0].selected_policy_digest,
        aspect_decision_plan.records[0].selected_policy_digest
    );
    assert_eq!(
        aspect_policy_plan.records[0].selected_policy_basis,
        aspect_decision_plan.records[0].selected_policy_basis
    );
    assert_eq!(
        aspect_decision_plan.records[0].outcome,
        AspectMergeDecisionOutcome::SourceIntroducedIntoTarget
    );
}

#[test]
fn merge_base_selection_remains_consistent_from_plan_to_result_proof() {
    let (mut runtime, feature, main) = build_aspect_policy_runtime();
    let (
        plan_selected_merge_base_name,
        plan_selected_merge_base_digest,
        lowered_selected_merge_base_digest,
        plan_proof_selected_merge_base_digest,
    ) = {
        let planned = runtime
            .merge()
            .from(feature.clone())
            .into(main.clone())
            .merge_base_named("signal.merge-base.fork-point")
            .plan()
            .unwrap();
        let plan_proof =
            merge_plan_proof_report(planned.plan(), planned.plan().registry_bundle_digest());
        let lowered_merge_base = planned
            .plan()
            .lowered_merge_base()
            .expect("lowered merge-base plan");
        (
            planned.plan().selected_semantics().merge_base_name.clone(),
            planned
                .plan()
                .selected_semantics()
                .merge_base_digest
                .clone(),
            lowered_merge_base.selected_merge_base_digest.clone(),
            plan_proof.selected_merge_base_digest,
        )
    };

    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .merge_base_named("signal.merge-base.fork-point")
        .run()
        .unwrap();
    let result_proof = merge_result_proof_report(&result);

    assert_eq!(
        plan_selected_merge_base_name,
        result.selected_semantics.merge_base_name
    );
    assert_eq!(
        plan_selected_merge_base_digest,
        result.selected_semantics.merge_base_digest
    );
    assert_eq!(
        lowered_selected_merge_base_digest,
        plan_proof_selected_merge_base_digest
    );
    assert_eq!(
        result.selected_merge_base_digest,
        result_proof.selected_merge_base_digest
    );
    assert_eq!(
        plan_proof_selected_merge_base_digest,
        result_proof.selected_merge_base_digest
    );
}
