use crate::PhysicalIsolationHarnessReadinessDenial;

use crate::{
    HarnessCoverageStage, PhysicalScenarioActorRole, PhysicalScenarioExpectationKind,
    PhysicalScenarioIntent, PhysicalScenarioNonClaim, PhysicalSimulationScenarioFamily,
};

use super::{
    SimulationHarnessAcceptanceEvidenceLane, SimulationHarnessAcceptanceSuiteName,
    SimulationHarnessDogfoodSliceKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalSimulationHarnessCloseoutDenial {
    WrongCloseoutSuite {
        expected: HarnessCoverageStage,
        actual: HarnessCoverageStage,
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
        slice: SimulationHarnessDogfoodSliceKind,
    },
    DogfoodSliceScenarioCoverageMissing {
        slice: SimulationHarnessDogfoodSliceKind,
    },
    DogfoodSliceTranscriptCoverageMissing {
        slice: SimulationHarnessDogfoodSliceKind,
    },
    MissingScenarioNonClaim {
        non_claim: PhysicalScenarioNonClaim,
    },
    MissingShortcutDenialReport,
    MissingMutationCoverage,
    MissingAcceptanceSuiteLane {
        suite: SimulationHarnessAcceptanceSuiteName,
        lane: SimulationHarnessAcceptanceEvidenceLane,
    },
    MissingAcceptanceSuiteReceipt {
        suite: SimulationHarnessAcceptanceSuiteName,
    },
    DuplicateAcceptanceSuiteReceipt {
        suite: SimulationHarnessAcceptanceSuiteName,
    },
    MissingAcceptanceSuiteExecution {
        suite: SimulationHarnessAcceptanceSuiteName,
    },
    DuplicateAcceptanceSuiteExecution {
        suite: SimulationHarnessAcceptanceSuiteName,
    },
    StaleAcceptanceSuiteReceipt {
        suite: SimulationHarnessAcceptanceSuiteName,
    },
    MissingPhysicalIsolationReadinessReceipt(PhysicalIsolationHarnessReadinessDenial),
    FutureSlotClaimedImplementedBehavior,
}

impl From<PhysicalIsolationHarnessReadinessDenial> for PhysicalSimulationHarnessCloseoutDenial {
    fn from(denial: PhysicalIsolationHarnessReadinessDenial) -> Self {
        Self::MissingPhysicalIsolationReadinessReceipt(denial)
    }
}
