use crate::maintenance::S8LayoutMaintenancePublication;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutMaintenanceFacade;

impl LayoutMaintenanceFacade {
    pub const fn publication(&self) -> S8LayoutMaintenancePublication {
        S8LayoutMaintenancePublication
    }
}

pub const fn layout_maintenance() -> LayoutMaintenanceFacade {
    LayoutMaintenanceFacade
}
