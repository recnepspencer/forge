use crate::scenario::physical_integrity::physical_integrity_closeout_harness_execution::S3ExecutedCloseoutHarnessRun;
use crate::{
    LaneFamilyExtension, PhysicalIntegrityCloseoutDenial, PhysicalProofOracleKind,
    PhysicalScenarioDefinition, PhysicalScenarioDriverKind, PhysicalScenarioObserverKind,
    PhysicalScenarioQualityHarness, PhysicalStoryStep, PhysicalStoryTranscript, RoadmapLaneFamily,
    S3AcceptanceSuiteKind, S3CloseoutHarnessExecutionEvidence, S3HarnessTranscriptEvidence,
};
use forge_store_test_support::{LargeStorePressureClass, LargeStorePressureFixture};

pub(crate) struct S3CloseoutHarnessRunOutput {
    harness: S3HarnessTranscriptEvidence,
}

impl S3CloseoutHarnessRunOutput {
    pub(crate) const fn harness(&self) -> &S3HarnessTranscriptEvidence {
        &self.harness
    }
}

pub(crate) fn run_physical_integrity_closeout_harness(
    suite: S3AcceptanceSuiteKind,
    execution: S3CloseoutHarnessExecutionEvidence,
) -> Result<S3CloseoutHarnessRunOutput, PhysicalIntegrityCloseoutDenial> {
    let (plan, transcript) = physical_integrity_closeout_suite_plan_and_transcript(suite)?;
    let executed =
        S3ExecutedCloseoutHarnessRun::from_executed_output(suite, plan, transcript, execution)?;
    let harness = S3HarnessTranscriptEvidence::from_executed_closeout_run(executed)?;
    Ok(S3CloseoutHarnessRunOutput { harness })
}

pub(crate) fn physical_integrity_closeout_suite_plan_and_transcript(
    suite: S3AcceptanceSuiteKind,
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

fn suite_lane_extensions(suite: S3AcceptanceSuiteKind) -> Vec<LaneFamilyExtension> {
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

fn suite_drivers(suite: S3AcceptanceSuiteKind) -> Vec<PhysicalScenarioDriverKind> {
    match suite {
        S3AcceptanceSuiteKind::CorruptionLocalization => vec![
            PhysicalScenarioDriverKind::S3ByteFlipInjection,
            PhysicalScenarioDriverKind::S3TornFrameInjection,
            PhysicalScenarioDriverKind::S3StaleGenerationProbe,
            PhysicalScenarioDriverKind::S3ManifestDamageInjection,
            PhysicalScenarioDriverKind::S3IndexPageDamageInjection,
            PhysicalScenarioDriverKind::S3WalFrameDamageInjection,
            PhysicalScenarioDriverKind::S3ExtentDamageInjection,
            PhysicalScenarioDriverKind::S3ChunkDamageInjection,
        ],
        S3AcceptanceSuiteKind::BoundaryDenial => {
            vec![PhysicalScenarioDriverKind::S3BoundaryDenialProbe]
        }
        S3AcceptanceSuiteKind::HarnessTranscript => {
            vec![PhysicalScenarioDriverKind::PersistedFileDevice]
        }
        S3AcceptanceSuiteKind::SyntheticShortcutRejection => {
            vec![PhysicalScenarioDriverKind::S3SyntheticShortcutAttempt]
        }
        S3AcceptanceSuiteKind::S4IntegrityHandoff => {
            vec![PhysicalScenarioDriverKind::S3RecoveryHandoffProbe]
        }
        S3AcceptanceSuiteKind::LineCapComposition => {
            vec![PhysicalScenarioDriverKind::S3LineCapDiscovery]
        }
    }
}

const fn suite_observer(suite: S3AcceptanceSuiteKind) -> PhysicalScenarioObserverKind {
    match suite {
        S3AcceptanceSuiteKind::CorruptionLocalization => {
            PhysicalScenarioObserverKind::S3DamageClassification
        }
        S3AcceptanceSuiteKind::BoundaryDenial => PhysicalScenarioObserverKind::S3PreDecodeAdmission,
        S3AcceptanceSuiteKind::HarnessTranscript => PhysicalScenarioObserverKind::EvidenceExport,
        S3AcceptanceSuiteKind::SyntheticShortcutRejection => {
            PhysicalScenarioObserverKind::MaterializationShortcut
        }
        S3AcceptanceSuiteKind::S4IntegrityHandoff => {
            PhysicalScenarioObserverKind::S3RecoveryHandoff
        }
        S3AcceptanceSuiteKind::LineCapComposition => {
            PhysicalScenarioObserverKind::S3LineCapComposition
        }
    }
}

const fn suite_required_oracle(suite: S3AcceptanceSuiteKind) -> PhysicalProofOracleKind {
    match suite {
        S3AcceptanceSuiteKind::CorruptionLocalization => {
            PhysicalProofOracleKind::S3DamageLocalizesToPhysicalBoundary
        }
        S3AcceptanceSuiteKind::BoundaryDenial => {
            PhysicalProofOracleKind::S3DamagedBytesDenyBeforeLogicalDecode
        }
        S3AcceptanceSuiteKind::HarnessTranscript => {
            PhysicalProofOracleKind::TranscriptPreservesEvidence
        }
        S3AcceptanceSuiteKind::SyntheticShortcutRejection => {
            PhysicalProofOracleKind::S3SyntheticShortcutRejected
        }
        S3AcceptanceSuiteKind::S4IntegrityHandoff => {
            PhysicalProofOracleKind::S3RecoveryHandoffContainsOnlyIntegrityEvidence
        }
        S3AcceptanceSuiteKind::LineCapComposition => {
            PhysicalProofOracleKind::S3LineCapCompositionChecked
        }
    }
}
