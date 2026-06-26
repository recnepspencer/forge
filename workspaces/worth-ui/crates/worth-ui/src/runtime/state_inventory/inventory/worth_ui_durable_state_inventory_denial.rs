use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateInventoryCounters, WorthUiStateOwnerIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDurableStateInventoryDenial {
    AmbiguousNodeReplacementPlan {
        counters: WorthUiDurableStateInventoryCounters,
    },
    MissingOwnerIdentity {
        family_id: WorthUiDurableStateFamilyId,
        counters: WorthUiDurableStateInventoryCounters,
    },
    MissingReplacementPolicy {
        family_id: WorthUiDurableStateFamilyId,
        counters: WorthUiDurableStateInventoryCounters,
    },
    MissingPersistencePosture {
        family_id: WorthUiDurableStateFamilyId,
        counters: WorthUiDurableStateInventoryCounters,
    },
    ReservedPlatformStateFamily {
        family_id: WorthUiDurableStateFamilyId,
        counters: WorthUiDurableStateInventoryCounters,
    },
    MissingPlatformStateFamily {
        family_id: WorthUiDurableStateFamilyId,
        counters: WorthUiDurableStateInventoryCounters,
    },
    InvalidCustomStateFamilyId {
        family_id: WorthUiDurableStateFamilyId,
        counters: WorthUiDurableStateInventoryCounters,
    },
    InvalidOwnerIdentity {
        family_id: WorthUiDurableStateFamilyId,
        owner_identity: WorthUiStateOwnerIdentity,
        counters: WorthUiDurableStateInventoryCounters,
    },
    ReservedPlatformOwnerIdentity {
        family_id: WorthUiDurableStateFamilyId,
        owner_identity: WorthUiStateOwnerIdentity,
        counters: WorthUiDurableStateInventoryCounters,
    },
    DomainTruthStateFamily {
        family_id: WorthUiDurableStateFamilyId,
        owner_identity: WorthUiStateOwnerIdentity,
        counters: WorthUiDurableStateInventoryCounters,
    },
    DuplicateStateFamily {
        family_id: WorthUiDurableStateFamilyId,
        counters: WorthUiDurableStateInventoryCounters,
    },
}
