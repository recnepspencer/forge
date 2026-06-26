use forge_store_test_support::LargeStorePressureClass;

use crate::{
    BufferPoolPressureTranscriptIdentity, BufferPoolScenarioPlan, BufferPoolScenarioPlanDenial,
    PhysicalCounterExpectationKind, PhysicalOracleOutcome, PhysicalProofOracleKind,
    PhysicalScenarioObserverKind, PhysicalStoryTranscript, ScenarioDenialBoundary,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LargeStorePressureEvidenceBundle {
    pressure_class: LargeStorePressureClass,
    transcript_identity: BufferPoolPressureTranscriptIdentity,
}

impl LargeStorePressureEvidenceBundle {
    pub fn from_harness_transcript(
        plan: &BufferPoolScenarioPlan<'_>,
        transcript: &PhysicalStoryTranscript,
    ) -> Result<Self, LargeStorePressureEvidenceDenial> {
        if transcript.plan_identity() != plan.plan_identity() {
            return Err(LargeStorePressureEvidenceDenial::PlanTranscriptMismatch);
        }
        for oracle in required_oracles() {
            let satisfied = transcript.judgments().iter().any(|judgment| {
                judgment.oracle() == oracle
                    && judgment.outcome() == PhysicalOracleOutcome::Satisfied
            });
            if !satisfied {
                return Err(LargeStorePressureEvidenceDenial::MissingSatisfiedOracle(
                    oracle,
                ));
            }
        }
        for counter in required_counters() {
            if !transcript.counter_trace().is_expected(counter) {
                return Err(LargeStorePressureEvidenceDenial::MissingCounter(counter));
            }
        }
        for shortcut in required_shortcut_rejections() {
            if !transcript
                .shortcut_trace()
                .forbidden_shortcuts()
                .contains(&shortcut)
            {
                return Err(LargeStorePressureEvidenceDenial::MissingShortcutRejection(
                    shortcut,
                ));
            }
        }
        for observer in required_observers() {
            if !transcript.observer_trace().contains(observer) {
                return Err(LargeStorePressureEvidenceDenial::MissingObserverTrace(
                    observer,
                ));
            }
        }
        Ok(Self {
            pressure_class: plan.pressure_class(),
            transcript_identity: BufferPoolPressureTranscriptIdentity::from_transcript(
                plan, transcript,
            ),
        })
    }

    pub fn reject_shortcut(
        attempt: LargeStoreShortcutAttempt,
    ) -> Result<Self, LargeStorePressureEvidenceDenial> {
        Self::from_shortcut_attempt(attempt)
    }

    pub fn from_shortcut_attempt(
        attempt: LargeStoreShortcutAttempt,
    ) -> Result<Self, LargeStorePressureEvidenceDenial> {
        Err(LargeStorePressureEvidenceDenial::ShortcutRejected(
            attempt.denial_boundary(),
        ))
    }

    pub const fn pressure_class(&self) -> LargeStorePressureClass {
        self.pressure_class
    }

    pub const fn transcript_identity(&self) -> &BufferPoolPressureTranscriptIdentity {
        &self.transcript_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeStoreShortcutAttempt {
    BypassLoweredPlan,
    BypassObserverTrace,
    TestSupportOwnsMeaning,
}

impl LargeStoreShortcutAttempt {
    pub const fn denial_boundary(self) -> ScenarioDenialBoundary {
        match self {
            Self::BypassLoweredPlan => ScenarioDenialBoundary::BypassedLoweredPlan,
            Self::BypassObserverTrace => ScenarioDenialBoundary::BypassedObserverTrace,
            Self::TestSupportOwnsMeaning => ScenarioDenialBoundary::TestSupportOwnedMeaning,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeStorePressureEvidenceDenial {
    MissingCounter(PhysicalCounterExpectationKind),
    MissingObserverTrace(PhysicalScenarioObserverKind),
    MissingSatisfiedOracle(PhysicalProofOracleKind),
    MissingShortcutRejection(ScenarioDenialBoundary),
    Plan(BufferPoolScenarioPlanDenial),
    PlanTranscriptMismatch,
    ShortcutRejected(ScenarioDenialBoundary),
}

impl From<BufferPoolScenarioPlanDenial> for LargeStorePressureEvidenceDenial {
    fn from(value: BufferPoolScenarioPlanDenial) -> Self {
        Self::Plan(value)
    }
}

fn required_oracles() -> [PhysicalProofOracleKind; 4] {
    [
        PhysicalProofOracleKind::LargeStorePressureBounded,
        PhysicalProofOracleKind::OomAvoidanceBeforeMaterialization,
        PhysicalProofOracleKind::PressureTranscriptReplayStable,
        PhysicalProofOracleKind::ShortcutCertificationRejected,
    ]
}

fn required_counters() -> [PhysicalCounterExpectationKind; 11] {
    [
        PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
        PhysicalCounterExpectationKind::PressureFixtureStoreBytes,
        PhysicalCounterExpectationKind::PressureFixtureResidentBudgetBytes,
        PhysicalCounterExpectationKind::ResidentBytesPeak,
        PhysicalCounterExpectationKind::PinnedPagesPeak,
        PhysicalCounterExpectationKind::DirtyPagesPeak,
        PhysicalCounterExpectationKind::AllocationBytesPeak,
        PhysicalCounterExpectationKind::CopiedPayloadBytes,
        PhysicalCounterExpectationKind::DomainObjectConstructions,
        PhysicalCounterExpectationKind::UnboundedAllocationAttempts,
        PhysicalCounterExpectationKind::DiagnosticMaterializationBytes,
    ]
}

fn required_shortcut_rejections() -> [ScenarioDenialBoundary; 3] {
    [
        ScenarioDenialBoundary::BypassedLoweredPlan,
        ScenarioDenialBoundary::BypassedObserverTrace,
        ScenarioDenialBoundary::TestSupportOwnedMeaning,
    ]
}

fn required_observers() -> [PhysicalScenarioObserverKind; 5] {
    [
        PhysicalScenarioObserverKind::ResidentBudget,
        PhysicalScenarioObserverKind::AllocationEnvelope,
        PhysicalScenarioObserverKind::Materialization,
        PhysicalScenarioObserverKind::MaterializationShortcut,
        PhysicalScenarioObserverKind::CounterBundle,
    ]
}
