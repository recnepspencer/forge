use crate::facade::*;
use crate::tests::support::*;

use super::source_corpus::{
    DOT_SOURCE, ENTRIES_SOURCE, EXECUTION_FLOW_SOURCE, FACADE_SOURCE, HARNESS_BRIDGE_SOURCE,
    HISTORY_SOURCE, OBSERVER_SOURCE, RECORDER_SOURCE, SUMMARY_SOURCE,
};

#[test]
fn gate6_broad_entry_access_is_visibility_restricted_and_boundary_reads_are_explicit() {
    assert!(
        ENTRIES_SOURCE.contains("pub(crate) fn get_entry(")
            && ENTRIES_SOURCE.contains("pub(crate) fn get_entry_mut("),
        "broad entry accessors should be crate-visible compatibility seams, not public API"
    );
    assert!(
        !ENTRIES_SOURCE.contains("pub fn get_entry(")
            && !ENTRIES_SOURCE.contains("pub fn get_entry_mut("),
        "broad entry accessors should no longer be exported publicly"
    );
    assert!(
        DOT_SOURCE.contains("node_condition(")
            && HARNESS_BRIDGE_SOURCE.contains("node_eval_config(")
            && EXECUTION_FLOW_SOURCE.contains("node_lineage_artifact_id(")
            && RECORDER_SOURCE.contains("stamp_runtime_artifact_lineage_and_execution(")
            && HISTORY_SOURCE.contains("node_execution_trace_stamp(")
            && SUMMARY_SOURCE.contains("node_runtime_artifact_state_present("),
        "boundary modules should move onto explicit graph accessors instead of relying on public broad entry assembly"
    );
    assert!(
        !FACADE_SOURCE.contains("NodeEntry,"),
        "public facade types should not re-export broad NodeEntry compatibility storage"
    );
    assert!(
        !FACADE_SOURCE.contains("RuntimeArtifactState,"),
        "public facade types should not re-export broad RuntimeArtifactState compatibility state"
    );
    assert!(
        !OBSERVER_SOURCE.contains("pub fn runtime_artifact_state("),
        "graph observer should not expose broad runtime artifact compatibility state on the public API"
    );
}

#[test]
fn proof_bearing_form_families_exist_as_real_types() {
    fn assert_canonical<T: CanonicalForm>() {}
    fn assert_resolved<T: ResolvedForm>() {}
    fn assert_delta<T: DeltaForm>() {}
    fn assert_summary<T: SummaryForm>() {}

    assert_canonical::<CanonicalDependencies>();
    assert_canonical::<CanonicalChangedRegions>();
    assert_canonical::<DedupedNodeBatch>();
    assert_canonical::<DependencyBatchEdit>();
    assert_canonical::<PartitionScopeSet>();
    assert_canonical::<SortedSourceBatch>();
    assert_resolved::<ResolvedExecutionStrategy>();
    assert_resolved::<ResolvedMaintenanceStrategy>();
    assert_resolved::<ResolvedPerformancePolicy>();
    assert_delta::<DirtyBatch>();
    assert_delta::<DirtyDelta>();
    assert_delta::<StructuralDelta>();
    assert_delta::<PatchPlan>();
    assert_summary::<LocalityFootprint>();
    assert_summary::<NarrowedPropagationSet>();
    assert_summary::<FrontierWave>();
    assert_summary::<InvalidationFrontier>();
    assert_summary::<InvalidationSeedBatch>();
    assert_summary::<FrontierPlan>();
    assert_summary::<FrontierExecutionSummary>();
    assert_summary::<SemanticBatchCommit>();
    assert_summary::<TouchedScopeSummary>();
    assert_summary::<PendingSnapshotBatch>();
    assert_summary::<SnapshotBatchCommit>();
    assert_summary::<SubscriberRepairBatch>();
}

#[test]
fn single_consumer_preserves_one_way_packet_flow() {
    let packet = SingleConsumer::new(vec![1_u32, 2, 3]);

    assert_eq!(packet.as_ref(), &[1, 2, 3]);
    assert_eq!(packet.into_inner(), vec![1, 2, 3]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderedTestItem(u32);

impl OrderedStreamItem for OrderedTestItem {
    type OrderKey = u32;

    fn order_key(&self) -> Self::OrderKey {
        self.0
    }
}

#[test]
fn mergeable_ordered_stream_merges_locally_ordered_shards_without_global_sort() {
    let left = LocallyOrderedShard::new(vec![OrderedTestItem(0), OrderedTestItem(2)]);
    let right = LocallyOrderedShard::new(vec![OrderedTestItem(1), OrderedTestItem(3)]);

    let merged = MergeableOrderedStream::new(vec![left, right])
        .try_into_vec()
        .unwrap();

    assert_eq!(
        merged,
        vec![
            OrderedTestItem(0),
            OrderedTestItem(1),
            OrderedTestItem(2),
            OrderedTestItem(3)
        ]
    );
}

#[test]
fn unordered_canonicalization_is_explicit_fallback_for_ordered_shards() {
    let shard = LocallyOrderedShard::canonicalize_unordered(vec![
        OrderedTestItem(3),
        OrderedTestItem(1),
        OrderedTestItem(2),
    ]);

    assert_eq!(
        shard.into_vec(),
        vec![OrderedTestItem(1), OrderedTestItem(2), OrderedTestItem(3)]
    );
}

#[test]
fn prepared_dependency_capture_recording_preserves_sorted_unique_order_without_resort() {
    let mut capture = crate::logic::prepared::PreparedDependencyCapture::new();
    let source_a = NodeId::new(9, 0);
    let source_b = NodeId::new(3, 1);

    capture.record(source_a, ASPECT_B, None);
    capture.record(source_b, ASPECT_A, None);
    capture.record(source_a, ASPECT_B, None);

    let capture = capture.into_sorted_unique();
    assert_eq!(capture.as_slice().len(), 2);
    assert!(capture.as_slice().windows(2).all(|pair| {
        (
            pair[0].source.index(),
            pair[0].source.generation(),
            pair[0].aspect.index(),
            pair[0].scope.as_ref(),
        ) < (
            pair[1].source.index(),
            pair[1].source.generation(),
            pair[1].aspect.index(),
            pair[1].scope.as_ref(),
        )
    }));
    assert_eq!(capture.as_slice()[0].source, source_b);
    assert_eq!(capture.as_slice()[1].source, source_a);
}
