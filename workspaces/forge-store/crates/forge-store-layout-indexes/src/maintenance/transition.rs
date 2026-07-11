use crate::production_transition::define_owner_outcome;

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

type LaggedMutationPlan = (S8LayoutMutationPlan, S8IndexLagWitness);

define_owner_outcome!(
    pub S8LayoutMutationAdmissionOutcome,
    pub S8LayoutMutationAdmissionView,
    S8LayoutMutationAdmissionCase,
    LiveMaintenanceAdmissionAndLowering,
    AdmitLiveMaintenance,
    [
        ready => Ready(S8LayoutMutationPlan): Declared => Admit => Admitted,
        lagged => Lagged(LaggedMutationPlan): Declared => Admit => MaintenanceAdmittedLagged,
        deferred => Deferred(LaggedMutationPlan): Declared => Defer => MaintenanceAdmittedDeferred,
        denied => Denied(S8IndexMaintenanceFailureOutcome): Declared => Deny => Denied
    ]
);

impl S8LayoutMutationAdmissionOutcome {
    pub fn into_lagged(self) -> Result<LaggedMutationPlan, Self> {
        match self.into_owner_payload() {
            S8LayoutMutationAdmissionCase::Lagged(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub fn into_deferred(self) -> Result<LaggedMutationPlan, Self> {
        match self.into_owner_payload() {
            S8LayoutMutationAdmissionCase::Deferred(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}

define_owner_outcome!(
    pub S8IndexMaintenanceTransitionOutcome,
    pub S8IndexMaintenanceTransitionView,
    S8IndexMaintenanceTransitionCase,
    LiveMaintenanceAdmissionAndLowering,
    LowerLiveMaintenance,
    [
        ready_exact => ReadyExact(S8LoweredMaintenanceProtocol): Admitted => LowerReady => MaintenanceReady,
        lagged => Lagged(S8LoweredMaintenanceProtocol): MaintenanceAdmittedLagged => LowerLagged => MaintenanceLagged,
        rebuild_only => RebuildOnly(S8LoweredMaintenanceProtocol): MaintenanceAdmittedDeferred => LowerDeferred => MaintenanceRebuildOnly,
        advisory_only => AdvisoryOnly(S8LoweredMaintenanceProtocol): MaintenanceAdmittedDeferred => LowerDeferred => MaintenanceAdvisoryOnly,
        verifier_only => VerifierOnly(S8LoweredMaintenanceProtocol): MaintenanceAdmittedDeferred => LowerDeferred => MaintenanceVerifierOnly,
        migration_only => MigrationOnly(S8LoweredMaintenanceProtocol): MaintenanceAdmittedDeferred => LowerDeferred => MaintenanceMigrationOnly,
        deferred => Deferred(S8LoweredMaintenanceProtocol): MaintenanceAdmittedDeferred => LowerDeferred => MaintenanceDeferred
    ]
);

impl S8IndexMaintenanceTransitionOutcome {
    pub fn into_lagged(self) -> Result<S8LoweredMaintenanceProtocol, Self> {
        match self.into_owner_payload() {
            S8IndexMaintenanceTransitionCase::Lagged(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub fn into_verifier_only(self) -> Result<S8LoweredMaintenanceProtocol, Self> {
        match self.into_owner_payload() {
            S8IndexMaintenanceTransitionCase::VerifierOnly(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}
