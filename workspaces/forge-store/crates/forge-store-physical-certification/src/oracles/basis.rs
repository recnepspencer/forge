use crate::{
    IndependentVerifierObservation, ObservedPhysicalTrace, ObserverKind,
    PhysicalScenarioCanonicalIdentity, PhysicalSimulationPlan, PhysicalSimulationPlanIdentity,
    RecoveryOutcomeObservation, ShortcutRejectionObservation, ShortcutRejectionObservationKind,
};

use super::OracleDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleVerdictBasis {
    scenario_identity: PhysicalScenarioCanonicalIdentity,
    plan_identity: PhysicalSimulationPlanIdentity,
    observer: ObserverKind,
    runtime_trace_present: bool,
    independent_verifier: Option<IndependentVerifierObservation>,
    recovery_outcome: Option<RecoveryOutcomeObservation>,
    shortcut_rejections: Vec<ShortcutRejectionObservation>,
}

impl OracleVerdictBasis {
    pub(crate) fn from_plan_and_trace(
        plan: &PhysicalSimulationPlan,
        trace: &ObservedPhysicalTrace,
    ) -> Result<Self, OracleDenial> {
        if trace.scenario_identity() != plan.scenario_identity()
            || trace.plan_identity() != plan.identity()
        {
            return Err(OracleDenial::PlanTraceIdentityMismatch);
        }
        Ok(Self {
            scenario_identity: plan.scenario_identity().clone(),
            plan_identity: plan.identity().clone(),
            observer: trace.observer(),
            runtime_trace_present: true,
            independent_verifier: trace.independent_verifier().cloned(),
            recovery_outcome: trace.recovery_outcome().cloned(),
            shortcut_rejections: trace.shortcut_rejections().to_vec(),
        })
    }

    pub const fn scenario_identity(&self) -> &PhysicalScenarioCanonicalIdentity {
        &self.scenario_identity
    }

    pub const fn plan_identity(&self) -> &PhysicalSimulationPlanIdentity {
        &self.plan_identity
    }

    pub const fn observer(&self) -> ObserverKind {
        self.observer
    }

    pub const fn runtime_trace_present(&self) -> bool {
        self.runtime_trace_present
    }

    pub fn independent_verifier(&self) -> Option<&IndependentVerifierObservation> {
        self.independent_verifier.as_ref()
    }

    pub fn independent_verifier_present(&self) -> bool {
        self.independent_verifier.is_some()
    }

    pub fn recovery_outcome(&self) -> Option<&RecoveryOutcomeObservation> {
        self.recovery_outcome.as_ref()
    }

    pub fn has_shortcut_rejection(&self, kind: ShortcutRejectionObservationKind) -> bool {
        self.shortcut_rejections
            .iter()
            .any(|observation| observation.kind() == kind)
    }

    pub fn shortcut_rejections(&self) -> &[ShortcutRejectionObservation] {
        &self.shortcut_rejections
    }
}
