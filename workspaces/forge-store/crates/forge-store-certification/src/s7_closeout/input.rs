use forge_store_physical_certification::{
    materialize_s7_closeout_evidence, S7ExecutedCloseoutSources,
    S7MaterializedCloseoutEvidenceBundle,
};
use forge_store_readiness::S6ClosedS7PlacementAdmissionSeed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S7CloseoutEvidencePolicy {
    counter_backed_foundational: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7CloseoutCertificationInput {
    materialized_evidence: S7MaterializedCloseoutEvidenceBundle,
    policy: S7CloseoutEvidencePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S7CloseoutShortcutInput {
    CopiedReceipt,
    CopiedChunkRows {
        row_count: usize,
    },
    CopiedProofId {
        proof_id: String,
    },
    S6PlacementReadinessOnly {
        seed: S6ClosedS7PlacementAdmissionSeed,
    },
    S5FutureChunkPlaceholderOnly {
        label: String,
    },
    TerminalProjectionOnly,
    RawCountersOnly {
        row_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S7CloseoutRequest {
    Canonical(S7CloseoutCertificationInput),
    Shortcut(S7CloseoutShortcutInput),
}

impl S7CloseoutEvidencePolicy {
    pub const fn counter_backed_foundational() -> Self {
        Self {
            counter_backed_foundational: true,
        }
    }

    pub const fn is_counter_backed_foundational(self) -> bool {
        self.counter_backed_foundational
    }
}

impl S7CloseoutCertificationInput {
    pub fn from_executed_sources(
        executed_sources: S7ExecutedCloseoutSources,
        policy: S7CloseoutEvidencePolicy,
    ) -> Self {
        Self {
            materialized_evidence: materialize_s7_closeout_evidence(executed_sources),
            policy,
        }
    }

    pub const fn materialized_evidence(&self) -> &S7MaterializedCloseoutEvidenceBundle {
        &self.materialized_evidence
    }

    pub(crate) fn into_materialized_evidence(self) -> S7MaterializedCloseoutEvidenceBundle {
        self.materialized_evidence
    }

    pub const fn policy(&self) -> S7CloseoutEvidencePolicy {
        self.policy
    }
}
