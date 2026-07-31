use worth_store_physical_backend::{BackendTargetProfile, PhysicalDurabilityAdmissionIdentity};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::RuntimeIdentity;

use super::{
    AdmittedPhysicalDurabilityPolicy, GroupCommitDelay, GroupCommitLimit, PhysicalCheckpointPolicy,
    PhysicalDurabilityPolicyIdentity, PhysicalIdempotencyPolicy,
};

/// Read-only projection of the durability policy bound to one serving runtime.
///
/// The observation carries identity and admitted limits. It cannot reconstruct
/// the move-owned policy, platform basis, or runtime owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalDurabilityObservation {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    policy: PhysicalDurabilityPolicyIdentity,
    admission_basis: PhysicalDurabilityAdmissionIdentity,
    profile: BackendTargetProfile,
    group_commit_limit: GroupCommitLimit,
    group_commit_delay: GroupCommitDelay,
    idempotency: PhysicalIdempotencyPolicy,
    checkpoint: PhysicalCheckpointPolicy,
}

impl PhysicalDurabilityObservation {
    pub(super) fn new(runtime: RuntimeIdentity, policy: &AdmittedPhysicalDurabilityPolicy) -> Self {
        Self {
            store: policy.store_identity(),
            runtime,
            policy: policy.identity(),
            admission_basis: policy.admission_basis_identity(),
            profile: policy.profile(),
            group_commit_limit: policy.group_commit_limit(),
            group_commit_delay: policy.group_commit_delay(),
            idempotency: policy.idempotency_policy(),
            checkpoint: policy.checkpoint_policy(),
        }
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn runtime_identity(self) -> RuntimeIdentity {
        self.runtime
    }

    pub const fn policy_identity(self) -> PhysicalDurabilityPolicyIdentity {
        self.policy
    }

    pub const fn admission_basis_identity(self) -> PhysicalDurabilityAdmissionIdentity {
        self.admission_basis
    }

    pub const fn profile(self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn group_commit_limit(self) -> GroupCommitLimit {
        self.group_commit_limit
    }

    pub const fn group_commit_delay(self) -> GroupCommitDelay {
        self.group_commit_delay
    }

    pub const fn idempotency_policy(self) -> PhysicalIdempotencyPolicy {
        self.idempotency
    }

    pub const fn checkpoint_policy(self) -> PhysicalCheckpointPolicy {
        self.checkpoint
    }
}
