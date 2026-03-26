use crate::facade::history::BranchId;
use crate::facade::merge::MergeIntent;
use crate::tests::support::{
    create_branch_from_main, create_entity, persisted_runtime_with_test_schema, update_entity,
    update_entity_on_branch,
};

#[test]
fn complexity_budget_merge_planning_reports_request_shaped_work() {
    let mut runtime = persisted_runtime_with_test_schema();
    let shared = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, shared, "main-value");
    update_entity_on_branch(
        &mut runtime,
        shared,
        "feature-value",
        BranchId("feature".to_string()),
    );

    runtime.performance_access().reset_counters();
    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
            BranchId("main".to_string()),
            BranchId("feature".to_string()),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("merge planning artifact");

    let counters = runtime.performance_access().counters();
    assert!(runtime
        .performance_access()
        .contracts()
        .iter()
        .any(|contract| contract.id == "runtime.merge.planning"));
    assert_eq!(counters.merge_planning_requests, 1);
    assert!(counters.merge_planning_schema_kinds_snapshotted >= 1);
    assert!(counters.merge_planning_target_commits_scoped >= 1);
    assert!(counters.merge_planning_source_commits_scoped >= 1);
    assert!(counters.merge_planning_target_records_scoped >= 1);
    assert!(counters.merge_planning_source_records_scoped >= 1);
    assert_eq!(
        counters.merge_identity_candidates_discovered,
        artifact.identity_discovery.candidate_count
    );
    assert!(counters.merge_identity_target_records_scanned >= 1);
    assert!(counters.merge_identity_target_records_indexed >= 1);
    assert_eq!(
        counters.merge_conflict_records_classified,
        artifact.conflict_classification.classified_record_count
    );
    assert_eq!(
        counters.merge_causal_records_annotated,
        artifact.causal_annotation.classified_record_count
    );
    assert_eq!(
        counters.merge_policy_records_resolved,
        artifact.policy_resolution.resolved_record_count
    );
    assert_eq!(
        counters.merge_lowered_records_emitted,
        artifact.lowered_plan.record_count
    );
    assert_eq!(
        counters.merge_decision_log_width,
        artifact.decision_log.decisions.len()
    );
    assert!(counters.merge_planning_elapsed_nanos > 0);
}
