use super::{WorthUiSemanticSliceId, WorthUiSemanticSliceInventory, WorthUiSemanticSliceOwner};

const QUERY_OWNED_SLICE_IDS: &[WorthUiSemanticSliceId] = &[
    WorthUiSemanticSliceId::QueryBindingIdentity,
    WorthUiSemanticSliceId::QueryLiveViewBinding,
    WorthUiSemanticSliceId::QueryBindingPreservationPosture,
    WorthUiSemanticSliceId::QueryBindingRebindPosture,
    WorthUiSemanticSliceId::QueryBindingRetirementPosture,
    WorthUiSemanticSliceId::QueryResultPosture,
    WorthUiSemanticSliceId::QueryProjectionFact,
    WorthUiSemanticSliceId::QueryStateSnapshot,
    WorthUiSemanticSliceId::QueryEffectPosture,
    WorthUiSemanticSliceId::QueryRecoveryPosture,
    WorthUiSemanticSliceId::QueryInspectionTarget,
    WorthUiSemanticSliceId::VirtualizedDataFrameTarget,
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiQueryOwnedSemanticSliceInventory {
    _private: (),
}

impl WorthUiQueryOwnedSemanticSliceInventory {
    pub fn current() -> Self {
        Self { _private: () }
    }

    pub fn slice_ids(&self) -> &'static [WorthUiSemanticSliceId] {
        QUERY_OWNED_SLICE_IDS
    }

    pub fn contains(&self, id: WorthUiSemanticSliceId) -> bool {
        QUERY_OWNED_SLICE_IDS.contains(&id)
    }

    pub fn audit_against_inventory(&self, inventory: &WorthUiSemanticSliceInventory) -> bool {
        self.slice_ids().iter().all(|id| {
            inventory.slice(*id).is_some_and(|descriptor| {
                descriptor.owner() == WorthUiSemanticSliceOwner::QueryAuthority
            })
        })
    }
}
