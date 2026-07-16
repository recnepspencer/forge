use crate::{
    IntegrityEvidenceCounters, IntegrityEvidenceLocality, IntegrityEvidenceOutcome,
    PhysicalIntegrityEvidenceBundle,
};
use worth_foundational::{FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole};
use worth_proof::{ProofOutcome, ProofOutcomeKind, TransitionOutcome};

pub type IntegrityProofProgressionOutcome = ProofOutcome<IntegrityProofProgressionSnapshot>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityProofProgressionSnapshot {
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
    outcome: IntegrityEvidenceOutcome,
    locality: IntegrityEvidenceLocality,
    counters: IntegrityEvidenceCounters,
}

impl IntegrityProofProgressionSnapshot {
    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub const fn boundary_role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub const fn outcome(&self) -> &IntegrityEvidenceOutcome {
        &self.outcome
    }

    pub const fn locality(&self) -> IntegrityEvidenceLocality {
        self.locality
    }

    pub const fn counters(&self) -> IntegrityEvidenceCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityProofProgressionReport {
    proof_outcome: IntegrityProofProgressionOutcome,
    claims_store_authority: bool,
    claims_repair_or_recovery: bool,
}

impl IntegrityProofProgressionReport {
    pub fn from_evidence(evidence: &PhysicalIntegrityEvidenceBundle) -> Self {
        let snapshot = IntegrityProofProgressionSnapshot {
            category: evidence.category(),
            role: evidence.boundary_role(),
            outcome: *evidence.integrity_outcome(),
            locality: evidence.locality(),
            counters: evidence.counters(),
        };
        Self {
            proof_outcome: ProofOutcome::from(TransitionOutcome::success(snapshot)),
            claims_store_authority: false,
            claims_repair_or_recovery: false,
        }
    }

    pub fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.snapshot().category
    }

    pub fn boundary_role(&self) -> FoundationalBoundaryArtifactRole {
        self.snapshot().role
    }

    pub fn outcome(&self) -> &IntegrityEvidenceOutcome {
        &self.snapshot().outcome
    }

    pub fn locality(&self) -> IntegrityEvidenceLocality {
        self.snapshot().locality
    }

    pub fn counters(&self) -> IntegrityEvidenceCounters {
        self.snapshot().counters
    }

    pub const fn proof_outcome(&self) -> &IntegrityProofProgressionOutcome {
        &self.proof_outcome
    }

    pub fn proof_outcome_kind(&self) -> ProofOutcomeKind {
        self.proof_outcome.kind()
    }

    pub const fn claims_store_authority(&self) -> bool {
        self.claims_store_authority
    }

    pub const fn claims_repair_or_recovery(&self) -> bool {
        self.claims_repair_or_recovery
    }

    fn snapshot(&self) -> &IntegrityProofProgressionSnapshot {
        match self.proof_outcome.as_raw() {
            TransitionOutcome::Success(snapshot) => snapshot,
            TransitionOutcome::Denied(_)
            | TransitionOutcome::Deferred(_)
            | TransitionOutcome::Stale(_)
            | TransitionOutcome::RebindRequired(_)
            | TransitionOutcome::Failed(_) => {
                unreachable!("S.3 proof reports are built from executed evidence")
            }
        }
    }
}
