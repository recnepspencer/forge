use std::sync::Arc;

use crate::basis_lifecycle::{AdmittedBasisCapability, BasisOperationLane};
use crate::domain_installation::{
    WorthQueryBoundCapabilityGeneration, WorthQueryDomainInstallationGeneration,
    WorthQueryInstalledDomainAuthority,
};
use crate::runtime::WorthQueryRuntimeAuthorityIdentity;

#[derive(Clone)]
pub(in crate::domain_installation::operation_execution) struct WorthQueryProjectionLifecycleBasis<
    L: BasisOperationLane,
> {
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    installation_generation: WorthQueryDomainInstallationGeneration,
    domain_authority: Arc<WorthQueryInstalledDomainAuthority>,
    binding_identity: String,
    capability_identity: u64,
    capability_generation: WorthQueryBoundCapabilityGeneration,
    basis: AdmittedBasisCapability<L>,
}

impl<L: BasisOperationLane> WorthQueryProjectionLifecycleBasis<L> {
    pub(in crate::domain_installation::operation_execution) const fn capability_generation(
        &self,
    ) -> WorthQueryBoundCapabilityGeneration {
        self.capability_generation
    }

    pub(super) fn from_source<D, O, F, S>(source: &S) -> Self
    where
        S: super::source::WorthQueryProjectionLifecycleSource<D, O, F, L>,
    {
        let bound = source.bound_operation();
        Self {
            runtime_authority: bound.operation().domain_authority().runtime_authority(),
            installation_generation: bound.operation().installation_generation(),
            domain_authority: Arc::clone(bound.operation().domain_authority()),
            binding_identity: bound.binding_identity().to_string(),
            capability_identity: bound.capability_identity(),
            capability_generation: WorthQueryBoundCapabilityGeneration::mint(),
            basis: bound.basis().clone(),
        }
    }

    pub(super) fn binds<D, O, F, S>(&self, source: &S, checks: &mut usize) -> bool
    where
        S: super::source::WorthQueryProjectionLifecycleSource<D, O, F, L>,
    {
        let bound = source.bound_operation();
        exact(
            checks,
            self.runtime_authority == bound.operation().domain_authority().runtime_authority(),
        ) && exact(
            checks,
            self.installation_generation == bound.operation().installation_generation(),
        ) && exact(
            checks,
            Arc::ptr_eq(&self.domain_authority, bound.operation().domain_authority()),
        ) && exact(checks, self.binding_identity == bound.binding_identity())
            && exact(
                checks,
                self.capability_identity == bound.capability_identity(),
            )
            && exact(checks, &self.basis == bound.basis())
    }
}

fn exact(checks: &mut usize, matches: bool) -> bool {
    *checks += 1;
    matches
}
