use worth_store_test_support::LargeStorePressureClass;

use crate::{
    BufferPoolScenarioPlan, PhysicalOracleOutcome, PhysicalProofOracleKind,
    PhysicalScenarioObserverKind, PhysicalScenarioPlanIdentity, PhysicalStoryTranscript,
    ScenarioCounterObservation, ScenarioDenialBoundary,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferPoolPressureTranscriptIdentity {
    plan_identity: PhysicalScenarioPlanIdentity,
    pressure_class: LargeStorePressureClass,
    counter_trace: Vec<ScenarioCounterObservation>,
    denial_trace: Vec<ScenarioDenialBoundary>,
    shortcut_trace: Vec<ScenarioDenialBoundary>,
    observer_trace: Vec<PhysicalScenarioObserverKind>,
    oracle_outcomes: Vec<(PhysicalProofOracleKind, PhysicalOracleOutcome)>,
}

impl BufferPoolPressureTranscriptIdentity {
    pub fn from_transcript(
        plan: &BufferPoolScenarioPlan<'_>,
        transcript: &PhysicalStoryTranscript,
    ) -> Self {
        Self {
            plan_identity: transcript.plan_identity().clone(),
            pressure_class: plan.pressure_class(),
            counter_trace: transcript.counter_trace().observed_counters().to_vec(),
            denial_trace: transcript.denial_trace().observed_denials().to_vec(),
            shortcut_trace: transcript.shortcut_trace().forbidden_shortcuts().to_vec(),
            observer_trace: transcript.observer_trace().observed_observers().to_vec(),
            oracle_outcomes: transcript
                .judgments()
                .iter()
                .map(|judgment| (judgment.oracle(), judgment.outcome()))
                .collect(),
        }
    }

    pub const fn plan_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.plan_identity
    }
}
