use super::unique_membership::PhysicalGroupMembershipDigest;
use super::PhysicalDurabilityGroupIdentity;

/// The root work a complete physical durability group will require in Phase 7.
///
/// This is planning truth only. It grants no root mutation or publication
/// authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalGroupRootPublicationPlan {
    IndividualMember,
    Shared(SharedPhysicalRootPublicationPlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedPhysicalRootPublicationPlan {
    group: PhysicalDurabilityGroupIdentity,
    membership: PhysicalGroupMembershipDigest,
}

impl PhysicalGroupRootPublicationPlan {
    pub(super) const fn for_group(
        group: PhysicalDurabilityGroupIdentity,
        membership: PhysicalGroupMembershipDigest,
        member_count: usize,
    ) -> Self {
        if member_count == 1 {
            Self::IndividualMember
        } else {
            Self::Shared(SharedPhysicalRootPublicationPlan { group, membership })
        }
    }
}

impl SharedPhysicalRootPublicationPlan {
    pub const fn group_identity(self) -> PhysicalDurabilityGroupIdentity {
        self.group
    }

    pub const fn membership_digest(self) -> [u8; 32] {
        self.membership.bytes()
    }
}
