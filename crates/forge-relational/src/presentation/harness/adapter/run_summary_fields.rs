use crate::publication::data::{PublicationArtifactSnapshot, PublicationObservationSnapshot};
use crate::snapshots::data::SnapshotHandle;

use super::external_harness_summary_projection::{
    external_harness_summary_projection_bool, external_harness_summary_projection_object,
    external_harness_summary_projection_u64, external_harness_summary_projection_usize,
    optional_external_harness_summary_projection_string,
    optional_external_harness_summary_projection_u64,
    optional_external_harness_summary_projection_usize, ExternalHarnessSummaryProjection,
};

pub(super) fn run_summary(
    snapshot: &SnapshotHandle,
    entity_hits: usize,
    relation_hits: usize,
) -> ExternalHarnessSummaryProjection {
    RunSummary::from_snapshot_read(snapshot, entity_hits, relation_hits)
        .into_external_harness_summary_projection()
}

pub(super) fn publication_artifacts_extension(
    publication_artifacts: PublicationArtifactSnapshot,
) -> ExternalHarnessSummaryProjection {
    PublicationArtifactsExtension::from_snapshot(publication_artifacts)
        .into_external_harness_summary_projection()
}

pub(super) fn publication_diagnostic_observation_fields(
    observation: &PublicationObservationSnapshot,
) -> ExternalHarnessSummaryProjection {
    PublicationDiagnosticObservationSummary::from_observation(observation)
        .into_external_harness_summary_projection()
}

struct RunSummary {
    snapshot_id: u64,
    entity_hits: usize,
    relation_hits: usize,
}

impl RunSummary {
    fn from_snapshot_read(
        snapshot: &SnapshotHandle,
        entity_hits: usize,
        relation_hits: usize,
    ) -> Self {
        Self {
            snapshot_id: snapshot.snapshot_id.0,
            entity_hits,
            relation_hits,
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object([
            (
                "snapshot_id",
                external_harness_summary_projection_u64(self.snapshot_id),
            ),
            (
                "entity_hits",
                external_harness_summary_projection_usize(self.entity_hits),
            ),
            (
                "relation_hits",
                external_harness_summary_projection_usize(self.relation_hits),
            ),
        ])
    }
}

struct PublicationArtifactsExtension {
    observation: PublicationAuthorityObservationSummary,
    latest_patch_record_count: usize,
    latest_replay_present: bool,
}

impl PublicationArtifactsExtension {
    fn from_snapshot(publication_artifacts: PublicationArtifactSnapshot) -> Self {
        Self {
            observation: PublicationAuthorityObservationSummary::from_observation(
                &publication_artifacts.observation,
            ),
            latest_patch_record_count: publication_artifacts
                .latest_patch
                .as_ref()
                .map(|patch| patch.records.len())
                .unwrap_or_default(),
            latest_replay_present: publication_artifacts.latest_replay.is_some(),
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object([
            (
                "observation",
                self.observation.into_external_harness_summary_projection(),
            ),
            (
                "latest_patch_record_count",
                external_harness_summary_projection_usize(self.latest_patch_record_count),
            ),
            (
                "latest_replay_present",
                external_harness_summary_projection_bool(self.latest_replay_present),
            ),
        ])
    }
}

struct PublicationAuthorityObservationSummary {
    latest_commit_id: Option<u64>,
    publication_snapshot_id: Option<u64>,
    publication_status: Option<String>,
    latest_patch_position: Option<u64>,
    latest_patch_record_count: Option<usize>,
    latest_replay_commit_id: Option<u64>,
    latest_patch_present: bool,
    latest_replay_present: bool,
}

impl PublicationAuthorityObservationSummary {
    fn from_observation(observation: &PublicationObservationSnapshot) -> Self {
        Self {
            latest_commit_id: observation.latest_commit_id.map(|commit_id| commit_id.0),
            publication_snapshot_id: observation
                .publication_snapshot_id
                .map(|snapshot_id| snapshot_id.0),
            publication_status: observation
                .publication_status
                .as_ref()
                .map(|status| format!("{status:?}")),
            latest_patch_position: observation.latest_patch_position.map(|position| position.0),
            latest_patch_record_count: observation.latest_patch_record_count,
            latest_replay_commit_id: observation
                .latest_replay_commit_id
                .map(|commit_id| commit_id.0),
            latest_patch_present: observation.latest_patch_present,
            latest_replay_present: observation.latest_replay_present,
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object(self.into_projection_fields())
    }

    fn into_projection_fields(self) -> Vec<(&'static str, ExternalHarnessSummaryProjection)> {
        vec![
            (
                "latest_commit_id",
                optional_external_harness_summary_projection_u64(self.latest_commit_id),
            ),
            (
                "publication_snapshot_id",
                optional_external_harness_summary_projection_u64(self.publication_snapshot_id),
            ),
            (
                "publication_status",
                optional_external_harness_summary_projection_string(self.publication_status),
            ),
            (
                "latest_patch_position",
                optional_external_harness_summary_projection_u64(self.latest_patch_position),
            ),
            (
                "latest_patch_record_count",
                optional_external_harness_summary_projection_usize(self.latest_patch_record_count),
            ),
            (
                "latest_replay_commit_id",
                optional_external_harness_summary_projection_u64(self.latest_replay_commit_id),
            ),
            (
                "latest_patch_present",
                external_harness_summary_projection_bool(self.latest_patch_present),
            ),
            (
                "latest_replay_present",
                external_harness_summary_projection_bool(self.latest_replay_present),
            ),
        ]
    }
}

struct PublicationDiagnosticObservationSummary {
    authority: PublicationAuthorityObservationSummary,
    diagnostics_artifact_count: usize,
}

impl PublicationDiagnosticObservationSummary {
    fn from_observation(observation: &PublicationObservationSnapshot) -> Self {
        Self {
            authority: PublicationAuthorityObservationSummary::from_observation(observation),
            diagnostics_artifact_count: observation.diagnostics_artifact_count,
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        let mut fields = self.authority.into_projection_fields();
        fields.push((
            "diagnostics_artifact_count",
            external_harness_summary_projection_usize(self.diagnostics_artifact_count),
        ));
        external_harness_summary_projection_object(fields)
    }
}
