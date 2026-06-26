use crate::{PhysicalScenarioPlanIdentity, PhysicalStoryTranscript, ScenarioDenialBoundary};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntheticCloseoutShortcutAttempt {
    LogsOnlyProof,
    SameRunSelfComparison,
    ExpectedErrorsOnly,
    InMemoryOnlyBuffers,
    SmallFixtureOnly,
    FixtureLabelsOnly,
    TestSupportOwnedOracleMeaning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticCloseoutShortcutRejectionReport {
    rejected_attempt: SyntheticCloseoutShortcutAttempt,
    rejected_boundary: ScenarioDenialBoundary,
    transcript_identity: PhysicalScenarioPlanIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticCloseoutShortcutInput {
    attempted_shortcut: SyntheticCloseoutShortcutAttempt,
    attempted_boundary: ScenarioDenialBoundary,
    transcript_identity: PhysicalScenarioPlanIdentity,
}

impl SyntheticCloseoutShortcutInput {
    pub(crate) fn from_transcript(
        attempted_shortcut: SyntheticCloseoutShortcutAttempt,
        transcript: &PhysicalStoryTranscript,
    ) -> Self {
        Self {
            attempted_shortcut,
            attempted_boundary: attempted_shortcut.required_boundary(),
            transcript_identity: transcript.plan_identity().clone(),
        }
    }

    pub const fn attempted_shortcut(&self) -> SyntheticCloseoutShortcutAttempt {
        self.attempted_shortcut
    }

    pub const fn attempted_boundary(&self) -> ScenarioDenialBoundary {
        self.attempted_boundary
    }

    pub const fn transcript_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.transcript_identity
    }
}

impl SyntheticCloseoutShortcutRejectionReport {
    pub(crate) fn attempt_shortcut_certification(
        input: SyntheticCloseoutShortcutInput,
        transcript: &PhysicalStoryTranscript,
    ) -> Result<(), SyntheticCloseoutRejectionDenial> {
        let boundary = input.attempted_shortcut.required_boundary();
        if input.transcript_identity == *transcript.plan_identity()
            && input.attempted_boundary == boundary
            && transcript
                .shortcut_trace()
                .forbidden_shortcuts()
                .contains(&boundary)
        {
            Err(SyntheticCloseoutRejectionDenial {
                attempt: input.attempted_shortcut,
                boundary,
                transcript_identity: transcript.plan_identity().clone(),
            })
        } else {
            Ok(())
        }
    }

    pub fn from_failed_shortcut_attempt(denial: SyntheticCloseoutRejectionDenial) -> Self {
        Self {
            rejected_attempt: denial.attempt,
            rejected_boundary: denial.boundary,
            transcript_identity: denial.transcript_identity,
        }
    }

    pub const fn rejected_attempt(&self) -> SyntheticCloseoutShortcutAttempt {
        self.rejected_attempt
    }

    pub const fn rejected_boundary(&self) -> ScenarioDenialBoundary {
        self.rejected_boundary
    }

    pub const fn transcript_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.transcript_identity
    }
}

impl SyntheticCloseoutShortcutAttempt {
    pub const fn required_boundary(self) -> ScenarioDenialBoundary {
        match self {
            Self::LogsOnlyProof => ScenarioDenialBoundary::WholeStoreMaterialization,
            Self::SameRunSelfComparison => ScenarioDenialBoundary::BypassedLoweredPlan,
            Self::ExpectedErrorsOnly => ScenarioDenialBoundary::BypassedObserverTrace,
            Self::InMemoryOnlyBuffers => ScenarioDenialBoundary::WholeStoreMaterialization,
            Self::SmallFixtureOnly => ScenarioDenialBoundary::BypassedObserverTrace,
            Self::FixtureLabelsOnly => ScenarioDenialBoundary::TestSupportOwnedMeaning,
            Self::TestSupportOwnedOracleMeaning => ScenarioDenialBoundary::TestSupportOwnedMeaning,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticCloseoutRejectionDenial {
    attempt: SyntheticCloseoutShortcutAttempt,
    boundary: ScenarioDenialBoundary,
    transcript_identity: PhysicalScenarioPlanIdentity,
}

impl SyntheticCloseoutRejectionDenial {
    pub const fn rejected_attempt(&self) -> SyntheticCloseoutShortcutAttempt {
        self.attempt
    }

    pub const fn boundary(&self) -> ScenarioDenialBoundary {
        self.boundary
    }

    pub const fn transcript_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.transcript_identity
    }
}
