use std::collections::HashSet;

use worth_proof::NonEmpty;

use super::PhysicalDurabilityGroupBasis;
use crate::physical_runtime::{
    DataSettledPhysicalMutation, PhysicalMutationIdempotencyKeyIdentity, PhysicalMutationIdentity,
    PhysicalWalMemberIdentity,
};

pub struct DataSettledPhysicalMutationMembers {
    basis: PhysicalDurabilityGroupBasis,
    members: NonEmpty<DataSettledPhysicalMutation>,
}

pub struct RejectedDataSettledPhysicalMutationMembers {
    members: NonEmpty<DataSettledPhysicalMutation>,
    cause: PhysicalDataSettledGroupDenial,
}

pub type PhysicalDataSettledGroupAdmissionOutcome =
    Result<DataSettledPhysicalMutationMembers, RejectedDataSettledPhysicalMutationMembers>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDataSettledGroupDenial {
    AuthorityReleased,
    ForeignStore,
    StaleRuntime,
    MemberCountMismatch,
    GroupIdentityMismatch,
    MembershipDigestMismatch,
    MemberOrdinalMismatch,
    MemberMutationMismatch,
    DuplicateMutationIdentity,
    DuplicateMemberIdentity,
    DuplicateIdempotencyIdentity,
}

impl DataSettledPhysicalMutationMembers {
    pub(in crate::physical_runtime) fn admit(
        basis: PhysicalDurabilityGroupBasis,
        members: NonEmpty<DataSettledPhysicalMutation>,
    ) -> PhysicalDataSettledGroupAdmissionOutcome {
        if members.len() != basis.member_count().get() as usize {
            return Err(rejected(
                members,
                PhysicalDataSettledGroupDenial::MemberCountMismatch,
            ));
        }
        let mut mutations = HashSet::<PhysicalMutationIdentity>::new();
        let mut wal_members = HashSet::<PhysicalWalMemberIdentity>::new();
        let mut idempotency = HashSet::<PhysicalMutationIdempotencyKeyIdentity>::new();
        for (index, member) in members.as_slice().iter().enumerate() {
            let binding = member.group_binding();
            if binding.group_identity() != basis.identity() {
                return Err(rejected(
                    members,
                    PhysicalDataSettledGroupDenial::GroupIdentityMismatch,
                ));
            }
            if binding.membership_digest() != basis.membership_digest() {
                return Err(rejected(
                    members,
                    PhysicalDataSettledGroupDenial::MembershipDigestMismatch,
                ));
            }
            if binding.member_count() != basis.member_count()
                || binding.ordinal().get() != index as u32 + 1
            {
                return Err(rejected(
                    members,
                    PhysicalDataSettledGroupDenial::MemberOrdinalMismatch,
                ));
            }
            if binding.member_identity() != member.wal_member_identity() {
                return Err(rejected(
                    members,
                    PhysicalDataSettledGroupDenial::MemberMutationMismatch,
                ));
            }
            if !mutations.insert(member.mutation_identity()) {
                return Err(rejected(
                    members,
                    PhysicalDataSettledGroupDenial::DuplicateMutationIdentity,
                ));
            }
            if !wal_members.insert(binding.member_identity()) {
                return Err(rejected(
                    members,
                    PhysicalDataSettledGroupDenial::DuplicateMemberIdentity,
                ));
            }
            if !idempotency.insert(member.idempotency_identity()) {
                return Err(rejected(
                    members,
                    PhysicalDataSettledGroupDenial::DuplicateIdempotencyIdentity,
                ));
            }
        }
        Ok(Self { basis, members })
    }

    pub const fn basis(&self) -> PhysicalDurabilityGroupBasis {
        self.basis
    }

    pub fn members(&self) -> &[DataSettledPhysicalMutation] {
        self.members.as_slice()
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        PhysicalDurabilityGroupBasis,
        NonEmpty<DataSettledPhysicalMutation>,
    ) {
        (self.basis, self.members)
    }
}

#[cfg_attr(not(feature = "certification-test-authority"), allow(dead_code))]
impl RejectedDataSettledPhysicalMutationMembers {
    pub(in crate::physical_runtime) fn runtime_released(
        members: NonEmpty<DataSettledPhysicalMutation>,
    ) -> Self {
        rejected(members, PhysicalDataSettledGroupDenial::AuthorityReleased)
    }

    pub(in crate::physical_runtime) fn foreign_store(
        members: NonEmpty<DataSettledPhysicalMutation>,
    ) -> Self {
        rejected(members, PhysicalDataSettledGroupDenial::ForeignStore)
    }

    pub(in crate::physical_runtime) fn stale_runtime(
        members: NonEmpty<DataSettledPhysicalMutation>,
    ) -> Self {
        rejected(members, PhysicalDataSettledGroupDenial::StaleRuntime)
    }

    pub const fn cause(&self) -> PhysicalDataSettledGroupDenial {
        self.cause
    }

    pub fn into_members(self) -> NonEmpty<DataSettledPhysicalMutation> {
        self.members
    }
}

fn rejected(
    members: NonEmpty<DataSettledPhysicalMutation>,
    cause: PhysicalDataSettledGroupDenial,
) -> RejectedDataSettledPhysicalMutationMembers {
    RejectedDataSettledPhysicalMutationMembers { members, cause }
}
