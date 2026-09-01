use std::sync::{Arc, Weak};

use crate::branch::{
    admit_runtime_signal_branch_observation, AdmittedSignalBranchBasis,
    ManagedSignalBranchReference, ManagedSignalBranchReferenceAdmissionDenial,
    SignalBranchBasisDescriptor, SignalBranchBasisObservationDenial,
    SignalBranchBasisReadmissionDenial, SignalBranchRetainedReadmissionDenial,
    SignalBranchRetentionAcquisitionDenial, SignalBranchRetentionLease,
    SignalBranchRetentionOwnerRelationship, SignalBranchRetentionReleaseDenial,
    SignalBranchRetentionReleaseOutcome,
};

use super::{
    SignalOwner, SignalOwnerLifecycleObservation, SignalOwnerServiceCostSnapshot,
    SignalOwnerUnavailable,
};

mod denial_mapping;
mod descriptor_validation;
#[cfg(test)]
mod tests;

use denial_mapping::{
    map_basis_admission_denial, map_managed_observation_admission_denial,
    map_managed_readmission_admission_denial, map_observation_readmission_denial,
    map_observation_retention_denial, map_readmission_retention_denial,
    map_release_admission_denial, map_retained_admission_denial, map_retained_retention_denial,
    map_retention_admission_denial,
};
use descriptor_validation::{
    compare_descriptor_with_observation, validate_managed_descriptor, validate_retained_descriptor,
};

/// Concrete weak service for exact Signal basis observation and retention.
pub struct SignalBranchBasisPort<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    owner: Weak<SignalOwner<D, I, T>>,
    diagnostic_owner_runtime_instance_id: u64,
}

impl<D, I, T> Clone for SignalBranchBasisPort<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.clone(),
            diagnostic_owner_runtime_instance_id: self.diagnostic_owner_runtime_instance_id,
        }
    }
}

impl<D, I, T> SignalBranchBasisPort<D, I, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(crate) fn new(
        owner: Weak<SignalOwner<D, I, T>>,
        diagnostic_owner_runtime_instance_id: u64,
    ) -> Self {
        Self {
            owner,
            diagnostic_owner_runtime_instance_id,
        }
    }

    pub(crate) fn diagnostic_owner_runtime_instance_id(&self) -> u64 {
        self.diagnostic_owner_runtime_instance_id
    }

    pub fn issue_managed_branch_reference(
        &self,
        basis: &AdmittedSignalBranchBasis,
    ) -> Result<ManagedSignalBranchReference, ManagedSignalBranchReferenceAdmissionDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(ManagedSignalBranchReferenceAdmissionDenial::OwnerUnavailable)?;
        owner.issue_managed_branch_reference(basis)
    }

    pub fn observe_current(
        &self,
        reference: &ManagedSignalBranchReference,
    ) -> Result<AdmittedSignalBranchBasis, SignalBranchBasisObservationDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchBasisObservationDenial::OwnerUnavailable)?;
        let branch_id = reference.branch_id();
        let (admission, cell) = owner
            .admit_managed_branch_reference(reference)
            .map_err(map_managed_observation_admission_denial)?;
        let retention = owner
            .acquire_admitted_retention(&admission, branch_id)
            .map_err(|denial| map_observation_retention_denial(denial, branch_id))?;
        let observation = cell.observe_exact(&admission)?;
        Ok(admit_runtime_signal_branch_observation(
            observation,
            branch_id,
            retention,
        ))
    }

    pub fn readmit_exact(
        &self,
        reference: &ManagedSignalBranchReference,
        descriptor: &SignalBranchBasisDescriptor,
    ) -> Result<AdmittedSignalBranchBasis, SignalBranchBasisReadmissionDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchBasisReadmissionDenial::OwnerUnavailable)?;
        let branch_id = reference.branch_id();
        let (admission, cell) = owner
            .admit_managed_branch_reference(reference)
            .map_err(map_managed_readmission_admission_denial)?;
        validate_managed_descriptor(&owner, descriptor, branch_id)?;
        let retention = owner
            .acquire_admitted_retention(&admission, branch_id)
            .map_err(|denial| map_readmission_retention_denial(denial, branch_id))?;
        let observation = cell
            .observe_exact(&admission)
            .map_err(|denial| map_observation_readmission_denial(denial, branch_id))?;
        compare_descriptor_with_observation(descriptor, &observation)?;
        Ok(admit_runtime_signal_branch_observation(
            observation,
            branch_id,
            retention,
        ))
    }

    pub fn compare_current_exact(
        &self,
        basis: &AdmittedSignalBranchBasis,
    ) -> Result<(), SignalBranchBasisReadmissionDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchBasisReadmissionDenial::OwnerUnavailable)?;
        let branch_id = basis.owner_branch_id();
        validate_managed_descriptor(&owner, basis.descriptor(), branch_id)?;
        let admission = owner.admit().map_err(map_basis_admission_denial)?;
        let observation = owner
            .observe_branch_exact(&admission, branch_id)
            .map_err(|denial| map_observation_readmission_denial(denial, branch_id))?;
        compare_descriptor_with_observation(basis.descriptor(), &observation)
    }

    pub fn readmit_retained_exact(
        &self,
        descriptor: &SignalBranchBasisDescriptor,
        lease: &SignalBranchRetentionLease,
    ) -> Result<AdmittedSignalBranchBasis, SignalBranchRetainedReadmissionDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchRetainedReadmissionDenial::OwnerUnavailable)?;
        match lease.owner_relationship(&owner.retention_binding()) {
            SignalBranchRetentionOwnerRelationship::DifferentOwner => {
                return Err(SignalBranchRetainedReadmissionDenial::ForeignRetention)
            }
            SignalBranchRetentionOwnerRelationship::OwnerLost => {
                let _admission = owner.admit().map_err(map_retained_admission_denial)?;
                return Err(SignalBranchRetainedReadmissionDenial::UnavailableRetainedTarget);
            }
            SignalBranchRetentionOwnerRelationship::SameOwner => {}
        }
        let admission = owner.admit().map_err(map_retained_admission_denial)?;
        validate_retained_descriptor(descriptor, lease)?;
        let branch_id = descriptor.branch_id();
        let retention = owner
            .acquire_admitted_retention(&admission, branch_id)
            .map_err(map_retained_retention_denial)?;
        Ok(admit_runtime_signal_branch_observation(
            descriptor.observation().clone(),
            branch_id,
            retention,
        ))
    }

    pub fn retain_exact(
        &self,
        basis: &AdmittedSignalBranchBasis,
    ) -> Result<SignalBranchRetentionLease, SignalBranchRetentionAcquisitionDenial> {
        let owner = self
            .upgrade_owner()
            .map_err(SignalBranchRetentionAcquisitionDenial::OwnerUnavailable)?;
        let admission = owner.admit().map_err(map_retention_admission_denial)?;
        owner.acquire_external_retention(&admission, basis)
    }

    pub fn release_exact(
        &self,
        lease: SignalBranchRetentionLease,
    ) -> SignalBranchRetentionReleaseOutcome {
        let owner = match self.upgrade_owner() {
            Ok(owner) => owner,
            Err(unavailable) => {
                return SignalBranchRetentionReleaseOutcome::Denied {
                    lease,
                    denial: SignalBranchRetentionReleaseDenial::OwnerUnavailable(unavailable),
                }
            }
        };
        match lease.owner_relationship(&owner.retention_binding()) {
            SignalBranchRetentionOwnerRelationship::DifferentOwner => {
                SignalBranchRetentionReleaseOutcome::Denied {
                    lease,
                    denial: SignalBranchRetentionReleaseDenial::ForeignRuntime,
                }
            }
            SignalBranchRetentionOwnerRelationship::OwnerLost => match owner.admit() {
                Ok(_admission) => SignalBranchRetentionReleaseOutcome::Denied {
                    lease,
                    denial: SignalBranchRetentionReleaseDenial::OwnerUnavailable(
                        SignalOwnerUnavailable,
                    ),
                },
                Err(denial) => SignalBranchRetentionReleaseOutcome::Denied {
                    lease,
                    denial: map_release_admission_denial(denial),
                },
            },
            SignalBranchRetentionOwnerRelationship::SameOwner => match owner.admit() {
                Ok(_admission) => SignalBranchRetentionReleaseOutcome::Released(lease.release()),
                Err(denial) => SignalBranchRetentionReleaseOutcome::Denied {
                    lease,
                    denial: map_release_admission_denial(denial),
                },
            },
        }
    }

    pub fn owner_lifecycle_observation(&self) -> SignalOwnerLifecycleObservation {
        self.upgrade_owner()
            .map_or(SignalOwnerLifecycleObservation::Closed, |owner| {
                owner.lifecycle_observation()
            })
    }

    pub fn owner_service_cost_snapshot(
        &self,
    ) -> Result<SignalOwnerServiceCostSnapshot, SignalOwnerUnavailable> {
        let owner = self.upgrade_owner()?;
        match owner.lifecycle_observation() {
            SignalOwnerLifecycleObservation::Open => Ok(owner.cost_snapshot()),
            SignalOwnerLifecycleObservation::Closing | SignalOwnerLifecycleObservation::Closed => {
                Err(SignalOwnerUnavailable)
            }
        }
    }

    pub(super) fn upgrade_owner(
        &self,
    ) -> Result<Arc<SignalOwner<D, I, T>>, SignalOwnerUnavailable> {
        SignalOwner::upgrade(&self.owner)
    }
}
