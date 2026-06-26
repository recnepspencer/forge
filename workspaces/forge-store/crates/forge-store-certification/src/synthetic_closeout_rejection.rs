use crate::{PhysicalStoryTranscript, ScenarioDenialBoundary};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntheticCloseoutShortcutAttempt {
    LogsOnlyProof,
    SameRunSelfComparison,
    SmallFixtureOnly,
    TestSupportOwnedOracleMeaning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntheticCloseoutShortcutRejectionReport {
    rejected_attempt: SyntheticCloseoutShortcutAttempt,
    rejected_boundary: ScenarioDenialBoundary,
}

impl SyntheticCloseoutShortcutRejectionReport {
    pub fn attempt_shortcut_certification(
        attempt: SyntheticCloseoutShortcutAttempt,
        transcript: &PhysicalStoryTranscript,
    ) -> Result<(), SyntheticCloseoutRejectionDenial> {
        let boundary = attempt.required_boundary();
        if transcript
            .shortcut_trace()
            .forbidden_shortcuts()
            .contains(&boundary)
        {
            Err(SyntheticCloseoutRejectionDenial { attempt, boundary })
        } else {
            Ok(())
        }
    }

    pub fn from_failed_shortcut_attempt(denial: SyntheticCloseoutRejectionDenial) -> Self {
        Self {
            rejected_attempt: denial.attempt,
            rejected_boundary: denial.boundary,
        }
    }

    pub const fn rejected_attempt(self) -> SyntheticCloseoutShortcutAttempt {
        self.rejected_attempt
    }

    pub const fn rejected_boundary(self) -> ScenarioDenialBoundary {
        self.rejected_boundary
    }
}

impl SyntheticCloseoutShortcutAttempt {
    pub const fn required_boundary(self) -> ScenarioDenialBoundary {
        match self {
            Self::LogsOnlyProof => ScenarioDenialBoundary::WholeStoreMaterialization,
            Self::SameRunSelfComparison => ScenarioDenialBoundary::BypassedLoweredPlan,
            Self::SmallFixtureOnly => ScenarioDenialBoundary::BypassedObserverTrace,
            Self::TestSupportOwnedOracleMeaning => ScenarioDenialBoundary::TestSupportOwnedMeaning,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntheticCloseoutRejectionDenial {
    attempt: SyntheticCloseoutShortcutAttempt,
    boundary: ScenarioDenialBoundary,
}

impl SyntheticCloseoutRejectionDenial {
    pub const fn rejected_attempt(self) -> SyntheticCloseoutShortcutAttempt {
        self.attempt
    }

    pub const fn boundary(self) -> ScenarioDenialBoundary {
        self.boundary
    }
}
