use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::capability::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId,
};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::super::milestones::SemanticPhysicalClaimFamily;
use super::claim_policy::SemanticPhysicalClaimStatus;
use super::claim_report_row::SemanticPhysicalClaimReportRow;
use super::claim_validation::{S0ClaimReportBuildRejection, S0ClaimReportParseRejection};

#[derive(serde::Deserialize)]
pub(super) struct RawSemanticPhysicalClaimReport {
    #[serde(flatten)]
    pub(super) envelope: RawClaimReportEnvelope,
    pub(super) rows: Vec<RawSemanticPhysicalClaimReportRow>,
}

#[derive(serde::Deserialize)]
pub(super) struct RawClaimReportEnvelope {
    pub(super) schema_version: String,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: String,
    pub(super) generated_by: String,
    pub(super) deterministic_digest: String,
    pub(super) nondeterministic_metadata: RawClaimNondeterministicMetadata,
}

#[derive(serde::Deserialize)]
pub(super) struct RawClaimNondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl RawClaimNondeterministicMetadata {
    pub(super) fn into_validated(
        self,
    ) -> Result<super::super::artifacts::S0NondeterministicMetadata, S0ClaimReportParseRejection>
    {
        super::super::artifacts::S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0ClaimReportParseRejection::RowBuildRejected(
                S0ClaimReportBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawSemanticPhysicalClaimReportRow {
    row_id: String,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<RawClaimEvidenceRef>,
    forbidden_claims: Vec<RawClaimForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    claim_family: SemanticPhysicalClaimFamily,
    claim_status: SemanticPhysicalClaimStatus,
    semantic_capability_proven: String,
    closeout_or_planned_source: String,
    named_suite: String,
    evidence_lanes: Vec<String>,
}

impl RawSemanticPhysicalClaimReportRow {
    pub(super) fn into_validated(
        self,
    ) -> Result<SemanticPhysicalClaimReportRow, S0ClaimReportParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0ClaimReportParseRejection::RowBuildRejected(
                S0ClaimReportBuildRejection::EmptyRequiredField,
            )
        })?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawClaimEvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let forbidden_claims = self
            .forbidden_claims
            .into_iter()
            .map(RawClaimForbiddenClaim::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0ClaimReportParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        SemanticPhysicalClaimReportRow::new(
            row_id,
            self.subject_kind,
            self.subject_path_or_symbol,
            self.classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.claim_family,
            self.claim_status,
            self.semantic_capability_proven,
            self.closeout_or_planned_source,
            self.named_suite,
            self.evidence_lanes,
        )
        .map_err(S0ClaimReportParseRejection::RowBuildRejected)
    }
}

#[derive(serde::Deserialize)]
struct RawClaimEvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawClaimEvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0ClaimReportParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(serde::Deserialize)]
struct RawClaimForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawClaimForbiddenClaim {
    fn into_validated(self) -> Result<BackendForbiddenClaim, S0ClaimReportParseRejection> {
        BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDeferredSequence)
    }
}
