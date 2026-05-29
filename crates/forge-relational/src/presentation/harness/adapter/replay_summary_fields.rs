use serde_json::{Map, Value};

use crate::replay::data::RelationalReplayRecord;

pub(super) fn replay_summary(replay: RelationalReplayRecord) -> Value {
    Value::Object(Map::from_iter([
        (
            "schema_version".to_string(),
            Value::from(replay.schema_version.0 as u64),
        ),
        ("commit_id".to_string(), Value::from(replay.commit_id.0)),
        ("version_id".to_string(), Value::from(replay.version_id.0)),
        ("snapshot_id".to_string(), Value::from(replay.snapshot_id.0)),
        (
            "patch_stream_position".to_string(),
            Value::from(replay.patch.position.0),
        ),
        (
            "patch_record_count".to_string(),
            Value::from(replay.patch.records.len() as u64),
        ),
        (
            "patch_ordering".to_string(),
            Value::String(format!("{:?}", replay.patch.ordering)),
        ),
        (
            "patch_publication_mode".to_string(),
            Value::String(format!("{:?}", replay.patch.publication_mode)),
        ),
    ]))
}
