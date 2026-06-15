use crate::runtime::{WorthUiHandlePlanGeneration, WorthUiPlanNodeInputFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiRuntimeHandle {
    family: WorthUiPlanNodeInputFamily,
    plan_index: u32,
    plan_generation: WorthUiHandlePlanGeneration,
}

macro_rules! typed_runtime_handle {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name {
            plan_index: u32,
            plan_generation: WorthUiHandlePlanGeneration,
        }

        impl $name {
            pub(crate) fn new(
                plan_index: u32,
                plan_generation: WorthUiHandlePlanGeneration,
            ) -> Self {
                Self {
                    plan_index,
                    plan_generation,
                }
            }

            pub fn plan_index(self) -> u32 {
                self.plan_index
            }

            pub fn plan_generation(self) -> WorthUiHandlePlanGeneration {
                self.plan_generation
            }
        }
    };
}

typed_runtime_handle!(WorthUiComponentHandle);
typed_runtime_handle!(WorthUiCommandHandle);
typed_runtime_handle!(WorthUiTokenHandle);
typed_runtime_handle!(WorthUiChildRangeHandle);
typed_runtime_handle!(WorthUiViewBindingHandle);
typed_runtime_handle!(WorthUiLaneHandle);
typed_runtime_handle!(WorthUiStateSlotHandle);

impl WorthUiRuntimeHandle {
    pub(crate) fn new(
        family: WorthUiPlanNodeInputFamily,
        plan_index: u32,
        plan_generation: WorthUiHandlePlanGeneration,
    ) -> Self {
        Self {
            family,
            plan_index,
            plan_generation,
        }
    }

    pub fn family(self) -> WorthUiPlanNodeInputFamily {
        self.family
    }

    pub fn plan_index(self) -> u32 {
        self.plan_index
    }

    pub fn plan_generation(self) -> WorthUiHandlePlanGeneration {
        self.plan_generation
    }
}
