use super::super::artifacts::{S0ArtifactEnvelopeMetadata, S0NondeterministicMetadata};
use super::super::evidence::S0StableDigest;
use super::phrase_finding::TerminologyPhraseFinding;
use super::validation::{
    reject_duplicate_rows, require_non_empty, terminology_report_digest,
    TerminologyCleanupRejection, TerminologyRiskReportDigestBasis,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TerminologyRiskReport {
    #[serde(flatten)]
    pub(super) envelope: S0ArtifactEnvelopeMetadata,
    pub(super) rows: Vec<TerminologyPhraseFinding>,
    pub(super) scan_digest: S0StableDigest,
}

impl TerminologyRiskReport {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<TerminologyPhraseFinding>,
        scan_digest: S0StableDigest,
    ) -> Result<Self, TerminologyCleanupRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = terminology_report_digest(&TerminologyRiskReportDigestBasis {
            schema_version: super::super::artifacts::S0_ARTIFACT_SCHEMA_VERSION,
            artifact_kind: super::super::evidence::S0ArtifactKind::TerminologyRiskReport,
            source_revision: &source_revision,
            roadmap_parent_digest: &roadmap_parent_digest,
            generated_by: &generated_by,
            scan_digest: &scan_digest,
            rows: &rows,
        })?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                super::super::evidence::S0ArtifactKind::TerminologyRiskReport,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            rows,
            scan_digest,
        })
    }
}
