use crate::replay::data::RelationalReplayRecord;

use super::harness_summary_projection_value::{
    harness_summary_object, harness_summary_string, harness_summary_u64, harness_summary_usize,
    HarnessSummaryProjectionValue,
};

pub(super) fn replay_summary(replay: RelationalReplayRecord) -> HarnessSummaryProjectionValue {
    ReplaySummary::from_replay_record(replay).into_harness_summary_projection_value()
}

struct ReplaySummary {
    schema_version: u64,
    commit_id: u64,
    version_id: u64,
    snapshot_id: u64,
    patch_stream_position: u64,
    patch_record_count: usize,
    patch_ordering: String,
    patch_publication_mode: String,
}

impl ReplaySummary {
    fn from_replay_record(replay: RelationalReplayRecord) -> Self {
        Self {
            schema_version: replay.schema_version.0 as u64,
            commit_id: replay.commit_id.0,
            version_id: replay.version_id.0,
            snapshot_id: replay.snapshot_id.0,
            patch_stream_position: replay.patch.position.0,
            patch_record_count: replay.patch.records.len(),
            patch_ordering: format!("{:?}", replay.patch.ordering),
            patch_publication_mode: format!("{:?}", replay.patch.publication_mode),
        }
    }

    fn into_harness_summary_projection_value(self) -> HarnessSummaryProjectionValue {
        harness_summary_object([
            ("schema_version", harness_summary_u64(self.schema_version)),
            ("commit_id", harness_summary_u64(self.commit_id)),
            ("version_id", harness_summary_u64(self.version_id)),
            ("snapshot_id", harness_summary_u64(self.snapshot_id)),
            (
                "patch_stream_position",
                harness_summary_u64(self.patch_stream_position),
            ),
            (
                "patch_record_count",
                harness_summary_usize(self.patch_record_count),
            ),
            (
                "patch_ordering",
                harness_summary_string(self.patch_ordering),
            ),
            (
                "patch_publication_mode",
                harness_summary_string(self.patch_publication_mode),
            ),
        ])
    }
}
