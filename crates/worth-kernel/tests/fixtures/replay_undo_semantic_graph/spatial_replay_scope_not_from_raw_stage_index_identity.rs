use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::replay_undo_semantic_graph::lower_spatial_replay_scope_identity;
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

fn main() {}

fn rejects_raw_stage_index_identity(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    evidence_lookup_receipt: &EvidenceLookupExecutionReceipt,
    raw_stage_index_identity: &str,
) {
    let _ = lower_spatial_replay_scope_identity(
        spatial_touch_authority,
        evidence_lookup_receipt,
        raw_stage_index_identity,
    );
}
