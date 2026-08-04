use std::num::{NonZeroU32, NonZeroU64};

use super::{PhysicalGroupRootPublicationPlan, SealedPhysicalDurabilityGroupMembers};

/// Amplification facts proven once every member WAL frame has been appended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalGroupAppendAmplificationObservation {
    members: NonZeroU32,
    wal_bytes: NonZeroU64,
    root_plan: PhysicalGroupRootPublicationPlan,
}

/// Amplification facts proven once the shared WAL barrier has completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalGroupBarrierAmplificationObservation {
    append: PhysicalGroupAppendAmplificationObservation,
}

impl PhysicalGroupAppendAmplificationObservation {
    pub(super) fn for_appended_group(group: &SealedPhysicalDurabilityGroupMembers) -> Self {
        let wal_bytes = group
            .members()
            .iter()
            .try_fold(0_u64, |total, member| {
                total.checked_add(member.mutation().settlement().range().byte_count())
            })
            .expect("a sealed group contains representable completed WAL append ranges");
        Self {
            members: group.basis().member_count(),
            wal_bytes: NonZeroU64::new(wal_bytes)
                .expect("every sealed group member has a nonempty WAL append"),
            root_plan: group.basis().root_publication_plan(),
        }
    }

    pub const fn mutations_admitted(self) -> u32 {
        self.members.get()
    }

    pub const fn groups_formed(self) -> u32 {
        1
    }

    pub const fn members_per_group(self) -> u32 {
        self.members.get()
    }

    pub const fn wal_frames_appended(self) -> u32 {
        self.members.get()
    }

    pub const fn wal_bytes_appended(self) -> u64 {
        self.wal_bytes.get()
    }

    pub const fn root_publications_planned(self) -> u32 {
        1
    }

    pub const fn shared_root_publications_planned(self) -> u32 {
        match self.root_plan {
            PhysicalGroupRootPublicationPlan::IndividualMember => 0,
            PhysicalGroupRootPublicationPlan::Shared(_) => 1,
        }
    }

    pub const fn root_publication_plan(self) -> PhysicalGroupRootPublicationPlan {
        self.root_plan
    }

    pub(super) const fn after_shared_barrier(self) -> PhysicalGroupBarrierAmplificationObservation {
        PhysicalGroupBarrierAmplificationObservation { append: self }
    }
}

impl PhysicalGroupBarrierAmplificationObservation {
    pub const fn append(self) -> PhysicalGroupAppendAmplificationObservation {
        self.append
    }

    pub const fn wal_barrier_executions(self) -> u32 {
        1
    }

    pub const fn members_proven_wal_durable(self) -> u32 {
        self.append.members_per_group()
    }
}
