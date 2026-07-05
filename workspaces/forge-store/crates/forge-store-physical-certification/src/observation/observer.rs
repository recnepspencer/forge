use super::{
    CheckpointCrashReplayObservation, CheckpointInterlockObservation,
    CompactionInterlockObservation, ObservationDenial, ObservedPhysicalTrace,
    S5CheckpointPublicationCrashLaneOutput, S5CheckpointPublicationScheduledLaneOutput,
    S5CheckpointPublicationShortcutRejectionOutput, S5CompactionMutationObservationSet,
};
use crate::{
    ExecutedPhysicalSimulationObservation, IndependentVerifierObservation, ObserverKind,
    PhysicalSimulationPlan, ProductionBoundaryDriverTrace, RecoveryOutcomeObservation,
    S6IoPressureOracleObservation, ShortcutRejectionObservation,
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
    checkpoint_crash_replay: Option<CheckpointCrashReplayObservation>,
    checkpoint_interlock: Option<CheckpointInterlockObservation>,
    compaction_interlock: Option<CompactionInterlockObservation>,
    compaction_mutations: Option<S5CompactionMutationObservationSet>,
    s6_io_pressure: Option<S6IoPressureOracleObservation>,
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
            checkpoint_crash_replay: None,
            checkpoint_interlock: None,
            compaction_interlock: None,
            compaction_mutations: None,
            s6_io_pressure: None,
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

    pub fn with_compaction_interlock_observation(
        mut self,
        observation: CompactionInterlockObservation,
    ) -> Self {
        self.compaction_interlock = Some(observation);
        self
    }

    pub fn with_checkpoint_interlock_observation(
        mut self,
        observation: CheckpointInterlockObservation,
    ) -> Self {
        self.checkpoint_interlock = Some(observation);
        self
    }

    pub fn with_scheduled_checkpoint_publication_lane(
        mut self,
        output: S5CheckpointPublicationScheduledLaneOutput,
    ) -> Result<Self, ObservationDenial> {
        if output.plan_identity() != self.plan.identity().digest_bytes() {
            return Err(ObservationDenial::CheckpointPublicationLanePlanMismatch);
        }
        self.checkpoint_interlock = Some(output.observation());
        Ok(self)
    }

    pub fn with_scheduled_checkpoint_crash_replay_lane(
        mut self,
        output: S5CheckpointPublicationCrashLaneOutput,
    ) -> Result<Self, ObservationDenial> {
        if output.plan_identity() != self.plan.identity().digest_bytes() {
            return Err(ObservationDenial::CheckpointPublicationLanePlanMismatch);
        }
        self.recovery_outcome = Some(output.recovery_outcome());
        self.checkpoint_crash_replay = Some(output.observation());
        Ok(self)
    }

    pub fn with_scheduled_checkpoint_shortcut_rejection_lane(
        mut self,
        output: S5CheckpointPublicationShortcutRejectionOutput,
    ) -> Result<Self, ObservationDenial> {
        if output.plan_identity() != self.plan.identity().digest_bytes() {
            return Err(ObservationDenial::CheckpointPublicationLanePlanMismatch);
        }
        self = self.with_shortcut_rejection_observation(output.observation());
        Ok(self)
    }

    pub fn with_scheduled_compaction_mutation_lanes(
        mut self,
        observations: S5CompactionMutationObservationSet,
    ) -> Self {
        self.compaction_mutations = Some(observations);
        self
    }

    pub fn with_s6_io_pressure_observation(
        mut self,
        observation: S6IoPressureOracleObservation,
    ) -> Self {
        self.s6_io_pressure = Some(observation);
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
            self.checkpoint_crash_replay,
            self.checkpoint_interlock,
            self.compaction_interlock,
            self.compaction_mutations,
            self.s6_io_pressure,
            self.shortcut_rejections,
        ))
    }
}
