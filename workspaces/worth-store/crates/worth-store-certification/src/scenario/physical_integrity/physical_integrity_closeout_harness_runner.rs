use crate::scenario::physical_integrity::physical_integrity_closeout_harness_execution::ExecutedIntegrityCloseoutHarnessRun;
use crate::{
    IntegrityHarnessExecutionEvidence, IntegrityHarnessTranscriptEvidence, LaneFamilyExtension,
    PhysicalIntegrityAcceptanceSuite, PhysicalIntegrityCloseoutDenial, PhysicalProofOracleKind,
    PhysicalScenarioDefinition, PhysicalScenarioDriverKind, PhysicalScenarioObserverKind,
    PhysicalScenarioQualityHarness, PhysicalStoryStep, PhysicalStoryTranscript, RoadmapLaneFamily,
};
use worth_store_test_support::{LargeStorePressureClass, LargeStorePressureFixture};

pub(crate) struct IntegrityHarnessRunOutput {
    harness: IntegrityHarnessTranscriptEvidence,
}

impl IntegrityHarnessRunOutput {
    pub(crate) const fn harness(&self) -> &IntegrityHarnessTranscriptEvidence {
        &self.harness
    }
}

pub(crate) fn run_physical_integrity_closeout_harness(
    suite: PhysicalIntegrityAcceptanceSuite,
    execution: IntegrityHarnessExecutionEvidence,
) -> Result<IntegrityHarnessRunOutput, PhysicalIntegrityCloseoutDenial> {
    let (plan, transcript) = physical_integrity_closeout_suite_plan_and_transcript(suite)?;
    let executed = ExecutedIntegrityCloseoutHarnessRun::from_executed_output(
        suite, plan, transcript, execution,
    )?;
    let harness = IntegrityHarnessTranscriptEvidence::from_executed_closeout_run(executed)?;
    Ok(IntegrityHarnessRunOutput { harness })
}

pub(crate) fn physical_integrity_closeout_suite_plan_and_transcript(
    suite: PhysicalIntegrityAcceptanceSuite,
) -> Result<(crate::PhysicalScenarioPlan, PhysicalStoryTranscript), PhysicalIntegrityCloseoutDenial>
{
    let mut harness = PhysicalScenarioQualityHarness::cross_cutting_scenario();
    for extension in suite_lane_extensions(suite) {
        harness = harness
            .with_lane_family_extension(extension)
            .map_err(|_| PhysicalIntegrityCloseoutDenial::HarnessExecutionFailed(suite))?;
    }
    let definition = PhysicalScenarioDefinition::story(format!("new-closeout-{:?}", suite))
        .roadmap_lane_family(RoadmapLaneFamily::Integrity)
        .large_store_pressure_fixture(LargeStorePressureFixture::for_class(
            LargeStorePressureClass::StreamingPressure,
        ))
        .proves_law(format!(
            "S.3 {:?} suite must execute through Roadmap 2",
            suite
        ))
        .step(PhysicalStoryStep::GivenHostilePhysicalBytes)
        .step(PhysicalStoryStep::ThenShortcutCertificationFails)
        .requires_oracle(suite_required_oracle(suite))
        .define()
        .map_err(|_| PhysicalIntegrityCloseoutDenial::HarnessExecutionFailed(suite))?;
    let plan = harness
        .lower(definition)
        .map_err(|_| PhysicalIntegrityCloseoutDenial::HarnessExecutionFailed(suite))?;
    let transcript =
        harness.transcribe(harness.judge(harness.observe(harness.execute(plan.clone()))));
    Ok((plan, transcript))
}

fn suite_lane_extensions(suite: PhysicalIntegrityAcceptanceSuite) -> Vec<LaneFamilyExtension> {
    suite_drivers(suite)
        .into_iter()
        .map(|driver| {
            LaneFamilyExtension::new(
                RoadmapLaneFamily::Integrity,
                driver,
                suite_required_oracle(suite),
            )
            .with_observer(suite_observer(suite))
        })
        .collect()
}

fn suite_drivers(suite: PhysicalIntegrityAcceptanceSuite) -> Vec<PhysicalScenarioDriverKind> {
    match suite {
        PhysicalIntegrityAcceptanceSuite::CorruptionLocalization => vec![
            PhysicalScenarioDriverKind::ByteFlipInjection,
            PhysicalScenarioDriverKind::TornFrameInjection,
            PhysicalScenarioDriverKind::StaleGenerationProbe,
            PhysicalScenarioDriverKind::ManifestDamageInjection,
            PhysicalScenarioDriverKind::IndexPageDamageInjection,
            PhysicalScenarioDriverKind::WalFrameDamageInjection,
            PhysicalScenarioDriverKind::ExtentDamageInjection,
            PhysicalScenarioDriverKind::ChunkDamageInjection,
        ],
        PhysicalIntegrityAcceptanceSuite::BoundaryDenial => {
            vec![PhysicalScenarioDriverKind::IntegrityBoundaryDenialProbe]
        }
        PhysicalIntegrityAcceptanceSuite::HarnessTranscript => {
            vec![PhysicalScenarioDriverKind::PersistedFileDevice]
        }
        PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection => {
            vec![PhysicalScenarioDriverKind::SyntheticShortcutAttempt]
        }
        PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff => {
            vec![PhysicalScenarioDriverKind::RecoveryIntegrityHandoffProbe]
        }
        PhysicalIntegrityAcceptanceSuite::LineCapComposition => {
            vec![PhysicalScenarioDriverKind::IntegrityCompositionDiscovery]
        }
    }
}

const fn suite_observer(suite: PhysicalIntegrityAcceptanceSuite) -> PhysicalScenarioObserverKind {
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

const fn suite_required_oracle(suite: PhysicalIntegrityAcceptanceSuite) -> PhysicalProofOracleKind {
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
