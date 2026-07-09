use crate::diagnostics::data::{
    DiagnosticCode, RelationalDiagnosticFields, RelationalDiagnosticValue,
    RelationalDiagnosticsEntry,
};
use crate::durability::data::{
    DurableCheckpoint, DurableCheckpointId, DurableCheckpointManifest, DurableSegmentId,
};
use crate::history::data::CommitId;

pub(in crate::durability::authority) fn persisted_checkpoint_created(
    manifest: &DurableCheckpointManifest,
    checkpoint: &DurableCheckpoint,
) -> RelationalDiagnosticsEntry {
    checkpoint_created(checkpoint_created_fields(
        Some(manifest.checkpoint_id),
        checkpoint
            .coverage
            .up_to_commit
            .as_ref()
            .map(|commit| commit.commit_id),
        manifest.partition_count,
    ))
}

pub(in crate::durability::authority) fn in_memory_checkpoint_created(
    checkpoint: &DurableCheckpoint,
) -> RelationalDiagnosticsEntry {
    checkpoint_created(checkpoint_created_fields(
        None,
        checkpoint
            .coverage
            .up_to_commit
            .as_ref()
            .map(|commit| commit.commit_id),
        checkpoint.partition_images.len(),
    ))
}

pub(in crate::durability::authority) fn durable_store_compacted(
    checkpoint_id: DurableCheckpointId,
    removed_segments: &[DurableSegmentId],
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(
        DiagnosticCode::DurableCompactionCompleted,
        "durable store compacted",
        durable_store_compacted_fields(checkpoint_id, removed_segments),
    )
}

pub(in crate::durability::authority) fn durable_segment_append_succeeded(
    segment_id: DurableSegmentId,
    commit_id: CommitId,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(
        DiagnosticCode::DurableAppendSucceeded,
        "durable segment append succeeded",
        durable_segment_append_succeeded_fields(segment_id, commit_id),
    )
}

fn checkpoint_created(fields: RelationalDiagnosticFields) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry::new(
        DiagnosticCode::CheckpointCreated,
        "durable checkpoint created",
        fields,
    )
}

fn checkpoint_created_fields(
    checkpoint_id: Option<DurableCheckpointId>,
    up_to_commit: Option<CommitId>,
    partition_count: usize,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        ("checkpoint_id", optional_checkpoint_id_value(checkpoint_id)),
        ("up_to_commit", optional_commit_id_value(up_to_commit)),
        (
            "partition_count",
            RelationalDiagnosticValue::unsigned(partition_count),
        ),
    ])
    .into()
}

fn durable_store_compacted_fields(
    checkpoint_id: DurableCheckpointId,
    removed_segments: &[DurableSegmentId],
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "checkpoint_id",
            RelationalDiagnosticValue::DurableCheckpointId(checkpoint_id),
        ),
        ("removed_segments", segment_id_array(removed_segments)),
    ])
    .into()
}

fn durable_segment_append_succeeded_fields(
    segment_id: DurableSegmentId,
    commit_id: CommitId,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "segment_id",
            RelationalDiagnosticValue::DurableSegmentId(segment_id),
        ),
        ("commit_id", RelationalDiagnosticValue::CommitId(commit_id)),
    ])
    .into()
}

fn optional_checkpoint_id_value(
    checkpoint_id: Option<DurableCheckpointId>,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::optional(
        checkpoint_id.map(RelationalDiagnosticValue::DurableCheckpointId),
    )
}

fn optional_commit_id_value(commit_id: Option<CommitId>) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::optional(commit_id.map(RelationalDiagnosticValue::CommitId))
}

fn segment_id_array(segment_ids: &[DurableSegmentId]) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        segment_ids
            .iter()
            .copied()
            .map(RelationalDiagnosticValue::DurableSegmentId),
    )
}
