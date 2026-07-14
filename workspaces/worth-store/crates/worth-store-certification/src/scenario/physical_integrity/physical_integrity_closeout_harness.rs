use crate::scenario::physical_integrity::physical_integrity_closeout_harness_execution::ExecutedIntegrityCloseoutHarnessRun;
use crate::{
    IntegrityHarnessExecutionEvidence, PhysicalIntegrityAcceptanceSuite,
    PhysicalIntegrityCloseoutDenial, PhysicalOracleOutcome, PhysicalProofOracleKind,
    PhysicalScenarioDriverKind, PhysicalScenarioObserverKind, PhysicalScenarioPlan,
    PhysicalScenarioPlanIdentity, PhysicalStoryTranscript, RoadmapLaneFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityHarnessTranscriptEvidence {
    acceptance_suite: PhysicalIntegrityAcceptanceSuite,
    transcript_identity: PhysicalScenarioPlanIdentity,
    lane_family: RoadmapLaneFamily,
    driver_families: Vec<PhysicalScenarioDriverKind>,
    observer_families: Vec<PhysicalScenarioObserverKind>,
    oracle_families: Vec<PhysicalProofOracleKind>,
    executed_output: IntegrityHarnessExecutionEvidence,
}

impl IntegrityHarnessTranscriptEvidence {
    pub(crate) fn from_executed_closeout_run(
        executed: ExecutedIntegrityCloseoutHarnessRun,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        Self::from_suite_plan_and_transcript(
            executed.acceptance_suite(),
            executed.plan(),
            executed.transcript(),
            executed.executed_output(),
        )
    }

    pub(crate) fn from_suite_plan_and_transcript(
        acceptance_suite: PhysicalIntegrityAcceptanceSuite,
        plan: &PhysicalScenarioPlan,
        transcript: &PhysicalStoryTranscript,
        executed_output: IntegrityHarnessExecutionEvidence,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        require_harness_identity(acceptance_suite, plan, transcript, executed_output)?;
        Ok(Self {
            acceptance_suite,
            transcript_identity: transcript.plan_identity().clone(),
            lane_family: transcript.plan_identity().lane_family(),
            driver_families: plan
                .driver_requirements()
                .iter()
                .map(|requirement| requirement.kind())
                .collect(),
            observer_families: transcript.observer_trace().observed_observers().to_vec(),
            oracle_families: transcript
                .judgments()
                .iter()
                .map(|judgment| judgment.oracle())
                .collect(),
            executed_output,
        })
    }

    pub const fn acceptance_suite(&self) -> PhysicalIntegrityAcceptanceSuite {
        self.acceptance_suite
    }

    pub const fn lane_family(&self) -> RoadmapLaneFamily {
        self.lane_family
    }

    pub const fn transcript_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.transcript_identity
    }

    pub fn driver_families(&self) -> &[PhysicalScenarioDriverKind] {
        &self.driver_families
    }

    pub fn observer_families(&self) -> &[PhysicalScenarioObserverKind] {
        &self.observer_families
    }

    pub fn oracle_families(&self) -> &[PhysicalProofOracleKind] {
        &self.oracle_families
    }

    pub const fn executed_output(&self) -> IntegrityHarnessExecutionEvidence {
        self.executed_output
    }

    pub fn names_required_families(&self) -> bool {
        !self.driver_families.is_empty()
            && !self.observer_families.is_empty()
            && !self.oracle_families.is_empty()
            && self.executed_output.output_count() > 0
    }
}

fn require_harness_identity(
    suite: PhysicalIntegrityAcceptanceSuite,
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
    executed_output: IntegrityHarnessExecutionEvidence,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    executed_output.require_suite(suite)?;
    if transcript.plan_identity() != plan.identity() {
        return Err(PhysicalIntegrityCloseoutDenial::MissingHarnessTranscript(
            suite,
        ));
    }
    if transcript.plan_identity().lane_family() != RoadmapLaneFamily::Integrity {
        return Err(PhysicalIntegrityCloseoutDenial::WrongHarnessLane(suite));
    }
    if plan.driver_requirements().is_empty()
        || transcript.observer_trace().observed_observers().is_empty()
        || transcript.judgments().is_empty()
    {
        return Err(PhysicalIntegrityCloseoutDenial::MissingHarnessFamily(suite));
    }
    require_suite_specific_families(suite, plan, transcript)?;
    if !has_satisfied_oracle(
        transcript,
        PhysicalProofOracleKind::ScenarioPlanOwnsStrategy,
    ) || !has_satisfied_oracle(
        transcript,
        PhysicalProofOracleKind::TranscriptPreservesEvidence,
    ) {
        return Err(PhysicalIntegrityCloseoutDenial::MissingHarnessFamily(suite));
    }
    Ok(())
}

fn require_suite_specific_families(
    suite: PhysicalIntegrityAcceptanceSuite,
    plan: &PhysicalScenarioPlan,
    transcript: &PhysicalStoryTranscript,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let driver = required_driver(suite);
    if !plan
        .driver_requirements()
        .iter()
        .any(|requirement| requirement.kind() == driver)
    {
        return Err(PhysicalIntegrityCloseoutDenial::MissingHarnessFamily(suite));
    }
    if !transcript
        .observer_trace()
        .observed_observers()
        .contains(&required_observer(suite))
    {
        return Err(PhysicalIntegrityCloseoutDenial::MissingHarnessFamily(suite));
    }
    if !has_satisfied_oracle(transcript, required_oracle(suite)) {
        return Err(PhysicalIntegrityCloseoutDenial::MissingHarnessFamily(suite));
    }
    Ok(())
}

const fn required_driver(suite: PhysicalIntegrityAcceptanceSuite) -> PhysicalScenarioDriverKind {
    match suite {
        PhysicalIntegrityAcceptanceSuite::CorruptionLocalization => {
            PhysicalScenarioDriverKind::ByteFlipInjection
        }
        PhysicalIntegrityAcceptanceSuite::BoundaryDenial => {
            PhysicalScenarioDriverKind::IntegrityBoundaryDenialProbe
        }
        PhysicalIntegrityAcceptanceSuite::HarnessTranscript => {
            PhysicalScenarioDriverKind::PersistedFileDevice
        }
        PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection => {
            PhysicalScenarioDriverKind::SyntheticShortcutAttempt
        }
        PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff => {
            PhysicalScenarioDriverKind::RecoveryIntegrityHandoffProbe
        }
        PhysicalIntegrityAcceptanceSuite::LineCapComposition => {
            PhysicalScenarioDriverKind::IntegrityCompositionDiscovery
        }
    }
}

const fn required_observer(
    suite: PhysicalIntegrityAcceptanceSuite,
) -> PhysicalScenarioObserverKind {
    match suite {
        PhysicalIntegrityAcceptanceSuite::CorruptionLocalization => {
            PhysicalScenarioObserverKind::DamageClassification
        }
        PhysicalIntegrityAcceptanceSuite::BoundaryDenial => {
            PhysicalScenarioObserverKind::PreDecodeIntegrityAdmission
        }
        PhysicalIntegrityAcceptanceSuite::HarnessTranscript => {
            PhysicalScenarioObserverKind::EvidenceExport
        }
        PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection => {
            PhysicalScenarioObserverKind::MaterializationShortcut
        }
        PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff => {
            PhysicalScenarioObserverKind::RecoveryIntegrityHandoff
        }
        PhysicalIntegrityAcceptanceSuite::LineCapComposition => {
            PhysicalScenarioObserverKind::IntegrityComposition
        }
    }
}

const fn required_oracle(suite: PhysicalIntegrityAcceptanceSuite) -> PhysicalProofOracleKind {
    match suite {
        PhysicalIntegrityAcceptanceSuite::CorruptionLocalization => {
            PhysicalProofOracleKind::DamageLocalizesToPhysicalBoundary
        }
        PhysicalIntegrityAcceptanceSuite::BoundaryDenial => {
            PhysicalProofOracleKind::DamagedBytesDenyBeforeLogicalDecode
        }
        PhysicalIntegrityAcceptanceSuite::HarnessTranscript => {
            PhysicalProofOracleKind::TranscriptPreservesEvidence
        }
        PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection => {
            PhysicalProofOracleKind::SyntheticShortcutRejected
        }
        PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff => {
            PhysicalProofOracleKind::RecoveryHandoffContainsOnlyIntegrityEvidence
        }
        PhysicalIntegrityAcceptanceSuite::LineCapComposition => {
            PhysicalProofOracleKind::IntegrityCompositionChecked
        }
    }
}

fn has_satisfied_oracle(
    transcript: &PhysicalStoryTranscript,
    oracle: PhysicalProofOracleKind,
) -> bool {
    transcript.judgments().iter().any(|judgment| {
        judgment.oracle() == oracle && judgment.outcome() == PhysicalOracleOutcome::Satisfied
    })
}
