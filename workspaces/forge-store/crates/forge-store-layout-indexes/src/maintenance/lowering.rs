use super::{
    layout_rebuild, ExactMaintenancePlan, ExactMaintenanceProtocol, LaggedMaintenancePlan,
    LaggedMaintenanceProtocol, LayoutMaintenanceFacade, LayoutRebuildFacade,
    VerifierMaintenancePlan, VerifierMaintenanceProtocol,
};

impl LayoutMaintenanceFacade {
    pub fn lower_exact(&self, plan: ExactMaintenancePlan) -> ExactMaintenanceProtocol {
        ExactMaintenanceProtocol::issue(plan.observation().clone())
    }

    pub fn lower_lagged(&self, plan: LaggedMaintenancePlan) -> LaggedMaintenanceProtocol {
        LaggedMaintenanceProtocol::issue(plan.observation().clone())
    }

    pub fn lower_verifier(&self, plan: VerifierMaintenancePlan) -> VerifierMaintenanceProtocol {
        VerifierMaintenanceProtocol::issue(plan.observation().clone())
    }

    pub const fn rebuild(&self) -> LayoutRebuildFacade {
        layout_rebuild()
    }
}
