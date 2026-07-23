use crate::basis_lifecycle::BasisOperationLane;

use super::super::super::{WorthQueryBoundDomainOperation, WorthQueryDomainRebindReceipt};
use super::super::authority::{require_rebind_successor, require_same_runtime};
use super::super::canonical::compare_admitted_bases;
use super::super::conditional_comparison::compare_conditional_continuity;
use super::super::denial::{WorthQueryCompatibilityCounters, WorthQueryRebindCompatibilityDenial};
use super::super::portable_comparison::compare_portable_operation;
use super::super::witness::{
    WorthQueryPortableAndBasisEvidence, WorthQueryRebindEvidence, WorthQueryRebindWitness,
};

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundDomainOperation<D, O, F, L> {
    pub fn rebind_with(
        &self,
        candidate: &Self,
        receipt: WorthQueryDomainRebindReceipt,
    ) -> Result<WorthQueryRebindWitness, WorthQueryRebindCompatibilityDenial> {
        self.rebind_with_required_domain_receipts(candidate, receipt, Vec::new())
    }

    pub fn rebind_with_required_domain_receipts(
        &self,
        candidate: &Self,
        receipt: WorthQueryDomainRebindReceipt,
        required_domain_receipts: Vec<WorthQueryDomainRebindReceipt>,
    ) -> Result<WorthQueryRebindWitness, WorthQueryRebindCompatibilityDenial> {
        let mut counters = WorthQueryCompatibilityCounters::default();
        let portable = compare_portable_operation(self, candidate, &mut counters)?;
        require_same_runtime(self, candidate, &mut counters)?;
        require_rebind_successor(
            self,
            candidate,
            &receipt,
            &required_domain_receipts,
            &mut counters,
        )?;
        let basis = compare_admitted_bases(self.basis(), candidate.basis(), &mut counters)?;
        let conditional = compare_conditional_continuity(self, candidate, &mut counters)?;
        Ok(WorthQueryRebindWitness::mint(
            self,
            candidate,
            WorthQueryRebindEvidence::new(
                WorthQueryPortableAndBasisEvidence::new(portable, basis),
                conditional,
                required_domain_receipts,
            ),
            receipt,
            counters,
        ))
    }
}
