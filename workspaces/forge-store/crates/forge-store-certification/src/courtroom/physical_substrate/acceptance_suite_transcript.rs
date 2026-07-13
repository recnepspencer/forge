use crate::{
    PhysicalCounterExpectationKind, PhysicalOracleOutcome, PhysicalProofOracleKind,
    PhysicalScenarioDriverKind, PhysicalScenarioObserverKind, PhysicalScenarioPlan,
    PhysicalStoryTranscript,
};
use forge_store_test_support::LargeStorePressureClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S2AcceptanceSuiteKind {
    BoundedMemoryCloseout,
    LargeStorePressure,
    BackgroundEnvelopeHonesty,
    SyntheticTestRejection,
    FoundationalBoundaryEvidence,
    S3ReadinessHandoff,
}

impl S2AcceptanceSuiteKind {
    pub const ALL: [Self; 6] = [
        Self::BoundedMemoryCloseout,
        Self::LargeStorePressure,
        Self::BackgroundEnvelopeHonesty,
        Self::SyntheticTestRejection,
        Self::FoundationalBoundaryEvidence,
        Self::S3ReadinessHandoff,
    ];
}

pub(crate) fn transcript_supports_acceptance_suite(
    suite: S2AcceptanceSuiteKind,
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> bool {
    match suite {
        S2AcceptanceSuiteKind::BoundedMemoryCloseout => covers_bounded_closeout(plan, transcript),
        S2AcceptanceSuiteKind::LargeStorePressure => covers_large_store_pressure(plan, transcript),
        S2AcceptanceSuiteKind::BackgroundEnvelopeHonesty => {
            covers_background_envelopes(plan, transcript)
        }
        S2AcceptanceSuiteKind::SyntheticTestRejection => {
            covers_synthetic_rejection(plan, transcript)
        }
        S2AcceptanceSuiteKind::FoundationalBoundaryEvidence => {
            covers_foundational_boundary(plan, transcript)
        }
        S2AcceptanceSuiteKind::S3ReadinessHandoff => {
            covers_physical_integrity_readiness_handoff(plan, transcript)
        }
    }
}

fn covers_large_store_pressure(
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> bool {
    plan.large_store_pressure_fixture().is_some()
        && has_satisfied_oracle(
            transcript,
            PhysicalProofOracleKind::LargeStorePressureBounded,
        )
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::ResidentBytesPeak,
        )
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::AllocationBytesPeak,
        )
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
        )
}

fn covers_bounded_closeout(
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> bool {
    plan.large_store_pressure_fixture().is_some()
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::ResidentBytesPeak,
        )
        && has_expected_counter(transcript, PhysicalCounterExpectationKind::PinnedPagesPeak)
        && has_expected_counter(transcript, PhysicalCounterExpectationKind::DirtyPagesPeak)
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::AllocationBytesPeak,
        )
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::CopiedPayloadBytes,
        )
}

fn covers_background_envelopes(
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> bool {
    plan.large_store_pressure_fixture().is_some()
        && has_driver(plan, PhysicalScenarioDriverKind::MemoryPressureDriver)
        && has_observer(transcript, PhysicalScenarioObserverKind::AllocationEnvelope)
        && has_observer(transcript, PhysicalScenarioObserverKind::Materialization)
        && has_satisfied_oracle(
            transcript,
            PhysicalProofOracleKind::OomAvoidanceBeforeMaterialization,
        )
        && observed_zero(
            transcript,
            PhysicalCounterExpectationKind::UnboundedAllocationAttempts,
        )
        && observed_zero(
            transcript,
            PhysicalCounterExpectationKind::DomainObjectConstructions,
        )
}

fn covers_synthetic_rejection(
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> bool {
    plan.large_store_pressure_fixture().is_some()
        && has_observer(
            transcript,
            PhysicalScenarioObserverKind::MaterializationShortcut,
        )
        && has_satisfied_oracle(
            transcript,
            PhysicalProofOracleKind::ShortcutCertificationRejected,
        )
}

fn covers_foundational_boundary(
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> bool {
    plan.large_store_pressure_fixture().is_some()
        && has_observer(transcript, PhysicalScenarioObserverKind::CounterBundle)
        && has_satisfied_oracle(
            transcript,
            PhysicalProofOracleKind::TranscriptPreservesEvidence,
        )
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::PressureFixtureStoreBytes,
        )
        && has_expected_counter(
            transcript,
            PhysicalCounterExpectationKind::PressureFixtureResidentBudgetBytes,
        )
}

fn covers_physical_integrity_readiness_handoff(
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> bool {
    matches!(
        plan.large_store_pressure_fixture()
            .map(|fixture| fixture.class()),
        Some(LargeStorePressureClass::ProtectedPressure)
    ) && has_observer(transcript, PhysicalScenarioObserverKind::ResidentBudget)
        && has_satisfied_oracle(
            transcript,
            PhysicalProofOracleKind::OomAvoidanceBeforeMaterialization,
        )
        && has_expected_counter(transcript, PhysicalCounterExpectationKind::PinnedPagesPeak)
        && observed_zero(
            transcript,
            PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
        )
}

fn has_driver(plan: &PhysicalScenarioPlan, driver: PhysicalScenarioDriverKind) -> bool {
    plan.driver_requirements()
        .iter()
        .any(|requirement| requirement.kind() == driver)
}

fn has_observer(
    transcript: &PhysicalStoryTranscript,
    observer: PhysicalScenarioObserverKind,
) -> bool {
    transcript
        .observer_trace()
        .observed_observers()
        .contains(&observer)
}

fn has_satisfied_oracle(
    transcript: &PhysicalStoryTranscript,
    oracle: PhysicalProofOracleKind,
) -> bool {
    transcript.judgments().iter().any(|judgment| {
        judgment.oracle() == oracle && judgment.outcome() == PhysicalOracleOutcome::Satisfied
    })
}

fn has_expected_counter(
    transcript: &PhysicalStoryTranscript,
    counter: PhysicalCounterExpectationKind,
) -> bool {
    transcript.counter_trace().is_expected(counter)
}

fn observed_zero(
    transcript: &PhysicalStoryTranscript,
    counter: PhysicalCounterExpectationKind,
) -> bool {
    transcript.counter_trace().observed_value(counter) == Some(0)
}
