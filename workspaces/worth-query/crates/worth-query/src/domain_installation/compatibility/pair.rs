use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;

use super::super::{WorthQueryBoundDomainOperation, WorthQueryInstalledDomainAuthority};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryCompatibilityEndpoint {
    capability_identity: u64,
    binding_identity: String,
    domain_authority: Arc<WorthQueryInstalledDomainAuthority>,
    operation_authority:
        Arc<worth_query_installation::facade::WorthQueryInstalledDomainOperationAuthority>,
}

impl WorthQueryCompatibilityEndpoint {
    pub(super) fn from_bound<D, O, F, L: BasisOperationLane>(
        bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> Self {
        Self {
            capability_identity: bound.capability_identity(),
            binding_identity: bound.binding_identity().to_string(),
            domain_authority: Arc::clone(bound.operation().domain_authority()),
            operation_authority: Arc::clone(bound.operation().operation_authority()),
        }
    }

    fn matches<D, O, F, L: BasisOperationLane>(
        &self,
        bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> bool {
        self.capability_identity == bound.capability_identity()
            && self.binding_identity == bound.binding_identity()
            && Arc::ptr_eq(&self.domain_authority, bound.operation().domain_authority())
            && Arc::ptr_eq(
                &self.operation_authority,
                bound.operation().operation_authority(),
            )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryCompatibilityPairBasis {
    subject: WorthQueryCompatibilityEndpoint,
    candidate: WorthQueryCompatibilityEndpoint,
}

impl WorthQueryCompatibilityPairBasis {
    pub(super) fn from_bounds<D, O, F, L: BasisOperationLane>(
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> Self {
        Self {
            subject: WorthQueryCompatibilityEndpoint::from_bound(subject),
            candidate: WorthQueryCompatibilityEndpoint::from_bound(candidate),
        }
    }

    pub(super) fn matches<D, O, F, L: BasisOperationLane>(
        &self,
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> bool {
        self.subject.matches(subject) && self.candidate.matches(candidate)
    }

    pub(super) fn matches_current_pair<D, O, F, L: BasisOperationLane>(
        &self,
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> bool {
        self.matches(subject, candidate)
            && subject.installation_is_current()
            && candidate.installation_is_current()
    }

    pub(super) fn matches_rebind_pair<D, O, F, L: BasisOperationLane>(
        &self,
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> bool {
        self.matches(subject, candidate)
            && !subject.installation_is_current()
            && candidate.installation_is_current()
    }
}
