use super::source_corpus::{
    MERGE_PLAN_SOURCE, MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE, MERGE_RUNTIME_NODE_PLAN_SOURCE,
    MERGE_RUNTIME_PLAN_SOURCE, STATE_SOURCE,
};

#[test]
fn snapshot_restore_plan_separates_restore_proof_from_delta_accounting() {
    assert!(
        STATE_SOURCE.contains("pub struct CheckpointRestoreSnapshotBatch")
            && STATE_SOURCE.contains("classified: ClassifiedSnapshotBatchCommit"),
        "restore plan should name the classified checkpoint rebuild proof explicitly"
    );
    assert!(
        STATE_SOURCE.contains("pub struct RestoreDeltaAccounting")
            && STATE_SOURCE.contains("dependency_snapshot_delta_node_count: u64"),
        "restore plan should name delta accounting separately from the rebuild proof"
    );
    assert!(
        STATE_SOURCE.contains("checkpoint_restore_batch: CheckpointRestoreSnapshotBatch")
            && STATE_SOURCE.contains("delta_accounting: RestoreDeltaAccounting"),
        "snapshot restore plan should carry distinct proof and accounting fields"
    );
    assert!(
        STATE_SOURCE.contains("intent: SnapshotRestoreIntent")
            && STATE_SOURCE.contains("shared_node_count: u64")
            && STATE_SOURCE.contains("current_only_node_count: u64")
            && STATE_SOURCE.contains("snapshot_only_node_count: u64")
            && STATE_SOURCE.contains("coarse_replacement_required: bool")
            && STATE_SOURCE.contains("coarse_reasons: Vec<SnapshotRestoreCoarseReason>"),
        "snapshot restore plan should keep its restore-structure fields private"
    );
    assert!(
        STATE_SOURCE.contains("pub fn checkpoint_restore_batch(&self) -> &CheckpointRestoreSnapshotBatch")
            && STATE_SOURCE.contains("pub fn dependency_snapshot_delta_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn shared_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn current_only_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn snapshot_only_node_count(&self) -> u64")
            && STATE_SOURCE.contains("pub fn coarse_replacement_required(&self) -> bool")
            && STATE_SOURCE.contains("pub fn coarse_reasons(&self) -> &[SnapshotRestoreCoarseReason]"),
        "snapshot restore plan should expose restore proof and accounting only through explicit accessors"
    );
}

#[test]
fn merge_runtime_uses_sealed_projection_accessors_instead_of_rederiving_lane_state() {
    assert!(
        MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE.contains("struct NodeMergeProjection"),
        "merge runtime should define a single projection for merge-comparable state"
    );
    assert!(
        MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE.contains("fn node_merge_projection("),
        "merge runtime should centralize merge projection assembly behind one accessor"
    );
    assert!(
        !MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE.contains("node_merge_comparable(")
            && !MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE.contains("node_lineage_artifact_id(")
            && !MERGE_RUNTIME_ARTIFACT_PROJECTION_SOURCE.contains("node_merge_authority("),
        "merge runtime should not fall back to separate comparable, lineage, and authority helpers"
    );
}

#[test]
fn merge_planning_packets_are_mediated_through_constructors_and_accessors() {
    assert!(
        MERGE_PLAN_SOURCE.contains("impl NodeMergeInputState")
            && MERGE_PLAN_SOURCE.contains("impl NodeMergePlan")
            && MERGE_PLAN_SOURCE.contains("impl LoweredMergePlan"),
        "merge planning packet families should be mediated through implementation boundaries"
    );
    assert!(
        !MERGE_RUNTIME_NODE_PLAN_SOURCE.contains("NodeMergePlan {")
            && !MERGE_RUNTIME_NODE_PLAN_SOURCE.contains("NodeMergeInputState {")
            && !MERGE_RUNTIME_NODE_PLAN_SOURCE.contains("LoweredMergePlan {")
            && !MERGE_RUNTIME_PLAN_SOURCE.contains("NodeMergePlan {")
            && !MERGE_RUNTIME_PLAN_SOURCE.contains("NodeMergeInputState {")
            && !MERGE_RUNTIME_PLAN_SOURCE.contains("LoweredMergePlan {"),
        "merge runtime should not assemble merge planning packets by open struct literal"
    );
    assert!(
        MERGE_RUNTIME_NODE_PLAN_SOURCE.contains("NodeMergePlan::new(")
            && MERGE_RUNTIME_NODE_PLAN_SOURCE.contains("NodeMergeInputState::new(")
            && MERGE_RUNTIME_PLAN_SOURCE.contains("LoweredMergePlan::new("),
        "merge runtime should construct merge planning packets through their constructors"
    );
}
