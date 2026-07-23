use crate::basis_lifecycle::BasisOperationLane;

use super::super::super::WorthQueryBoundDomainOperation;
use super::super::authority::{
    require_current_same_installation_after_runtime, require_same_runtime,
};
use super::super::canonical::compare_admitted_bases;
use super::super::conditional_comparison::compare_conditional_continuity;
use super::super::denial::{WorthQueryBasisCompatibilityDenial, WorthQueryCompatibilityCounters};
use super::super::portable_comparison::compare_portable_operation;
use super::super::witness::{
    WorthQueryBasisCompatibilityEvidence, WorthQueryBasisCompatibilityWitness,
    WorthQueryPortableAndBasisEvidence,
};

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundDomainOperation<D, O, F, L> {
    pub fn compatible_basis_with(
        &self,
        candidate: &Self,
    ) -> Result<WorthQueryBasisCompatibilityWitness, WorthQueryBasisCompatibilityDenial> {
        let mut counters = WorthQueryCompatibilityCounters::default();
        let portable = compare_portable_operation(self, candidate, &mut counters)?;
        require_same_runtime(self, candidate, &mut counters)?;
        require_current_same_installation_after_runtime(self, candidate, &mut counters)?;
        let basis = compare_admitted_bases(self.basis(), candidate.basis(), &mut counters)?;
        let conditional = compare_conditional_continuity(self, candidate, &mut counters)?;
        Ok(WorthQueryBasisCompatibilityWitness::mint(
            self,
            candidate,
            WorthQueryBasisCompatibilityEvidence::new(
                WorthQueryPortableAndBasisEvidence::new(portable, basis),
                conditional,
            ),
            counters,
        ))
    }
}
