use crate::runtime::{WorthUiOrdinaryLaneNode, WorthUiRuntimeHandle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneTouchReceipt {
    storage: WorthUiOrdinaryLaneTouchStorage,
    touch_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOrdinaryTouchBreadth {
    Direct,
    Subtree,
    RootShell,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiOrdinaryLaneTouchStorage {
    RootShell {
        roots: crate::runtime::plan_topology::WorthUiPlanRegionSlotSetView<1>,
        row_count: usize,
    },
    Direct {
        plan_index: u32,
        runtime_handle: WorthUiRuntimeHandle,
    },
    Subtree {
        plan_index: u32,
        runtime_handle: WorthUiRuntimeHandle,
        row_count: usize,
    },
}

impl WorthUiOrdinaryLaneTouchReceipt {
    pub(crate) fn root_shell(
        touches: crate::runtime::plan_topology::WorthUiPlanRegionSlotSetView<1>,
        row_count: usize,
        touch_digest: u64,
    ) -> Self {
        Self {
            storage: WorthUiOrdinaryLaneTouchStorage::RootShell {
                roots: touches,
                row_count,
            },
            touch_digest,
        }
    }

    pub(crate) fn single(row: &WorthUiOrdinaryLaneNode) -> Self {
        let runtime_handle = row.runtime_handle();
        Self {
            storage: WorthUiOrdinaryLaneTouchStorage::Direct {
                plan_index: row.plan_index(),
                runtime_handle,
            },
            touch_digest: fold_touch(0x6f72_6469_6e61_7279, runtime_handle),
        }
    }

    pub(crate) fn subtree(
        row: &WorthUiOrdinaryLaneNode,
        row_count: usize,
        touch_digest: u64,
    ) -> Self {
        Self {
            storage: WorthUiOrdinaryLaneTouchStorage::Subtree {
                plan_index: row.plan_index(),
                runtime_handle: row.runtime_handle(),
                row_count,
            },
            touch_digest,
        }
    }

    pub fn row_count(&self) -> usize {
        match &self.storage {
            WorthUiOrdinaryLaneTouchStorage::RootShell { row_count, .. }
            | WorthUiOrdinaryLaneTouchStorage::Subtree { row_count, .. } => *row_count,
            WorthUiOrdinaryLaneTouchStorage::Direct { .. } => 1,
        }
    }

    pub fn root_plan_index(&self) -> Option<u32> {
        match &self.storage {
            WorthUiOrdinaryLaneTouchStorage::RootShell { .. } => None,
            WorthUiOrdinaryLaneTouchStorage::Direct { plan_index, .. }
            | WorthUiOrdinaryLaneTouchStorage::Subtree { plan_index, .. } => Some(*plan_index),
        }
    }

    /// Reports the plan roots named by this compact receipt.
    ///
    /// Subtree descendants are proven by `breadth`, `row_count`, and the
    /// touch digest; they are deliberately not materialized into a per-frame
    /// membership collection.
    pub fn names_plan_index(&self, plan_index: u32) -> bool {
        match &self.storage {
            WorthUiOrdinaryLaneTouchStorage::RootShell { roots, .. } => {
                roots.contains(u64::from(plan_index))
            }
            WorthUiOrdinaryLaneTouchStorage::Direct {
                plan_index: root, ..
            }
            | WorthUiOrdinaryLaneTouchStorage::Subtree {
                plan_index: root, ..
            } => *root == plan_index,
        }
    }

    pub fn root_count(&self) -> usize {
        match &self.storage {
            WorthUiOrdinaryLaneTouchStorage::RootShell { roots, .. } => roots.len(),
            WorthUiOrdinaryLaneTouchStorage::Direct { .. }
            | WorthUiOrdinaryLaneTouchStorage::Subtree { .. } => 1,
        }
    }

    pub fn breadth(&self) -> WorthUiOrdinaryTouchBreadth {
        match &self.storage {
            WorthUiOrdinaryLaneTouchStorage::RootShell { .. } => {
                WorthUiOrdinaryTouchBreadth::RootShell
            }
            WorthUiOrdinaryLaneTouchStorage::Direct { .. } => WorthUiOrdinaryTouchBreadth::Direct,
            WorthUiOrdinaryLaneTouchStorage::Subtree { .. } => WorthUiOrdinaryTouchBreadth::Subtree,
        }
    }

    pub fn touch_digest(&self) -> u64 {
        self.touch_digest
    }

    pub fn single_runtime_handle(&self) -> Option<WorthUiRuntimeHandle> {
        match &self.storage {
            WorthUiOrdinaryLaneTouchStorage::Direct { runtime_handle, .. } => Some(*runtime_handle),
            WorthUiOrdinaryLaneTouchStorage::RootShell { .. }
            | WorthUiOrdinaryLaneTouchStorage::Subtree { .. } => None,
        }
    }
}

pub(crate) fn fold_touch(seed: u64, handle: WorthUiRuntimeHandle) -> u64 {
    let value = u64::from(handle.plan_index())
        ^ handle.slot_generation().as_u64().rotate_left(23)
        ^ handle.arena_identity().as_u64().rotate_left(41)
        ^ (handle.family() as u64).rotate_left(41);
    (seed ^ value).wrapping_mul(0x100000001b3)
}
