use crate::scenario::physical_integrity::physical_integrity_closeout_harness_runner::{
    run_physical_integrity_closeout_harness, physical_integrity_closeout_suite_plan_and_transcript,
};
use crate::{
    LaneFamilyExtension, PhysicalProofOracleKind, PhysicalScenarioDefinition,
    PhysicalScenarioDriverKind, PhysicalScenarioObserverKind, PhysicalScenarioQualityHarness,
    PhysicalStoryStep, RoadmapLaneFamily, S3AcceptanceSuiteKind,
    S3CloseoutHarnessExecutionEvidence, S3HarnessTranscriptEvidence,
};
use forge_store_test_support::{LargeStorePressureClass, LargeStorePressureFixture};

pub(crate) fn physical_integrity_harness(
    suite: S3AcceptanceSuiteKind,
    execution: S3CloseoutHarnessExecutionEvidence,
) -> S3HarnessTranscriptEvidence {
    run_physical_integrity_closeout_harness(suite, execution)
        .unwrap()
        .harness()
        .clone()
}

pub(crate) fn physical_integrity_synthetic_transcript() -> crate::PhysicalStoryTranscript {
    physical_integrity_closeout_suite_plan_and_transcript(S3AcceptanceSuiteKind::SyntheticShortcutRejection)
        .unwrap()
        .1
}

pub(crate) fn lane_plan_and_transcript(
    family: RoadmapLaneFamily,
) -> (crate::PhysicalScenarioPlan, crate::PhysicalStoryTranscript) {
    let harness = PhysicalScenarioQualityHarness::cross_cutting_scenario()
        .with_lane_family_extension(
            LaneFamilyExtension::new(
                family,
                PhysicalScenarioDriverKind::AdversarialByteDevice,
                PhysicalProofOracleKind::NoWholeStoreMaterialization,
            )
            .with_observer(PhysicalScenarioObserverKind::CounterBundle),
        )
        .unwrap();
    let definition = PhysicalScenarioDefinition::story(format!("new-closeout-{}", family.as_str()))
        .roadmap_lane_family(family)
        .large_store_pressure_fixture(LargeStorePressureFixture::for_class(
            LargeStorePressureClass::StreamingPressure,
        ))
        .proves_law("S.3 closeout evidence must run through the Roadmap 2 harness")
        .step(PhysicalStoryStep::GivenHostilePhysicalBytes)
        .step(PhysicalStoryStep::ThenShortcutCertificationFails)
        .requires_oracle(PhysicalProofOracleKind::NoWholeStoreMaterialization)
        .define()
        .unwrap();
    let plan = harness.lower(definition).unwrap();
    let transcript =
        harness.transcribe(harness.judge(harness.observe(harness.execute(plan.clone()))));
    (plan, transcript)
}
