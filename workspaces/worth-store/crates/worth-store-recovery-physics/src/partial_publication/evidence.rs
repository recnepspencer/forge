use crate::BackendResidueKind;

use super::{PartialPublicationCrashEdge, TornPublicationDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationEvidence {
    kind: PartialPublicationEvidenceKind,
    persisted_digest: String,
}

impl PartialPublicationEvidence {
    pub fn from_persisted_crash_edge(edge: PartialPublicationCrashEdge) -> Self {
        let persisted_digest = format!("persisted-crash-edge:{edge:?}");
        Self {
            kind: PartialPublicationEvidenceKind::PersistedCrashEdge(edge),
            persisted_digest,
        }
    }

    pub fn from_backend_residue(
        residue: BackendResidueKind,
        residue_digest: impl Into<String>,
    ) -> Self {
        Self {
            kind: PartialPublicationEvidenceKind::BackendResidueOnly { residue },
            persisted_digest: residue_digest.into(),
        }
    }

    pub fn from_live_ack_memory(memory_digest: impl Into<String>) -> Self {
        Self {
            kind: PartialPublicationEvidenceKind::LiveAcknowledgmentMemoryOnly,
            persisted_digest: memory_digest.into(),
        }
    }

    pub fn from_log_only(log_digest: impl Into<String>) -> Self {
        Self {
            kind: PartialPublicationEvidenceKind::LogOnly,
            persisted_digest: log_digest.into(),
        }
    }

    pub fn from_torn_publication(denial: TornPublicationDenial) -> Self {
        let persisted_digest = format!("torn-publication:{denial:?}");
        Self {
            kind: PartialPublicationEvidenceKind::TornPublication(denial),
            persisted_digest,
        }
    }

    pub fn insufficient_persisted_evidence(ambiguity_digest: impl Into<String>) -> Self {
        let ambiguity_digest = ambiguity_digest.into();
        Self {
            persisted_digest: ambiguity_digest.clone(),
            kind: PartialPublicationEvidenceKind::InsufficientPersistedEvidence {
                ambiguity_digest,
            },
        }
    }

    pub(crate) const fn kind(&self) -> &PartialPublicationEvidenceKind {
        &self.kind
    }

    pub fn persisted_digest(&self) -> &str {
        &self.persisted_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PartialPublicationEvidenceKind {
    PersistedCrashEdge(PartialPublicationCrashEdge),
    BackendResidueOnly { residue: BackendResidueKind },
    LiveAcknowledgmentMemoryOnly,
    LogOnly,
    TornPublication(TornPublicationDenial),
    InsufficientPersistedEvidence { ambiguity_digest: String },
}
