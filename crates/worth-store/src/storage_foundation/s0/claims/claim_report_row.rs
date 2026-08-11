use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::capability::{BackendForbiddenClaim, Roadmap2SequenceId};
use super::super::evidence::S0EvidenceRef;
use super::super::milestones::SemanticPhysicalClaimFamily;
use super::claim_policy::{claim_status_requires_deferred_mapping, SemanticPhysicalClaimStatus};
use super::claim_validation::{require_non_empty, S0ClaimReportBuildRejection};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SemanticPhysicalClaimReportRow {
    pub(super) row_id: S0ArtifactRowId,
    pub(super) subject_kind: S0ArtifactSubjectKind,
    pub(super) subject_path_or_symbol: String,
    pub(super) classification: String,
    pub(super) evidence_refs: Vec<S0EvidenceRef>,
    pub(super) forbidden_claims: Vec<BackendForbiddenClaim>,
    pub(super) deferred_s_sequences: Vec<Roadmap2SequenceId>,
    pub(super) status: S0ArtifactRowStatus,
    pub(super) notes: String,
    pub(super) claim_family: SemanticPhysicalClaimFamily,
    pub(super) claim_status: SemanticPhysicalClaimStatus,
    pub(super) semantic_capability_proven: String,
    pub(super) closeout_or_planned_source: String,
    pub(super) named_suite: String,
    pub(super) evidence_lanes: Vec<String>,
}

impl SemanticPhysicalClaimReportRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: S0ArtifactRowId,
        subject_kind: S0ArtifactSubjectKind,
        subject_path_or_symbol: impl Into<String>,
        classification: impl Into<String>,
        evidence_refs: Vec<S0EvidenceRef>,
        forbidden_claims: Vec<BackendForbiddenClaim>,
        deferred_s_sequences: Vec<Roadmap2SequenceId>,
        status: S0ArtifactRowStatus,
        notes: impl Into<String>,
        claim_family: SemanticPhysicalClaimFamily,
        claim_status: SemanticPhysicalClaimStatus,
        semantic_capability_proven: impl Into<String>,
        closeout_or_planned_source: impl Into<String>,
        named_suite: impl Into<String>,
        evidence_lanes: Vec<String>,
    ) -> Result<Self, S0ClaimReportBuildRejection> {
        let subject_path_or_symbol = require_non_empty(subject_path_or_symbol)?;
        let classification = require_non_empty(classification)?;
        let notes = require_non_empty(notes)?;
        let semantic_capability_proven = require_non_empty(semantic_capability_proven)?;
        let closeout_or_planned_source = require_non_empty(closeout_or_planned_source)?;
        let named_suite = require_non_empty(named_suite)?;
        if evidence_refs.is_empty() {
            return Err(S0ClaimReportBuildRejection::MissingEvidenceRef);
        }
        if evidence_lanes.is_empty() {
            return Err(S0ClaimReportBuildRejection::MissingEvidenceLane);
        }
        if claim_status_requires_deferred_mapping(claim_family, claim_status)
            && deferred_s_sequences.is_empty()
        {
            return Err(S0ClaimReportBuildRejection::DeferredSequenceMissing);
        }
        Ok(Self {
            row_id,
            subject_kind,
            subject_path_or_symbol,
            classification,
            evidence_refs,
            forbidden_claims,
            deferred_s_sequences,
            status,
            notes,
            claim_family,
            claim_status,
            semantic_capability_proven,
            closeout_or_planned_source,
            named_suite,
            evidence_lanes,
        })
    }

    pub fn row_id(&self) -> &S0ArtifactRowId {
        &self.row_id
    }

    pub fn claim_family(&self) -> SemanticPhysicalClaimFamily {
        self.claim_family
    }

    pub fn claim_status(&self) -> SemanticPhysicalClaimStatus {
        self.claim_status
    }
}
