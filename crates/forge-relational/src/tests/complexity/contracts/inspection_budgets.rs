use crate::facade::inspection::{
    ConnectivityInspectionRequest, GraphInspectionRequest, InspectionRecordClass,
    InspectionScope, KindInspectionRequest, RecentCommitInspectionRequest,
    StructuralIdentityQueryRequest,
};
use crate::identity::data::KindId;
use crate::symbols::data::Symbol;
use crate::tests::support::*;

#[test]
fn complexity_budget_graph_summary_reports_explicit_inspection_work() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let _relation = create_relation(&mut runtime, left, right, "rel");

    runtime.performance_access().reset_counters();
    let summary = runtime.inspection_access().graph_summary(&GraphInspectionRequest {
        scope: InspectionScope::Current,
        partition_scope: None,
        relation_kind_scope: None,
        summary_only: true,
    });
    let counters = runtime.performance_access().counters();

    assert_eq!(summary.entity_count, 2);
    assert_eq!(summary.relation_count, 1);
    assert_eq!(counters.inspection_graph_summary_requests, 1);
    assert!(counters.visible_entity_records_materialized >= 2);
    assert!(counters.visible_relation_records_materialized >= 1);
}

#[test]
fn complexity_budget_structural_identity_distinguishes_direct_lookup_from_broad_query() {
    let mut runtime = runtime_with_test_schema();
    let entity = create_entity(&mut runtime, "alpha");
    let _other = create_entity(&mut runtime, "beta");

    runtime.performance_access().reset_counters();
    let direct = runtime.inspection_access().structural_identity(
        InspectionScope::Current,
        crate::facade::transactions::RecordRef::Entity(entity),
    );
    let direct_counters = runtime.performance_access().counters();

    assert!(direct.is_some());
    assert_eq!(direct_counters.inspection_structural_identity_lookups, 1);
    assert_eq!(direct_counters.inspection_structural_identity_query_scans, 0);
    assert_eq!(direct_counters.visibility_entity_slot_scans, 0);

    runtime.performance_access().reset_counters();
    let broad = runtime
        .inspection_access()
        .query_structural_identity(&StructuralIdentityQueryRequest {
            scope: InspectionScope::Current,
            partition_scope: None,
            fingerprint_family: Symbol(1),
        });
    let broad_counters = runtime.performance_access().counters();

    assert!(broad.is_empty());
    assert_eq!(broad_counters.inspection_structural_identity_query_scans, 1);
    assert!(broad_counters.inspection_structural_identity_lookups >= 2);
    assert!(broad_counters.visible_entity_records_materialized >= 2);
}

#[test]
fn complexity_budget_kind_summary_reports_request_shaped_scope() {
    let mut runtime = runtime_with_test_schema();
    let _left_a = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _left_b = create_entity_in_partition(&mut runtime, "left-b", PartitionId(7));
    let _right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));

    runtime.performance_access().reset_counters();
    let summary = runtime.inspection_access().kind_summary(&KindInspectionRequest {
        scope: InspectionScope::Current,
        partition_scope: Some(vec![PartitionId(7)]),
        kind_id: KindId(1),
        record_class: InspectionRecordClass::Entity,
    });
    let counters = runtime.performance_access().counters();

    assert_eq!(summary.count, 2);
    assert_eq!(summary.touched_partitions, vec![PartitionId(7)]);
    assert_eq!(counters.inspection_kind_summary_requests, 1);
    assert!(counters.visible_entity_records_materialized >= 3);
}

#[test]
fn complexity_budget_connectivity_summary_reports_broad_traversal_work_explicitly() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let isolated = create_entity(&mut runtime, "isolated");
    let _relation = create_relation(&mut runtime, left, right, "rel");

    runtime.performance_access().reset_counters();
    let summary = runtime
        .inspection_access()
        .connectivity_summary(&ConnectivityInspectionRequest {
            scope: InspectionScope::Current,
            partition_scope: None,
            relation_kind_scope: None,
            include_members: false,
        });
    let counters = runtime.performance_access().counters();

    assert_eq!(summary.component_count, 2);
    assert_eq!(summary.largest_component_size, 2);
    assert_eq!(summary.enumerated_entity_count, 3);
    assert_eq!(counters.inspection_connectivity_summary_requests, 1);
    assert!(counters.visible_entity_records_materialized >= 3);
    assert!(counters.visible_relation_records_materialized >= 1);
    assert!(summary
        .components
        .iter()
        .all(|component| component.members.is_none()));
    assert_eq!(isolated.partition_id, PartitionId::main());
}

#[test]
fn complexity_budget_commit_inspection_reads_are_index_explicit_and_bounded() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity(&mut runtime, "first");
    let _second = create_entity(&mut runtime, "second");
    let latest_commit = runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id)
        .expect("latest commit");

    runtime.performance_access().reset_counters();
    let commit = runtime
        .inspection_access()
        .inspect_commit(latest_commit)
        .expect("commit inspection");
    let recent = runtime
        .inspection_access()
        .inspect_recent_commits(&RecentCommitInspectionRequest {
            branch_id: Some(crate::facade::history::BranchId("main".to_string())),
            limit: 2,
        });
    let counters = runtime.performance_access().counters();

    assert_eq!(commit.commit.commit_id, latest_commit);
    assert_eq!(recent.commits.len(), 2);
    assert_eq!(counters.inspection_commit_reads, 3);
    assert_eq!(counters.visible_entity_records_materialized, 0);
    assert_eq!(counters.visible_relation_records_materialized, 0);
}
