use std::fmt;
use worth_proof::PhaseMarker;

use crate::basis_lifecycle::BasisOperationLane;

use super::super::super::WorthQueryBoundDomainOperation;
use super::super::conditional_comparison::WorthQueryConditionalContinuityEvidence;
use super::super::denial::WorthQueryCompatibilityCounters;
use super::authority::{
    mint_pair_proof, WorthQueryCompatibilityProof, WorthQueryPortableAndBasisEvidence,
};

#[derive(Debug)]
struct WorthQueryBasisCompatibilityPhase;
impl PhaseMarker for WorthQueryBasisCompatibilityPhase {}

pub(in crate::domain_installation::compatibility) struct WorthQueryBasisCompatibilityEvidence {
    common: WorthQueryPortableAndBasisEvidence,
    conditional: WorthQueryConditionalContinuityEvidence,
}

impl WorthQueryBasisCompatibilityEvidence {
    pub(in crate::domain_installation::compatibility) fn new(
        common: WorthQueryPortableAndBasisEvidence,
        conditional: WorthQueryConditionalContinuityEvidence,
    ) -> Self {
        Self {
            common,
            conditional,
        }
    }
}

pub struct WorthQueryBasisCompatibilityWitness {
    proof: WorthQueryCompatibilityProof<WorthQueryBasisCompatibilityPhase>,
    evidence: WorthQueryBasisCompatibilityEvidence,
    counters: WorthQueryCompatibilityCounters,
}

impl WorthQueryBasisCompatibilityWitness {
    pub(in crate::domain_installation::compatibility) fn mint<D, O, F, L: BasisOperationLane>(
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
        evidence: WorthQueryBasisCompatibilityEvidence,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        Self {
            proof: mint_pair_proof("basis-compatibility", subject, candidate),
            evidence,
            counters,
        }
    }

    pub fn counters(&self) -> WorthQueryCompatibilityCounters {
        self.counters
    }
}

impl fmt::Debug for WorthQueryBasisCompatibilityWitness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorthQueryBasisCompatibilityWitness")
            .field("relationship", &self.proof.payload().relationship())
            .field(
                "retained_comparison_evidence",
                &(self.evidence.common.comparison_count()
                    + self.evidence.conditional.count() as u32),
            )
            .finish_non_exhaustive()
    }
}
