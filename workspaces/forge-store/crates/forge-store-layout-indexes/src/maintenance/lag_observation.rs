use super::{
    IndexLagOutcome, LaggedMaintenanceProtocol, LayoutMaintenanceFacade, LayoutMutationPlan,
    VerifierMaintenanceProtocol,
};

impl LayoutMaintenanceFacade {
    fn inspect_plan_lag(&self, lowered: &LayoutMutationPlan) -> IndexLagOutcome {
        match lowered.lag_witness() {
            Some(witness) => IndexLagOutcome::Lagged(witness.clone()),
            None if lowered.maintenance_mode().permits_exact_answers() => IndexLagOutcome::Exact,
            None => IndexLagOutcome::NonExact(lowered.maintenance_mode()),
        }
    }

    pub fn inspect_lagged(&self, lowered: &LaggedMaintenanceProtocol) -> IndexLagOutcome {
        self.inspect_plan_lag(lowered.plan())
    }

    pub fn inspect_verifier(&self, lowered: &VerifierMaintenanceProtocol) -> IndexLagOutcome {
        self.inspect_plan_lag(lowered.plan())
    }
}
