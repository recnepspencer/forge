use crate::runtime::WorthUiHandlePlanGeneration;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiRendererSurfaceHandle {
    plan_index: u32,
    plan_generation: WorthUiHandlePlanGeneration,
}

impl WorthUiRendererSurfaceHandle {
    pub(crate) fn new(plan_index: u32, plan_generation: WorthUiHandlePlanGeneration) -> Self {
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
