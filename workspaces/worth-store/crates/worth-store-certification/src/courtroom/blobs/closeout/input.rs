use super::{materialize_blob_closeout_evidence, BlobCloseoutEvidenceBundle, BlobCloseoutSources};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCloseoutEvidencePolicy {
    counter_backed_foundational: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCloseoutCertificationInput {
    materialized_evidence: BlobCloseoutEvidenceBundle,
    policy: BlobCloseoutEvidencePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobCloseoutShortcutInput {
    CopiedReceipt,
    CopiedChunkRows { row_count: usize },
    CopiedProofId { proof_id: String },
    FutureChunkPlaceholderOnly { label: String },
    TerminalProjectionOnly,
    RawCountersOnly { row_count: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobCloseoutRequest {
    Canonical(Box<BlobCloseoutCertificationInput>),
    Shortcut(BlobCloseoutShortcutInput),
}

impl BlobCloseoutEvidencePolicy {
    pub const fn counter_backed_foundational() -> Self {
        Self {
            counter_backed_foundational: true,
        }
    }

    pub const fn is_counter_backed_foundational(self) -> bool {
        self.counter_backed_foundational
    }
}

impl BlobCloseoutCertificationInput {
    pub fn from_executed_sources(
        executed_sources: BlobCloseoutSources,
        policy: BlobCloseoutEvidencePolicy,
    ) -> Self {
        Self {
            materialized_evidence: materialize_blob_closeout_evidence(executed_sources),
            policy,
        }
    }

    pub const fn materialized_evidence(&self) -> &BlobCloseoutEvidenceBundle {
        &self.materialized_evidence
    }

    pub(crate) fn into_materialized_evidence(self) -> BlobCloseoutEvidenceBundle {
        self.materialized_evidence
    }

    pub const fn policy(&self) -> BlobCloseoutEvidencePolicy {
        self.policy
    }
}
