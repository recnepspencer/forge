use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0NondeterministicMetadata};
use super::super::capability::Roadmap2SequenceId;
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::fixtures::S1ForbiddenShortcut;
use super::maturity::{
    ForbiddenShortcutDetectionStatus, HarnessMaturityLevel, HarnessSubsystemMaturity,
};
use super::row::HarnessMaturityRow;
use super::validation::{S0HarnessMaturityBuildRejection, S0HarnessMaturityParseRejection};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct RawHarnessMaturityReport {
    #[serde(flatten)]
    pub(super) envelope: RawS0ArtifactEnvelope,
    pub(super) rows: Vec<RawHarnessMaturityRow>,
    pub(super) evidence_bundle_readiness: super::maturity::EvidenceBundleReadiness,
}

#[derive(Deserialize)]
pub(super) struct RawHarnessMaturityRow {
    pub(super) row_id: String,
    pub(super) subject_path_or_symbol: String,
    pub(super) evidence_refs: Vec<RawS0EvidenceRef>,
    pub(super) status: S0ArtifactRowStatus,
    pub(super) notes: String,
    pub(super) subsystem: HarnessSubsystemMaturity,
    pub(super) maturity_level: HarnessMaturityLevel,
    pub(super) required_for_sequences: Vec<String>,
    pub(super) forbidden_shortcuts_covered: Vec<S1ForbiddenShortcut>,
    pub(super) detection_status: ForbiddenShortcutDetectionStatus,
}

impl RawHarnessMaturityRow {
    pub(super) fn into_validated(
        self,
    ) -> Result<HarnessMaturityRow, S0HarnessMaturityParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0HarnessMaturityParseRejection::RowBuildRejected(
                S0HarnessMaturityBuildRejection::EmptyRequiredField,
            )
        })?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawS0EvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let required_for_sequences = self
            .required_for_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0HarnessMaturityParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        HarnessMaturityRow::new(
            row_id,
            self.subject_path_or_symbol,
            evidence_refs,
            self.status,
            self.notes,
            self.subsystem,
            self.maturity_level,
            required_for_sequences,
            self.forbidden_shortcuts_covered,
            self.detection_status,
        )
        .map_err(S0HarnessMaturityParseRejection::RowBuildRejected)
    }
}

#[derive(Deserialize)]
pub(super) struct RawS0ArtifactEnvelope {
    pub(super) schema_version: String,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: String,
    pub(super) generated_by: String,
    pub(super) deterministic_digest: String,
    pub(super) nondeterministic_metadata: RawS0NondeterministicMetadata,
}

#[derive(Deserialize)]
pub(super) struct RawS0NondeterministicMetadata {
    pub(super) generated_at_policy: String,
    pub(super) local_path_hint: Option<String>,
    pub(super) host_hint: Option<String>,
}

impl RawS0NondeterministicMetadata {
    pub(super) fn into_validated(
        self,
    ) -> Result<S0NondeterministicMetadata, S0HarnessMaturityParseRejection> {
        S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0HarnessMaturityParseRejection::RowBuildRejected(
                S0HarnessMaturityBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(Deserialize)]
pub(super) struct RawS0EvidenceRef {
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) digest: String,
}

impl RawS0EvidenceRef {
    pub(super) fn into_validated(self) -> Result<S0EvidenceRef, S0HarnessMaturityParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0HarnessMaturityParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}
