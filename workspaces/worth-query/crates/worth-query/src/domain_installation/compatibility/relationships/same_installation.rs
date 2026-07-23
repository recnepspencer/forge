use crate::basis_lifecycle::BasisOperationLane;

use super::super::super::WorthQueryBoundDomainOperation;
use super::super::authority::{
    require_current_same_installation_after_runtime, require_same_runtime,
};
use super::super::conditional_comparison::compare_conditional_affinity;
use super::super::denial::{WorthQueryCompatibilityCounters, WorthQuerySameInstallationDenial};
use super::super::witness::{
    WorthQuerySameInstallationEvidence, WorthQuerySameInstallationWitness,
};

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundDomainOperation<D, O, F, L> {
    pub fn same_installation_with(
        &self,
        candidate: &Self,
    ) -> Result<WorthQuerySameInstallationWitness, WorthQuerySameInstallationDenial> {
        let mut counters = WorthQueryCompatibilityCounters::default();
        require_same_runtime(self, candidate, &mut counters)?;
        require_current_same_installation_after_runtime(self, candidate, &mut counters)?;
        let conditional = compare_conditional_affinity(self, candidate, &mut counters)?;
        Ok(WorthQuerySameInstallationWitness::mint(
            self,
            candidate,
            WorthQuerySameInstallationEvidence::new(conditional),
            counters,
        ))
    }
}
