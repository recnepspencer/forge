use super::super::super::{WorthQueryBoundDomainOperation, WorthQueryDomainRebindReceipt};
use super::super::conditional_comparison::WorthQueryConditionalContinuityEvidence;
use super::super::denial::{WorthQueryCompatibilityCounters, WorthQueryCompatibilityUseDenial};
use super::authority::{
    mint_pair_proof, WorthQueryCompatibilityProof, WorthQueryPortableAndBasisEvidence,
};
use crate::basis_lifecycle::BasisOperationLane;
use std::fmt;
use worth_proof::PhaseMarker;

#[derive(Debug)]
struct WorthQueryRebindCompatibilityPhase;
impl PhaseMarker for WorthQueryRebindCompatibilityPhase {}
pub(in crate::domain_installation::compatibility) struct WorthQueryRebindEvidence {
    common: WorthQueryPortableAndBasisEvidence,
    conditional: WorthQueryConditionalContinuityEvidence,
    required_domain_receipts: Vec<WorthQueryDomainRebindReceipt>,
}
impl WorthQueryRebindEvidence {
    pub(in crate::domain_installation::compatibility) fn new(
        common: WorthQueryPortableAndBasisEvidence,
        conditional: WorthQueryConditionalContinuityEvidence,
        required_domain_receipts: Vec<WorthQueryDomainRebindReceipt>,
    ) -> Self {
        Self {
            common,
            conditional,
            required_domain_receipts,
        }
    }
}
pub struct WorthQueryRebindWitness {
    proof: WorthQueryCompatibilityProof<WorthQueryRebindCompatibilityPhase>,
    evidence: WorthQueryRebindEvidence,
    domain_rebind_receipt: WorthQueryDomainRebindReceipt,
    counters: WorthQueryCompatibilityCounters,
}
impl WorthQueryRebindWitness {
    pub(in crate::domain_installation::compatibility) fn mint<D, O, F, L: BasisOperationLane>(
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
        evidence: WorthQueryRebindEvidence,
        domain_rebind_receipt: WorthQueryDomainRebindReceipt,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        Self {
            proof: mint_pair_proof("rebind", subject, candidate),
            evidence,
            domain_rebind_receipt,
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
        if !pair.matches_rebind_pair(subject, candidate) {
            return Err(WorthQueryCompatibilityUseDenial::StaleAuthority);
        }
        if !self.evidence.conditional.candidate_is_live() {
            return Err(WorthQueryCompatibilityUseDenial::StaleConditionalLowering);
        }
        Ok(self)
    }
    pub fn counters(&self) -> WorthQueryCompatibilityCounters {
        self.counters
    }
}
impl fmt::Debug for WorthQueryRebindWitness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorthQueryRebindWitness")
            .field("relationship", &self.proof.payload().relationship())
            .field(
                "retained_comparison_evidence",
                &(self.evidence.common.comparison_count()
                    + self.evidence.conditional.count() as u32),
            )
            .field(
                "required_domain_rebind_receipts",
                &self.evidence.required_domain_receipts.len(),
            )
            .field(
                "domain_rebind_receipt",
                self.domain_rebind_receipt.receipt_identity(),
            )
            .finish_non_exhaustive()
    }
}
