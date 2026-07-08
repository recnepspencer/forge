use super::failure::S8IndexMaintenanceFailureOutcome;
use super::lag::S8IndexLagWitness;
use super::mutation_plan::S8LayoutMutationPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LoweredMaintenanceProtocol {
    plan: S8LayoutMutationPlan,
}

impl S8LoweredMaintenanceProtocol {
    pub(crate) const fn new(plan: S8LayoutMutationPlan) -> Self {
        Self { plan }
    }

    pub const fn plan(self) -> S8LayoutMutationPlan {
        self.plan
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutMutationAdmissionOutcome {
    Ready(S8LayoutMutationPlan),
    Lagged(S8LayoutMutationPlan, S8IndexLagWitness),
    Deferred(S8LayoutMutationPlan, S8IndexLagWitness),
    Denied(S8IndexMaintenanceFailureOutcome),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexMaintenanceTransitionOutcome {
    ReadyExact(S8LoweredMaintenanceProtocol),
    Lagged(S8LoweredMaintenanceProtocol),
    RebuildOnly(S8LoweredMaintenanceProtocol),
    AdvisoryOnly(S8LoweredMaintenanceProtocol),
    VerifierOnly(S8LoweredMaintenanceProtocol),
    MigrationOnly(S8LoweredMaintenanceProtocol),
    Deferred(S8LoweredMaintenanceProtocol),
    Denied(S8IndexMaintenanceFailureOutcome),
}
