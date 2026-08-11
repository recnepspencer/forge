use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0NondeterministicMetadata, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::physical_status::MilestonePhysicalStatusRow;
use super::sequence_status::RoadmapSequenceStatusMatrix;
use super::validation::{
    reject_duplicate_milestone_rows, reject_missing_required_milestone_rows,
    reject_unknown_matrix_rows, require_non_empty, stable_digest, S0MilestoneMatrixBuildRejection,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct MilestonePhysicalStatusMatrix {
    #[serde(flatten)]
    envelope: S0ArtifactEnvelopeMetadata,
    roadmap_sequence_status: RoadmapSequenceStatusMatrix,
    rows: Vec<MilestonePhysicalStatusRow>,
}

impl MilestonePhysicalStatusMatrix {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        roadmap_sequence_status: RoadmapSequenceStatusMatrix,
        required_milestone_ids: Vec<String>,
        mut rows: Vec<MilestonePhysicalStatusRow>,
    ) -> Result<Self, S0MilestoneMatrixBuildRejection> {
        let source_revision = require_non_empty(source_revision)
            .map_err(|_| S0MilestoneMatrixBuildRejection::EmptyRequiredField)?;
        let generated_by = require_non_empty(generated_by)
            .map_err(|_| S0MilestoneMatrixBuildRejection::EmptyRequiredField)?;
        if rows.is_empty() {
            return Err(S0MilestoneMatrixBuildRejection::MissingMilestoneRow);
        }
        rows.sort_by(|left, right| left.milestone_id().cmp(right.milestone_id()));
        reject_duplicate_milestone_rows(&rows)?;
        reject_unknown_matrix_rows(&roadmap_sequence_status, &rows)?;
        reject_missing_required_milestone_rows(&required_milestone_ids, &rows)?;
        let deterministic_digest = stable_digest(&MilestonePhysicalStatusMatrixDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: S0ArtifactKind::MilestonePhysicalStatusMatrix,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            roadmap_sequence_status: &roadmap_sequence_status,
            rows: &rows,
        })
        .map_err(|_| S0MilestoneMatrixBuildRejection::InvalidDigest)?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            roadmap_sequence_status,
            rows,
        })
    }

    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn roadmap_sequence_status(&self) -> &RoadmapSequenceStatusMatrix {
        &self.roadmap_sequence_status
    }

    pub fn rows(&self) -> &[MilestonePhysicalStatusRow] {
        &self.rows
    }
}

#[derive(Serialize)]
struct MilestonePhysicalStatusMatrixDigestBasis<'a> {
    schema_version: &'static str,
    artifact_kind: S0ArtifactKind,
    source_revision: &'a str,
    roadmap_parent_digest: &'a S0StableDigest,
    generated_by: &'a str,
    roadmap_sequence_status: &'a RoadmapSequenceStatusMatrix,
    rows: &'a [MilestonePhysicalStatusRow],
}
