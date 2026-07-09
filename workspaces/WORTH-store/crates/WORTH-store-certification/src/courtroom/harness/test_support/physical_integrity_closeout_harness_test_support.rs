use crate::physical_integrity_closeout_harness_runner::{
    run_s3_closeout_harness, s3_closeout_suite_plan_and_transcript,
};
use crate::{
    LaneFamilyExtension, PhysicalProofOracleKind, PhysicalScenarioDefinition,
    PhysicalScenarioDriverKind, PhysicalScenarioObserverKind, PhysicalScenarioQualityHarness,
    PhysicalStoryStep, RoadmapLaneFamily, S3AcceptanceSuiteKind,
    S3CloseoutHarnessExecutionEvidence, S3HarnessTranscriptEvidence,
};
use worth_store_test_support::{LargeStorePressureClass, LargeStorePressureFixture};

pub(crate) fn s3_harness(
    suite: S3AcceptanceSuiteKind,
    execution: S3CloseoutHarnessExecutionEvidence,
) -> S3HarnessTranscriptEvidence {
    run_s3_closeout_harness(suite, execution)
        .unwrap()
        .harness()
        .clone()
}

pub(crate) fn s3_synthetic_transcript() -> crate::PhysicalStoryTranscript {
    s3_closeout_suite_plan_and_transcript(S3AcceptanceSuiteKind::SyntheticShortcutRejection)
        .unwrap()
        .1
}

pub(crate) fn lane_plan_and_transcript(
    family: RoadmapLaneFamily,
) -> (crate::PhysicalScenarioPlan, crate::PhysicalStoryTranscript) {
    let harness = PhysicalScenarioQualityHarness::roadmap_2()
        .with_lane_family_extension(
            LaneFamilyExtension::new(
                family,
                PhysicalScenarioDriverKind::AdversarialByteDevice,
                PhysicalProofOracleKind::NoWholeStoreMaterialization,
            )
            .with_observer(PhysicalScenarioObserverKind::CounterBundle),
        )
        .unwrap();
    let definition = PhysicalScenarioDefinition::story(format!("s3-closeout-{}", family.as_str()))
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
