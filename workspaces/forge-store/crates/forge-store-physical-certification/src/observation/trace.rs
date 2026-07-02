use crate::{
    IndependentVerifierObservation, ObserverKind, PhysicalScenarioCanonicalIdentity,
    PhysicalSimulationPlan, PhysicalSimulationPlanIdentity, ProductionBoundaryDriverTrace,
    RecoveryOutcomeObservation, ShortcutRejectionObservation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedPhysicalTrace {
    observer: ObserverKind,
    scenario_identity: PhysicalScenarioCanonicalIdentity,
    plan_identity: PhysicalSimulationPlanIdentity,
    runtime_trace: ProductionBoundaryDriverTrace,
    independent_verifier: Option<IndependentVerifierObservation>,
    recovery_outcome: Option<RecoveryOutcomeObservation>,
    shortcut_rejections: Vec<ShortcutRejectionObservation>,
}

impl ObservedPhysicalTrace {
    pub(crate) fn from_parts(
        observer: ObserverKind,
        plan: &PhysicalSimulationPlan,
        runtime_trace: ProductionBoundaryDriverTrace,
        independent_verifier: Option<IndependentVerifierObservation>,
        recovery_outcome: Option<RecoveryOutcomeObservation>,
        shortcut_rejections: Vec<ShortcutRejectionObservation>,
    ) -> Self {
        Self {
            observer,
            scenario_identity: plan.scenario_identity().clone(),
            plan_identity: plan.identity().clone(),
            runtime_trace,
            independent_verifier,
            recovery_outcome,
            shortcut_rejections,
        }
    }

    pub const fn observer(&self) -> ObserverKind {
        self.observer
    }

    pub const fn scenario_identity(&self) -> &PhysicalScenarioCanonicalIdentity {
        &self.scenario_identity
    }

    pub const fn plan_identity(&self) -> &PhysicalSimulationPlanIdentity {
        &self.plan_identity
    }

    pub const fn runtime_trace(&self) -> &ProductionBoundaryDriverTrace {
        &self.runtime_trace
    }

    pub const fn independent_verifier(&self) -> Option<&IndependentVerifierObservation> {
        self.independent_verifier.as_ref()
    }

    pub const fn recovery_outcome(&self) -> Option<&RecoveryOutcomeObservation> {
        self.recovery_outcome.as_ref()
    }

    pub fn shortcut_rejections(&self) -> &[ShortcutRejectionObservation] {
        &self.shortcut_rejections
    }
}
