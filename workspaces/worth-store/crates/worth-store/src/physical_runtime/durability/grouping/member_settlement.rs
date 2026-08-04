use worth_proof::NonEmpty;
use worth_store_physical_backend::{
    BackendTargetProfile, MediaOperationIdentity, PhysicalDurabilityAdmissionIdentity,
    WalDurabilityBarrier,
};

use super::wal_barrier::{
    CompletionBoundPhysicalWalGroupBarrierSettlement, PhysicalWalGroupBarrierSettlement,
};
use crate::physical_runtime::{
    PhysicalDurabilityGroupBasis, PhysicalDurabilityGroupMemberBinding,
    PhysicalDurabilityPolicyIdentity, PhysicalEffectIdentity,
    PhysicalGroupBarrierAmplificationObservation, PhysicalMutationIdentity, PhysicalWalMemberBasis,
    PhysicalWorkIdentity, SealedPhysicalDurabilityGroupMembers, WalDurablePhysicalMutation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalBarrierSettlement {
    group: PhysicalWalGroupBarrierSettlement,
    binding: PhysicalDurabilityGroupMemberBinding,
    member: PhysicalWalMemberBasis,
}

pub(in crate::physical_runtime) struct CompletionBoundPhysicalWalBarrierSettlement(
    PhysicalWalBarrierSettlement,
);

pub struct WalDurablePhysicalMutationMembers {
    basis: PhysicalDurabilityGroupBasis,
    barrier: PhysicalWalGroupBarrierSettlement,
    amplification: PhysicalGroupBarrierAmplificationObservation,
    members: NonEmpty<WalDurablePhysicalMutation>,
}

impl WalDurablePhysicalMutationMembers {
    pub(in crate::physical_runtime) fn derive(
        group: SealedPhysicalDurabilityGroupMembers,
        barrier: CompletionBoundPhysicalWalGroupBarrierSettlement,
    ) -> Result<Self, SealedPhysicalDurabilityGroupMembers> {
        let shared = barrier.settlement();
        let basis = group.basis();
        if shared.group_identity() != basis.identity()
            || shared.membership_digest() != basis.membership_digest()
            || shared.member_count() != basis.member_count().get()
        {
            return Err(group);
        }
        let amplification = group.amplification_observation().after_shared_barrier();
        let members = group.into_members().map(|member| {
            let binding = member.binding();
            let member_basis = member.mutation().reserved().member_basis();
            let settlement =
                CompletionBoundPhysicalWalBarrierSettlement(PhysicalWalBarrierSettlement {
                    group: shared,
                    binding,
                    member: member_basis,
                });
            WalDurablePhysicalMutation::new(member, settlement)
        });
        Ok(Self {
            basis,
            barrier: shared,
            amplification,
            members,
        })
    }

    pub const fn basis(&self) -> PhysicalDurabilityGroupBasis {
        self.basis
    }

    pub const fn barrier_settlement(&self) -> PhysicalWalGroupBarrierSettlement {
        self.barrier
    }

    pub const fn amplification_observation(&self) -> PhysicalGroupBarrierAmplificationObservation {
        self.amplification
    }

    pub fn members(&self) -> &[WalDurablePhysicalMutation] {
        self.members.as_slice()
    }

    pub fn into_members(self) -> NonEmpty<WalDurablePhysicalMutation> {
        self.members
    }
}

impl PhysicalWalBarrierSettlement {
    pub const fn work(self) -> PhysicalWorkIdentity {
        self.group.work()
    }

    pub const fn effect(self) -> PhysicalEffectIdentity {
        self.group.effect()
    }

    pub const fn backend_operation(self) -> MediaOperationIdentity {
        self.group.backend_operation()
    }

    pub const fn group_settlement(self) -> PhysicalWalGroupBarrierSettlement {
        self.group
    }

    pub const fn group_binding(self) -> PhysicalDurabilityGroupMemberBinding {
        self.binding
    }

    pub const fn member_basis(self) -> PhysicalWalMemberBasis {
        self.member
    }

    pub const fn mutation_identity(self) -> PhysicalMutationIdentity {
        self.member.mutation_identity()
    }

    pub const fn policy_identity(self) -> PhysicalDurabilityPolicyIdentity {
        self.group.policy_identity()
    }

    pub const fn admission_basis_identity(self) -> PhysicalDurabilityAdmissionIdentity {
        self.group.admission_basis_identity()
    }

    pub const fn profile(self) -> BackendTargetProfile {
        self.group.profile()
    }

    pub const fn barrier(self) -> WalDurabilityBarrier {
        self.group.barrier()
    }

    pub const fn binding_digest(self) -> [u8; 32] {
        self.group.binding_digest()
    }
}

impl CompletionBoundPhysicalWalBarrierSettlement {
    pub(in crate::physical_runtime) const fn settlement(&self) -> PhysicalWalBarrierSettlement {
        self.0
    }
}
