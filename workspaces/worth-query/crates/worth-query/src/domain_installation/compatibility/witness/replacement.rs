use super::super::super::WorthQueryBoundDomainOperation;
use super::super::conditional_comparison::WorthQueryConditionalAffinityEvidence;
use super::super::denial::{WorthQueryCompatibilityCounters, WorthQueryCompatibilityUseDenial};
use super::authority::{
    mint_pair_proof, WorthQueryCompatibilityProof, WorthQueryPortableAndBasisEvidence,
};
use crate::basis_lifecycle::BasisOperationLane;
use std::fmt;
use worth_proof::PhaseMarker;

#[derive(Debug)]
struct WorthQueryReplacementPhase;
impl PhaseMarker for WorthQueryReplacementPhase {}

pub(in crate::domain_installation::compatibility) struct WorthQueryReplacementEvidence {
    common: WorthQueryPortableAndBasisEvidence,
    conditional: WorthQueryConditionalAffinityEvidence,
}
impl WorthQueryReplacementEvidence {
    pub(in crate::domain_installation::compatibility) fn new(
        common: WorthQueryPortableAndBasisEvidence,
        conditional: WorthQueryConditionalAffinityEvidence,
    ) -> Self {
        Self {
            common,
            conditional,
        }
    }
}

pub struct WorthQueryReplacementWitness {
    proof: WorthQueryCompatibilityProof<WorthQueryReplacementPhase>,
    evidence: WorthQueryReplacementEvidence,
    counters: WorthQueryCompatibilityCounters,
}
impl WorthQueryReplacementWitness {
    pub(in crate::domain_installation::compatibility) fn mint<D, O, F, L: BasisOperationLane>(
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
        evidence: WorthQueryReplacementEvidence,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        Self {
            proof: mint_pair_proof("replacement", subject, candidate),
            evidence,
            counters,
        }
    }
    pub(crate) fn readmit_for_pair<D, O, F, L: BasisOperationLane>(
        self,
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> Result<Self, WorthQueryCompatibilityUseDenial> {
        let pair = self.proof.basis().basis().value();
        if !pair.matches(subject, candidate) {
            return Err(WorthQueryCompatibilityUseDenial::WrongCapabilityPair);
        }
        if !pair.matches_current_pair(subject, candidate) {
            return Err(WorthQueryCompatibilityUseDenial::StaleAuthority);
        }
        if !self.evidence.conditional.both_are_live() {
            return Err(WorthQueryCompatibilityUseDenial::StaleConditionalLowering);
        }
        Ok(self)
    }
    pub fn counters(&self) -> WorthQueryCompatibilityCounters {
        self.counters
    }
}
impl fmt::Debug for WorthQueryReplacementWitness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorthQueryReplacementWitness")
            .field("relationship", &self.proof.payload().relationship())
            .field(
                "retained_comparison_evidence",
                &(self.evidence.common.comparison_count()
                    + self.evidence.conditional.count() as u32),
            )
            .finish_non_exhaustive()
    }
}
