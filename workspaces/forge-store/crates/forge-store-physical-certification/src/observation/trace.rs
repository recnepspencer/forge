use super::{
    CheckpointCrashReplayObservation, CheckpointInterlockObservation,
    CompactionInterlockObservation, S5CompactionMutationObservationSet,
};
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
    checkpoint_crash_replay: Option<CheckpointCrashReplayObservation>,
    checkpoint_interlock: Option<CheckpointInterlockObservation>,
    compaction_interlock: Option<CompactionInterlockObservation>,
    compaction_mutations: Option<S5CompactionMutationObservationSet>,
    shortcut_rejections: Vec<ShortcutRejectionObservation>,
}

impl ObservedPhysicalTrace {
    pub(crate) fn from_parts(
        observer: ObserverKind,
        plan: &PhysicalSimulationPlan,
        runtime_trace: ProductionBoundaryDriverTrace,
        independent_verifier: Option<IndependentVerifierObservation>,
        recovery_outcome: Option<RecoveryOutcomeObservation>,
        checkpoint_crash_replay: Option<CheckpointCrashReplayObservation>,
        checkpoint_interlock: Option<CheckpointInterlockObservation>,
        compaction_interlock: Option<CompactionInterlockObservation>,
        compaction_mutations: Option<S5CompactionMutationObservationSet>,
        shortcut_rejections: Vec<ShortcutRejectionObservation>,
    ) -> Self {
        Self {
            observer,
            scenario_identity: plan.scenario_identity().clone(),
            plan_identity: plan.identity().clone(),
            runtime_trace,
            independent_verifier,
            recovery_outcome,
            checkpoint_crash_replay,
            checkpoint_interlock,
            compaction_interlock,
            compaction_mutations,
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

    pub const fn checkpoint_crash_replay(&self) -> Option<&CheckpointCrashReplayObservation> {
        self.checkpoint_crash_replay.as_ref()
    }

    pub const fn checkpoint_interlock(&self) -> Option<CheckpointInterlockObservation> {
        self.checkpoint_interlock
    }

    pub const fn compaction_interlock(&self) -> Option<CompactionInterlockObservation> {
        self.compaction_interlock
    }

    pub const fn compaction_mutations(&self) -> Option<&S5CompactionMutationObservationSet> {
        self.compaction_mutations.as_ref()
    }

    pub fn with_scheduled_compaction_mutation_lanes(
        mut self,
        observations: S5CompactionMutationObservationSet,
    ) -> Self {
        self.compaction_mutations = Some(observations);
        self
    }

    pub fn shortcut_rejections(&self) -> &[ShortcutRejectionObservation] {
        &self.shortcut_rejections
    }
}
