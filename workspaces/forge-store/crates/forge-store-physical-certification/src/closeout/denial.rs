use forge_store_readiness::S5SimulationHarnessReadinessDenial;

use crate::{
    PhysicalScenarioActorRole, PhysicalScenarioExpectationKind, PhysicalScenarioIntent,
    PhysicalScenarioNonClaim, PhysicalSimulationScenarioFamily, Roadmap2HarnessSequence,
};

use super::{S45AcceptanceEvidenceLane, S45AcceptanceSuiteName, S45DogfoodSliceKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalSimulationHarnessCloseoutDenial {
    WrongCloseoutSuite {
        expected: Roadmap2HarnessSequence,
        actual: Roadmap2HarnessSequence,
    },
    WrongDogfoodScenarioFamily {
        expected: PhysicalSimulationScenarioFamily,
        actual: PhysicalSimulationScenarioFamily,
    },
    WrongDogfoodScenarioIntent {
        expected: PhysicalScenarioIntent,
        actual: PhysicalScenarioIntent,
    },
    WrongDogfoodScenarioExpectation {
        expected: PhysicalScenarioExpectationKind,
        actual: PhysicalScenarioExpectationKind,
    },
    MissingDogfoodActor {
        role: PhysicalScenarioActorRole,
    },
    DogfoodSliceScenarioEvidenceMismatch {
        slice: S45DogfoodSliceKind,
    },
    DogfoodSliceScenarioCoverageMissing {
        slice: S45DogfoodSliceKind,
    },
    DogfoodSliceTranscriptCoverageMissing {
        slice: S45DogfoodSliceKind,
    },
    MissingScenarioNonClaim {
        non_claim: PhysicalScenarioNonClaim,
    },
    MissingShortcutDenialReport,
    MissingMutationCoverage,
    MissingAcceptanceSuiteLane {
        suite: S45AcceptanceSuiteName,
        lane: S45AcceptanceEvidenceLane,
    },
    MissingAcceptanceSuiteReceipt {
        suite: S45AcceptanceSuiteName,
    },
    DuplicateAcceptanceSuiteReceipt {
        suite: S45AcceptanceSuiteName,
    },
    MissingAcceptanceSuiteExecution {
        suite: S45AcceptanceSuiteName,
    },
    DuplicateAcceptanceSuiteExecution {
        suite: S45AcceptanceSuiteName,
    },
    StaleAcceptanceSuiteReceipt {
        suite: S45AcceptanceSuiteName,
    },
    MissingS5ReadinessReceipt(S5SimulationHarnessReadinessDenial),
    FutureSlotClaimedImplementedBehavior,
}

impl From<S5SimulationHarnessReadinessDenial> for PhysicalSimulationHarnessCloseoutDenial {
    fn from(denial: S5SimulationHarnessReadinessDenial) -> Self {
        Self::MissingS5ReadinessReceipt(denial)
    }
}
