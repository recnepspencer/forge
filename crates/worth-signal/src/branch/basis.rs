use std::sync::Arc;

use super::authority::{
    mint_signal_branch_authority, signal_branch_basis_proof, SignalBranchBasisProof,
};
use super::descriptor::SignalBranchBasisDescriptor;
use super::reference::SignalBranchObservation;
use super::retention::SignalBranchAdmissionLease;
use super::retention::{SignalBranchRetentionBinding, SignalBranchRetentionOwnerRelationship};
use crate::state::SignalBranchId;

/// Owner-issued Signal observation token. Construction is private to the
/// Signal branch owner; callers cannot mint one from a descriptor.
#[derive(Debug, Clone)]
pub struct AdmittedSignalBranchBasis(Arc<AdmittedSignalBranchBasisInner>);

#[derive(Debug)]
struct AdmittedSignalBranchBasisInner {
    descriptor: SignalBranchBasisDescriptor,
    admission_identity: super::SignalBranchBasisAdmissionIdentity,
    _proof: SignalBranchBasisProof,
    _retention: SignalBranchAdmissionLease,
}

impl AdmittedSignalBranchBasis {
    /// Identity issued by the Signal owner for this exact admission.
    ///
    /// The identity is descriptive binding for later composition. It is not
    /// a serializable descriptor or proof of currentness.
    pub fn admission_identity(&self) -> &super::SignalBranchBasisAdmissionIdentity {
        &self.0.admission_identity
    }

    pub fn observation(&self) -> &SignalBranchObservation {
        self.0.descriptor.observation()
    }

    pub fn descriptor(&self) -> &SignalBranchBasisDescriptor {
        &self.0.descriptor
    }

    pub fn branch_id(&self) -> SignalBranchId {
        self.0.descriptor.branch_id()
    }

    pub(crate) fn owner_branch_id(&self) -> SignalBranchId {
        self.branch_id()
    }

    pub(crate) fn shared_holder_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    pub(crate) fn owner_identity_relationship(
        &self,
        owner: &SignalBranchRetentionBinding,
    ) -> SignalBranchRetentionOwnerRelationship {
        self.0._retention.owner_identity_relationship(owner)
    }
}

pub(crate) fn admit_signal_branch_observation(
    observation: SignalBranchObservation,
    branch_id: SignalBranchId,
    retention: SignalBranchAdmissionLease,
) -> AdmittedSignalBranchBasis {
    let authority = mint_signal_branch_authority();
    let proof = signal_branch_basis_proof(&authority);
    AdmittedSignalBranchBasis(Arc::new(AdmittedSignalBranchBasisInner {
        descriptor: SignalBranchBasisDescriptor::owner_issued(branch_id, observation),
        admission_identity: super::SignalBranchBasisAdmissionIdentity::issue(),
        _proof: proof,
        _retention: retention,
    }))
}

pub(crate) fn admit_runtime_signal_branch_observation(
    observation: SignalBranchObservation,
    branch_id: SignalBranchId,
    retention: SignalBranchAdmissionLease,
) -> AdmittedSignalBranchBasis {
    admit_signal_branch_observation(observation, branch_id, retention)
}
