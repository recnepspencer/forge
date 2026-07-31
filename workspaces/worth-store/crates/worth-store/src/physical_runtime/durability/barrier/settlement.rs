use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::{
    ArtifactTreePublicationEffect, BackendTargetProfile, MediaOperationIdentity,
    PhysicalDurabilityAdmissionIdentity, WalDurabilityBarrier,
};

use super::PhysicalWalBarrierDeclaration;
use crate::physical_runtime::{
    CompletedPhysicalWalBarrier, PhysicalDurabilityPolicyIdentity, PhysicalEffectIdentity,
    PhysicalWalMemberBasis, PhysicalWorkIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalBarrierSettlement {
    work: PhysicalWorkIdentity,
    effect: PhysicalEffectIdentity,
    member: PhysicalWalMemberBasis,
    policy: PhysicalDurabilityPolicyIdentity,
    admission_basis: PhysicalDurabilityAdmissionIdentity,
    profile: BackendTargetProfile,
    barrier: WalDurabilityBarrier,
    binding_digest: [u8; 32],
}

pub(in crate::physical_runtime) struct CompletionBoundPhysicalWalBarrierSettlement(
    PhysicalWalBarrierSettlement,
);

impl PhysicalWalBarrierSettlement {
    pub(in crate::physical_runtime) fn bind_completed(
        work: PhysicalWorkIdentity,
        physical: &CompletedPhysicalWalBarrier,
        scheduler: &QueueExecutionOutcome,
        declaration: &PhysicalWalBarrierDeclaration,
        expected_work: PhysicalWorkIdentity,
        expected_binding_digest: [u8; 32],
    ) -> Option<CompletionBoundPhysicalWalBarrierSettlement> {
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
        let settlement = Self {
            work,
            effect: PhysicalEffectIdentity::new(work, physical.physical().operation()),
            member: declaration.member_basis(),
            policy: declaration.policy_identity(),
            admission_basis: declaration.admission_basis_identity(),
            profile: declaration.profile(),
            barrier: declaration.required_barrier(),
            binding_digest: declaration.binding_digest(),
        };
        Some(CompletionBoundPhysicalWalBarrierSettlement(settlement))
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

    pub const fn member_basis(self) -> PhysicalWalMemberBasis {
        self.member
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

impl CompletionBoundPhysicalWalBarrierSettlement {
    pub(in crate::physical_runtime) const fn settlement(&self) -> PhysicalWalBarrierSettlement {
        self.0
    }
}
