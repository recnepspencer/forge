use std::fmt;

use worth_proof::PhaseMarker;

use crate::basis_lifecycle::BasisOperationLane;

use super::super::super::WorthQueryBoundDomainOperation;
use super::super::conditional_comparison::WorthQueryConditionalAffinityEvidence;
use super::super::denial::WorthQueryCompatibilityCounters;
use super::authority::{mint_pair_proof, WorthQueryCompatibilityProof};

#[derive(Debug)]
struct WorthQuerySameInstallationPhase;
impl PhaseMarker for WorthQuerySameInstallationPhase {}

pub(in crate::domain_installation::compatibility) struct WorthQuerySameInstallationEvidence {
    conditional: WorthQueryConditionalAffinityEvidence,
}

impl WorthQuerySameInstallationEvidence {
    pub(in crate::domain_installation::compatibility) fn new(
        conditional: WorthQueryConditionalAffinityEvidence,
    ) -> Self {
        Self { conditional }
    }
}

pub struct WorthQuerySameInstallationWitness {
    proof: WorthQueryCompatibilityProof<WorthQuerySameInstallationPhase>,
    evidence: WorthQuerySameInstallationEvidence,
    counters: WorthQueryCompatibilityCounters,
}

impl WorthQuerySameInstallationWitness {
    pub(in crate::domain_installation::compatibility) fn mint<D, O, F, L: BasisOperationLane>(
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
        evidence: WorthQuerySameInstallationEvidence,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        Self {
            proof: mint_pair_proof("same-installation", subject, candidate),
            evidence,
            counters,
        }
    }

    pub fn counters(&self) -> WorthQueryCompatibilityCounters {
        self.counters
    }
}

impl fmt::Debug for WorthQuerySameInstallationWitness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthQuerySameInstallationWitness")
            .field("relationship", &self.proof.payload().relationship())
            .field(
                "conditional_comparisons",
                &self.evidence.conditional.count(),
            )
            .finish_non_exhaustive()
    }
}
