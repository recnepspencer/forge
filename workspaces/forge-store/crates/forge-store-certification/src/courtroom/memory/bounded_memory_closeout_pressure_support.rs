use crate::{
    BufferPoolScenarioPlan, HarnessCloseoutEvidenceReport, HarnessCloseoutTranscriptEvidence,
    LargeStoreMemoryPressureScenario, LargeStorePressureClass, LargeStorePressureEvidenceBundle,
    PhysicalCounterExpectationKind, PhysicalScenarioQualityHarness, S2AcceptanceSuiteKind,
    SyntheticCloseoutShortcutAttempt, SyntheticCloseoutShortcutInput,
    SyntheticCloseoutShortcutRejectionReport,
};

pub(crate) fn pressure_bundles() -> Vec<LargeStorePressureEvidenceBundle> {
    LargeStorePressureClass::ALL
        .into_iter()
        .map(pressure_bundle)
        .collect()
}

pub(crate) fn pressure_bundle(class: LargeStorePressureClass) -> LargeStorePressureEvidenceBundle {
    let (plan, transcript) = pressure_plan_and_transcript(class);
    let buffer_plan = BufferPoolScenarioPlan::admit(&plan).unwrap();
    LargeStorePressureEvidenceBundle::from_harness_transcript(&buffer_plan, &transcript).unwrap()
}

pub(crate) fn harness_evidence() -> HarnessCloseoutEvidenceReport {
    let transcripts = acceptance_suite_transcripts();
    HarnessCloseoutEvidenceReport::from_harness_transcripts(&transcripts).unwrap()
}

pub(crate) fn harness_evidence_without_acceptance_suite(
    suite: S2AcceptanceSuiteKind,
) -> HarnessCloseoutEvidenceReport {
    harness_evidence().without_acceptance_suite_for_test(suite)
}

pub(crate) fn harness_evidence_for_class(
    class: LargeStorePressureClass,
) -> HarnessCloseoutEvidenceReport {
    let (plan, transcript) = pressure_plan_and_transcript(class);
    let evidence = HarnessCloseoutTranscriptEvidence::from_suite_plan_and_transcript(
        S2AcceptanceSuiteKind::LargeStorePressure,
        &plan,
        &transcript,
    )
    .unwrap();
    HarnessCloseoutEvidenceReport::from_harness_transcripts(&[evidence]).unwrap()
}

pub(crate) fn synthetic_rejections() -> Vec<SyntheticCloseoutShortcutRejectionReport> {
    let (_, transcript) = pressure_plan_and_transcript(LargeStorePressureClass::StreamingPressure);
    [
        SyntheticCloseoutShortcutAttempt::LogsOnlyProof,
        SyntheticCloseoutShortcutAttempt::SameRunSelfComparison,
        SyntheticCloseoutShortcutAttempt::ExpectedErrorsOnly,
        SyntheticCloseoutShortcutAttempt::InMemoryOnlyBuffers,
        SyntheticCloseoutShortcutAttempt::SmallFixtureOnly,
        SyntheticCloseoutShortcutAttempt::FixtureLabelsOnly,
        SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning,
    ]
    .into_iter()
    .map(|attempt| {
        let input = SyntheticCloseoutShortcutInput::from_transcript(attempt, &transcript);
        let denial = SyntheticCloseoutShortcutRejectionReport::attempt_shortcut_certification(
            input,
            &transcript,
        )
        .unwrap_err();
        SyntheticCloseoutShortcutRejectionReport::from_failed_shortcut_attempt(denial)
    })
    .collect()
}

fn pressure_plan_and_transcript(
    class: LargeStorePressureClass,
) -> (crate::PhysicalScenarioPlan, crate::PhysicalStoryTranscript) {
    let harness = PhysicalScenarioQualityHarness::roadmap_2()
        .with_buffer_pool_large_store_pressure_lanes()
        .unwrap();
    let plan = LargeStoreMemoryPressureScenario::for_class(class)
        .definition()
        .and_then(|definition| {
            harness
                .lower(definition)
                .map_err(|_| crate::LargeStoreScenarioDenial::InvalidScenarioDefinition)
        })
        .unwrap();
    let transcript =
        harness.transcribe(harness.judge(harness.observe(harness.execute(plan.clone()))));
    assert!(transcript
        .counter_trace()
        .is_expected(PhysicalCounterExpectationKind::ResidentBytesPeak));
    (plan, transcript)
}

fn acceptance_suite_transcripts() -> Vec<HarnessCloseoutTranscriptEvidence> {
    let mut transcripts: Vec<_> = S2AcceptanceSuiteKind::ALL
        .into_iter()
        .map(|suite| {
            let class = pressure_class_for_suite(suite);
            let (plan, transcript) = pressure_plan_and_transcript(class);
            HarnessCloseoutTranscriptEvidence::from_suite_plan_and_transcript(
                suite,
                &plan,
                &transcript,
            )
            .unwrap()
        })
        .collect();
    transcripts.extend(LargeStorePressureClass::ALL.into_iter().map(|class| {
        let (plan, transcript) = pressure_plan_and_transcript(class);
        HarnessCloseoutTranscriptEvidence::from_suite_plan_and_transcript(
            S2AcceptanceSuiteKind::LargeStorePressure,
            &plan,
            &transcript,
        )
        .unwrap()
    }));
    transcripts
}

fn pressure_class_for_suite(suite: S2AcceptanceSuiteKind) -> LargeStorePressureClass {
    match suite {
        S2AcceptanceSuiteKind::S3ReadinessHandoff => LargeStorePressureClass::ProtectedPressure,
        S2AcceptanceSuiteKind::BackgroundEnvelopeHonesty => {
            LargeStorePressureClass::StreamingPressure
        }
        S2AcceptanceSuiteKind::SyntheticTestRejection => LargeStorePressureClass::StreamingPressure,
        S2AcceptanceSuiteKind::BoundedMemoryCloseout
        | S2AcceptanceSuiteKind::LargeStorePressure
        | S2AcceptanceSuiteKind::FoundationalBoundaryEvidence => {
            LargeStorePressureClass::ModeratelyOverBudget
        }
    }
}
