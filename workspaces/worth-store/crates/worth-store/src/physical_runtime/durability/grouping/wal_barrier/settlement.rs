use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::{
    ArtifactTreePublicationEffect, BackendTargetProfile, MediaOperationIdentity,
    PhysicalDurabilityAdmissionIdentity, WalDurabilityBarrier,
};

use super::PhysicalWalGroupBarrierDeclaration;
use crate::physical_runtime::{
    CompletedPhysicalWalBarrier, PhysicalDurabilityGroupIdentity, PhysicalDurabilityPolicyIdentity,
    PhysicalEffectIdentity, PhysicalWorkIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalGroupBarrierSettlement {
    work: PhysicalWorkIdentity,
    effect: PhysicalEffectIdentity,
    group: PhysicalDurabilityGroupIdentity,
    membership: [u8; 32],
    member_count: u32,
    policy: PhysicalDurabilityPolicyIdentity,
    admission_basis: PhysicalDurabilityAdmissionIdentity,
    profile: BackendTargetProfile,
    barrier: WalDurabilityBarrier,
    binding_digest: [u8; 32],
}

pub(in crate::physical_runtime) struct CompletionBoundPhysicalWalGroupBarrierSettlement(
    PhysicalWalGroupBarrierSettlement,
);

impl PhysicalWalGroupBarrierSettlement {
    pub(in crate::physical_runtime) fn bind_completed(
        work: PhysicalWorkIdentity,
        physical: &CompletedPhysicalWalBarrier,
        scheduler: &QueueExecutionOutcome,
        declaration: &PhysicalWalGroupBarrierDeclaration,
        expected_work: PhysicalWorkIdentity,
        expected_binding_digest: [u8; 32],
    ) -> Option<CompletionBoundPhysicalWalGroupBarrierSettlement> {
        if work != expected_work
            || declaration.binding_digest() != expected_binding_digest
            || !matches!(scheduler, QueueExecutionOutcome::Executed(_))
            || physical.artifact() != declaration.artifact()
            || !matches!(
                physical.physical().effect(),
                ArtifactTreePublicationEffect::FileSynchronization(artifact)
                    if artifact == declaration.artifact()
            )
        {
            return None;
        }
        let basis = declaration.basis();
        Some(CompletionBoundPhysicalWalGroupBarrierSettlement(Self {
            work,
            effect: PhysicalEffectIdentity::new(work, physical.physical().operation()),
            group: basis.identity(),
            membership: basis.membership_digest(),
            member_count: basis.member_count().get(),
            policy: declaration.policy_identity(),
            admission_basis: declaration.admission_basis_identity(),
            profile: declaration.profile(),
            barrier: declaration.required_barrier(),
            binding_digest: declaration.binding_digest(),
        }))
    }

    pub const fn work(self) -> PhysicalWorkIdentity {
        self.work
    }

    pub const fn effect(self) -> PhysicalEffectIdentity {
        self.effect
    }

    pub const fn backend_operation(self) -> MediaOperationIdentity {
        self.effect.backend_operation()
    }

    pub const fn group_identity(self) -> PhysicalDurabilityGroupIdentity {
        self.group
    }

    pub const fn membership_digest(self) -> [u8; 32] {
        self.membership
    }

    pub const fn member_count(self) -> u32 {
        self.member_count
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

    pub const fn barrier(self) -> WalDurabilityBarrier {
        self.barrier
    }

    pub const fn binding_digest(self) -> [u8; 32] {
        self.binding_digest
    }
}

impl CompletionBoundPhysicalWalGroupBarrierSettlement {
    pub(in crate::physical_runtime) const fn settlement(
        &self,
    ) -> PhysicalWalGroupBarrierSettlement {
        self.0
    }
}
