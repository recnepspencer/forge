use crate::planning::S8AccessPlanSelection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessPlanningFacade;

impl AccessPlanningFacade {
    pub const fn selection(&self) -> S8AccessPlanSelection {
        S8AccessPlanSelection
    }
}

pub const fn access_planning() -> AccessPlanningFacade {
    AccessPlanningFacade
}
