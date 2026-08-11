use super::super::artifacts::{S0ArtifactRowId, S0ArtifactRowStatus, S0ArtifactSubjectKind};
use super::super::evidence::{S0ArtifactKind, S0EvidenceRef};
use super::validation::S0EvidenceBundleBuildRejection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S0CertificationStatus {
    Verified,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0CertificationMatrixRow {
    pub(super) row_id: S0ArtifactRowId,
    pub(super) subject_kind: S0ArtifactSubjectKind,
    pub(super) subject_path_or_symbol: String,
    pub(super) classification: String,
    pub(super) evidence_refs: Vec<S0EvidenceRef>,
    pub(super) status: S0ArtifactRowStatus,
    pub(super) notes: String,
    pub(super) certification_status: S0CertificationStatus,
}

impl S0CertificationMatrixRow {
    pub(super) fn new(
        row_id: &str,
        notes: impl Into<String>,
        certification_status: S0CertificationStatus,
        evidence_refs: Vec<S0EvidenceRef>,
    ) -> Result<Self, S0EvidenceBundleBuildRejection> {
        if evidence_refs.is_empty() {
            return Err(S0EvidenceBundleBuildRejection::MissingEvidenceRef);
        }
        Ok(Self {
            row_id: S0ArtifactRowId::new(row_id)
                .map_err(|_| S0EvidenceBundleBuildRejection::InvalidRowId)?,
            subject_kind: S0ArtifactSubjectKind::EvidenceLane,
            subject_path_or_symbol: "storage_foundation::s0".to_string(),
            classification: "s0-certification-matrix".to_string(),
            evidence_refs,
            status: match certification_status {
                S0CertificationStatus::Verified => S0ArtifactRowStatus::Admitted,
                S0CertificationStatus::Blocking => S0ArtifactRowStatus::Deferred,
            },
            notes: notes.into(),
            certification_status,
        })
    }

    pub fn certification_status(&self) -> S0CertificationStatus {
        self.certification_status
    }
}

pub(super) fn certification_evidence_ref(
    kind: S0ArtifactKind,
    digest: &super::super::evidence::S0StableDigest,
) -> S0EvidenceRef {
    S0EvidenceRef::new(kind, digest.clone())
}
