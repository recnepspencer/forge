use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::capability::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, Roadmap2SequenceId,
};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef, S0StableDigest};
use super::super::milestones::S0PhysicalStatus;
use super::deferred_category_policy::DeferredPhysicalGuaranteeCategory;
use super::deferred_guarantee_row::DeferredPhysicalGuaranteeRow;
use super::deferred_validation::{
    S0DeferredGuaranteeBuildRejection, S0DeferredGuaranteeParseRejection,
};

#[derive(serde::Deserialize)]
pub(super) struct RawDeferredPhysicalGuaranteeMap {
    #[serde(flatten)]
    pub(super) envelope: RawDeferredGuaranteeEnvelope,
    pub(super) rows: Vec<RawDeferredPhysicalGuaranteeRow>,
}

#[derive(serde::Deserialize)]
pub(super) struct RawDeferredGuaranteeEnvelope {
    pub(super) schema_version: String,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: String,
    pub(super) generated_by: String,
    pub(super) deterministic_digest: String,
    pub(super) nondeterministic_metadata: RawDeferredNondeterministicMetadata,
}

#[derive(serde::Deserialize)]
pub(super) struct RawDeferredNondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl RawDeferredNondeterministicMetadata {
    pub(super) fn into_validated(
        self,
    ) -> Result<
        super::super::artifacts::S0NondeterministicMetadata,
        S0DeferredGuaranteeParseRejection,
    > {
        super::super::artifacts::S0NondeterministicMetadata::excluded(
            self.generated_at_policy,
            self.local_path_hint,
            self.host_hint,
        )
        .map_err(|_| {
            S0DeferredGuaranteeParseRejection::RowBuildRejected(
                S0DeferredGuaranteeBuildRejection::EmptyRequiredField,
            )
        })
    }
}

#[derive(serde::Deserialize)]
pub(super) struct RawDeferredPhysicalGuaranteeRow {
    row_id: String,
    subject_kind: S0ArtifactSubjectKind,
    subject_path_or_symbol: String,
    classification: String,
    evidence_refs: Vec<RawDeferredEvidenceRef>,
    forbidden_claims: Vec<RawDeferredForbiddenClaim>,
    deferred_s_sequences: Vec<String>,
    status: S0ArtifactRowStatus,
    notes: String,
    guarantee_category: DeferredPhysicalGuaranteeCategory,
    current_evidence_status: S0PhysicalStatus,
    missing_proof: String,
    dependent_named_suite: String,
    dependent_evidence_lanes: Vec<String>,
}

impl RawDeferredPhysicalGuaranteeRow {
    pub(super) fn into_validated(
        self,
    ) -> Result<DeferredPhysicalGuaranteeRow, S0DeferredGuaranteeParseRejection> {
        let row_id = S0ArtifactRowId::new(self.row_id).map_err(|_| {
            S0DeferredGuaranteeParseRejection::RowBuildRejected(
                S0DeferredGuaranteeBuildRejection::EmptyRequiredField,
            )
        })?;
        let evidence_refs = self
            .evidence_refs
            .into_iter()
            .map(RawDeferredEvidenceRef::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let forbidden_claims = self
            .forbidden_claims
            .into_iter()
            .map(RawDeferredForbiddenClaim::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let deferred_s_sequences = self
            .deferred_s_sequences
            .into_iter()
            .map(|sequence| {
                Roadmap2SequenceId::new(sequence)
                    .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDeferredSequence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        DeferredPhysicalGuaranteeRow::new(
            row_id,
            self.subject_kind,
            self.subject_path_or_symbol,
            self.classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            self.status,
            self.notes,
            self.guarantee_category,
            self.current_evidence_status,
            self.missing_proof,
            self.dependent_named_suite,
            self.dependent_evidence_lanes,
        )
        .map_err(S0DeferredGuaranteeParseRejection::RowBuildRejected)
    }
}

#[derive(serde::Deserialize)]
struct RawDeferredEvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: String,
}

impl RawDeferredEvidenceRef {
    fn into_validated(self) -> Result<S0EvidenceRef, S0DeferredGuaranteeParseRejection> {
        let digest = S0StableDigest::new(self.digest)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDigest)?;
        Ok(S0EvidenceRef::new(self.artifact_kind, digest))
    }
}

#[derive(serde::Deserialize)]
struct RawDeferredForbiddenClaim {
    claim_kind: BackendForbiddenClaimKind,
    deferred_sequence: String,
}

impl RawDeferredForbiddenClaim {
    fn into_validated(self) -> Result<BackendForbiddenClaim, S0DeferredGuaranteeParseRejection> {
        BackendForbiddenClaim::new(self.claim_kind, self.deferred_sequence)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDeferredSequence)
    }
}
