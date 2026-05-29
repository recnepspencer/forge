use serde_json::{Map, Value};

use crate::publication::data::{PublicationArtifactSnapshot, PublicationObservationSnapshot};
use crate::snapshots::data::SnapshotHandle;

pub(super) fn run_summary(
    snapshot: &SnapshotHandle,
    entity_hits: usize,
    relation_hits: usize,
) -> Value {
    Value::Object(Map::from_iter([
        (
            "snapshot_id".to_string(),
            Value::from(snapshot.snapshot_id.0),
        ),
        ("entity_hits".to_string(), Value::from(entity_hits as u64)),
        (
            "relation_hits".to_string(),
            Value::from(relation_hits as u64),
        ),
    ]))
}

pub(super) fn publication_artifacts_extension(
    publication_artifacts: PublicationArtifactSnapshot,
) -> Value {
    Value::Object(Map::from_iter([
        (
            "observation".to_string(),
            publication_observation_fields(&publication_artifacts.observation),
        ),
        (
            "latest_patch_record_count".to_string(),
            Value::from(
                publication_artifacts
                    .latest_patch
                    .as_ref()
                    .map(|patch| patch.records.len())
                    .unwrap_or_default() as u64,
            ),
        ),
        (
            "latest_replay_present".to_string(),
            Value::Bool(publication_artifacts.latest_replay.is_some()),
        ),
    ]))
}

pub(super) fn publication_observation_fields(
    observation: &PublicationObservationSnapshot,
) -> Value {
    Value::Object(Map::from_iter([
        (
            "latest_commit_id".to_string(),
            observation
                .latest_commit_id
                .map(|commit_id| Value::from(commit_id.0))
                .unwrap_or(Value::Null),
        ),
        (
            "publication_snapshot_id".to_string(),
            observation
                .publication_snapshot_id
                .map(|snapshot_id| Value::from(snapshot_id.0))
                .unwrap_or(Value::Null),
        ),
        (
            "publication_status".to_string(),
            observation
                .publication_status
                .as_ref()
                .map(|status| Value::String(format!("{status:?}")))
                .unwrap_or(Value::Null),
        ),
        (
            "latest_patch_position".to_string(),
            observation
                .latest_patch_position
                .map(|position| Value::from(position.0))
                .unwrap_or(Value::Null),
        ),
        (
            "latest_patch_record_count".to_string(),
            optional_usize(observation.latest_patch_record_count),
        ),
        (
            "latest_replay_commit_id".to_string(),
            observation
                .latest_replay_commit_id
                .map(|commit_id| Value::from(commit_id.0))
                .unwrap_or(Value::Null),
        ),
        (
            "latest_patch_present".to_string(),
            Value::Bool(observation.latest_patch_present),
        ),
        (
            "latest_replay_present".to_string(),
            Value::Bool(observation.latest_replay_present),
        ),
        (
            "diagnostics_artifact_count".to_string(),
            Value::from(observation.diagnostics_artifact_count as u64),
        ),
    ]))
}

fn optional_usize(value: Option<usize>) -> Value {
    value
        .map(|count| Value::from(count as u64))
        .unwrap_or(Value::Null)
}
