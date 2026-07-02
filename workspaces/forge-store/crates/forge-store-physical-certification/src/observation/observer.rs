use super::{ObservationDenial, ObservedPhysicalTrace};
use crate::{
    ExecutedPhysicalSimulationObservation, IndependentVerifierObservation, ObserverKind,
    PhysicalSimulationPlan, ProductionBoundaryDriverTrace, RecoveryOutcomeObservation,
    ShortcutRejectionObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSimulationObserver {
    kind: ObserverKind,
}

#[derive(Debug, Clone)]
pub struct PhysicalObservationBuilder<'plan> {
    observer: ObserverKind,
    plan: &'plan PhysicalSimulationPlan,
    runtime_trace: Option<ProductionBoundaryDriverTrace>,
    independent_verifier: Option<IndependentVerifierObservation>,
    recovery_outcome: Option<RecoveryOutcomeObservation>,
    shortcut_rejections: Vec<ShortcutRejectionObservation>,
}

impl PhysicalSimulationObserver {
    pub const fn independent_physical_trace() -> Self {
        Self {
            kind: ObserverKind::IndependentPhysicalTrace,
        }
    }

    pub const fn recovery_outcome() -> Self {
        Self {
            kind: ObserverKind::RecoveryOutcomeObserver,
        }
    }

    pub const fn shortcut_rejection() -> Self {
        Self {
            kind: ObserverKind::ShortcutRejectionObserver,
        }
    }

    pub const fn kind(&self) -> ObserverKind {
        self.kind
    }

    pub fn observe_plan<'plan>(
        self,
        plan: &'plan PhysicalSimulationPlan,
    ) -> Result<PhysicalObservationBuilder<'plan>, ObservationDenial> {
        if !plan.observers().contains(self.kind) {
            return Err(ObservationDenial::ObserverNotRequired {
                observer: self.kind,
            });
        }
        Ok(PhysicalObservationBuilder {
            observer: self.kind,
            plan,
            runtime_trace: None,
            independent_verifier: None,
            recovery_outcome: None,
            shortcut_rejections: Vec::new(),
        })
    }

    pub fn observe_executed_plan<'plan>(
        self,
        plan: &'plan PhysicalSimulationPlan,
        execution: &ExecutedPhysicalSimulationObservation,
    ) -> Result<PhysicalObservationBuilder<'plan>, ObservationDenial> {
        if execution.scenario_identity() != plan.scenario_identity()
            || execution.plan_identity() != plan.identity()
        {
            return Err(ObservationDenial::ExecutionReceiptPlanMismatch);
        }
        Ok(self
            .observe_plan(plan)?
            .with_runtime_trace(execution.runtime_trace().clone()))
    }
}

impl<'plan> PhysicalObservationBuilder<'plan> {
    pub fn with_runtime_trace(mut self, trace: ProductionBoundaryDriverTrace) -> Self {
        self.runtime_trace = Some(trace);
        self
    }

    pub fn with_independent_verifier_observation(
        mut self,
        observation: IndependentVerifierObservation,
    ) -> Self {
        self.independent_verifier = Some(observation);
        self
    }

    pub fn with_recovery_outcome_observation(
        mut self,
        observation: RecoveryOutcomeObservation,
    ) -> Self {
        self.recovery_outcome = Some(observation);
        self
    }

    pub fn with_shortcut_rejection_observation(
        mut self,
        observation: ShortcutRejectionObservation,
    ) -> Self {
        if !self
            .shortcut_rejections
            .iter()
            .any(|candidate| candidate.kind() == observation.kind())
        {
            self.shortcut_rejections.push(observation);
        }
        self
    }

    pub fn complete(self) -> Result<ObservedPhysicalTrace, ObservationDenial> {
        let runtime_trace = self
            .runtime_trace
            .ok_or(ObservationDenial::MissingRuntimeTrace {
                observer: self.observer,
            })?;
        match self.observer {
            ObserverKind::RecoveryOutcomeObserver if self.recovery_outcome.is_none() => {
                return Err(ObservationDenial::MissingRecoveryOutcomeObservation);
            }
            ObserverKind::ShortcutRejectionObserver if self.shortcut_rejections.is_empty() => {
                return Err(ObservationDenial::MissingShortcutRejectionObservation);
            }
            _ => {}
        }
        Ok(ObservedPhysicalTrace::from_parts(
            self.observer,
            self.plan,
            runtime_trace,
            self.independent_verifier,
            self.recovery_outcome,
            self.shortcut_rejections,
        ))
    }
}
