use crate::basis_lifecycle::BasisOperationLane;

use super::super::super::WorthQueryBoundDomainOperation;
use super::super::authority::{
    require_current_same_installation_after_runtime, require_same_runtime,
};
use super::super::canonical::compare_admitted_bases;
use super::super::conditional_comparison::compare_conditional_affinity;
use super::super::denial::{WorthQueryCompatibilityCounters, WorthQueryExecutionSharingDenial};
use super::super::distinct_pair::require_distinct_capabilities;
use super::super::execution_sharing_policy::require_execution_sharing;
use super::super::portable_comparison::compare_portable_operation;
use super::super::witness::{
    WorthQueryExecutionSharingEvidence, WorthQueryExecutionSharingWitness,
    WorthQueryPortableAndBasisEvidence,
};

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundDomainOperation<D, O, F, L> {
    pub fn execution_sharing_with(
        &self,
        candidate: &Self,
    ) -> Result<WorthQueryExecutionSharingWitness, WorthQueryExecutionSharingDenial> {
        let mut counters = WorthQueryCompatibilityCounters::default();
        let portable = compare_portable_operation(self, candidate, &mut counters)?;
        require_same_runtime(self, candidate, &mut counters)?;
        require_current_same_installation_after_runtime(self, candidate, &mut counters)?;
        let basis = compare_admitted_bases(self.basis(), candidate.basis(), &mut counters)?;
        let conditional = compare_conditional_affinity(self, candidate, &mut counters)?;
        require_distinct_capabilities(self, candidate, &mut counters)?;
        require_execution_sharing(self, candidate, &mut counters)?;
        Ok(WorthQueryExecutionSharingWitness::mint(
            self,
            candidate,
            WorthQueryExecutionSharingEvidence::new(
                WorthQueryPortableAndBasisEvidence::new(portable, basis),
                conditional,
            ),
            counters,
        ))
    }
}
