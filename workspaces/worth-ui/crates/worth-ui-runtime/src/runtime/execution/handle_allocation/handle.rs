use super::{WorthUiHandleArenaIdentity, WorthUiHandleSlotGeneration};
use crate::runtime::WorthUiPlanNodeInputFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiRuntimeHandleLocator {
    arena_identity: WorthUiHandleArenaIdentity,
    plan_index: u32,
    slot_generation: WorthUiHandleSlotGeneration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiRuntimeHandle {
    family: WorthUiPlanNodeInputFamily,
    locator: WorthUiRuntimeHandleLocator,
}

macro_rules! typed_runtime_handle {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name {
            locator: WorthUiRuntimeHandleLocator,
        }

        impl $name {
            pub fn plan_index(self) -> u32 {
                self.locator.plan_index()
            }

            pub fn slot_generation(self) -> WorthUiHandleSlotGeneration {
                self.locator.slot_generation()
            }

            pub fn arena_identity(self) -> WorthUiHandleArenaIdentity {
                self.locator.arena_identity()
            }

            pub fn locator(self) -> WorthUiRuntimeHandleLocator {
                self.locator
            }

            pub(crate) fn from_runtime_handle(handle: WorthUiRuntimeHandle) -> Self {
                Self {
                    locator: handle.locator(),
                }
            }
        }
    };
}

typed_runtime_handle!(WorthUiComponentHandle);
typed_runtime_handle!(WorthUiCommandHandle);
typed_runtime_handle!(WorthUiTokenHandle);
typed_runtime_handle!(WorthUiChildRangeHandle);
typed_runtime_handle!(WorthUiViewBindingHandle);
typed_runtime_handle!(WorthUiStateSlotHandle);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiLaneHandle {
    locator: WorthUiRuntimeHandleLocator,
}

impl WorthUiLaneHandle {
    pub fn plan_index(self) -> u32 {
        self.locator.plan_index()
    }

    pub fn slot_generation(self) -> WorthUiHandleSlotGeneration {
        self.locator.slot_generation()
    }

    pub fn arena_identity(self) -> WorthUiHandleArenaIdentity {
        self.locator.arena_identity()
    }

    pub fn locator(self) -> WorthUiRuntimeHandleLocator {
        self.locator
    }
}

#[cfg(test)]
macro_rules! typed_handle_test_constructor {
    ($name:ident) => {
        impl $name {
            pub(crate) fn new(
                plan_index: u32,
                slot_generation: WorthUiHandleSlotGeneration,
                arena_identity: WorthUiHandleArenaIdentity,
            ) -> Self {
                Self {
                    locator: WorthUiRuntimeHandleLocator::new(
                        arena_identity,
                        plan_index,
                        slot_generation,
                    ),
                }
            }
        }
    };
}

#[cfg(test)]
typed_handle_test_constructor!(WorthUiComponentHandle);
#[cfg(test)]
typed_handle_test_constructor!(WorthUiChildRangeHandle);
#[cfg(test)]
typed_handle_test_constructor!(WorthUiViewBindingHandle);

impl WorthUiLaneHandle {
    pub(crate) fn from_locator(locator: WorthUiRuntimeHandleLocator) -> Self {
        Self { locator }
    }
}

impl WorthUiRuntimeHandle {
    pub(crate) fn new(
        family: WorthUiPlanNodeInputFamily,
        plan_index: u32,
        slot_generation: WorthUiHandleSlotGeneration,
        arena_identity: WorthUiHandleArenaIdentity,
    ) -> Self {
        Self {
            family,
            locator: WorthUiRuntimeHandleLocator::new(arena_identity, plan_index, slot_generation),
        }
    }

    pub fn family(self) -> WorthUiPlanNodeInputFamily {
        self.family
    }

    pub fn plan_index(self) -> u32 {
        self.locator.plan_index()
    }

    pub fn slot_generation(self) -> WorthUiHandleSlotGeneration {
        self.locator.slot_generation()
    }

    pub fn arena_identity(self) -> WorthUiHandleArenaIdentity {
        self.locator.arena_identity()
    }

    pub fn locator(self) -> WorthUiRuntimeHandleLocator {
        self.locator
    }
}

impl WorthUiRuntimeHandleLocator {
    pub(crate) fn new(
        arena_identity: WorthUiHandleArenaIdentity,
        plan_index: u32,
        slot_generation: WorthUiHandleSlotGeneration,
    ) -> Self {
        Self {
            arena_identity,
            plan_index,
            slot_generation,
        }
    }

    pub fn arena_identity(self) -> WorthUiHandleArenaIdentity {
        self.arena_identity
    }

    pub fn plan_index(self) -> u32 {
        self.plan_index
    }

    pub fn slot_generation(self) -> WorthUiHandleSlotGeneration {
        self.slot_generation
    }
}
