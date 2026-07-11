use crate::{
    BufferPoolPressureTranscriptIdentity, BufferPoolScenarioPlan, LaneFamilyExtension,
    LargeStoreMemoryPressureScenario, LargeStorePressureClass, LargeStorePressureEvidenceBundle,
    LargeStorePressureEvidenceDenial, LargeStoreShortcutAttempt, PhysicalCounterExpectationKind,
    PhysicalProofOracleKind, PhysicalScenarioDriverKind, PhysicalScenarioObserverKind,
    PhysicalScenarioQualityHarness, RoadmapLaneFamily, ScenarioDenialBoundary,
};

#[test]
fn large_store_pressure_classes_run_through_buffer_pool_harness() {
    for class in LargeStorePressureClass::ALL {
        let proof = run_pressure_class(class);
        assert_eq!(proof.bundle.pressure_class(), class);
        assert!(proof
            .transcript
            .counter_trace()
            .is_expected(PhysicalCounterExpectationKind::ResidentBytesPeak));
        assert!(proof
            .transcript
            .counter_trace()
            .is_expected(PhysicalCounterExpectationKind::AllocationBytesPeak));
        assert_eq!(
            proof
                .transcript
                .counter_trace()
                .observed_value(PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts),
            Some(0)
        );
        for counter in required_pressure_counters() {
            assert!(proof.transcript.counter_trace().is_expected(counter));
        }
        for observer in required_pressure_observers() {
            assert!(proof.transcript.observer_trace().contains(observer));
        }
    }
}

#[test]
fn large_store_pressure_replays_to_the_same_identity() {
    for class in LargeStorePressureClass::ALL {
        let first = run_pressure_class(class);
        let second = run_pressure_class(class);
        assert_eq!(first.plan_identity, second.plan_identity);
        assert_eq!(first.transcript_identity, second.transcript_identity);
        assert_eq!(
            first.transcript.counter_trace(),
            second.transcript.counter_trace()
        );
        assert_eq!(
            first.transcript.denial_trace(),
            second.transcript.denial_trace()
        );
    }
}

#[test]
fn protected_and_streaming_pressure_deny_before_materialization() {
    let protected = run_pressure_class(LargeStorePressureClass::ProtectedPressure);
    assert_eq!(
        protected.transcript.denial_trace().expected_denial(),
        Some(ScenarioDenialBoundary::ProtectedResidentPressure)
    );
    assert_protected_pressure_pin_counter(&protected);
    assert_no_unbounded_materialization(&protected);

    let streaming = run_pressure_class(LargeStorePressureClass::StreamingPressure);
    assert_eq!(
        streaming.transcript.denial_trace().expected_denial(),
        Some(ScenarioDenialBoundary::StreamingWindowPressure)
    );
    assert_streaming_pressure_copy_counter(&streaming);
    assert_no_unbounded_materialization(&streaming);
}

#[test]
fn shortcut_tests_fail_certification() {
    for attempt in [
        LargeStoreShortcutAttempt::BypassLoweredPlan,
        LargeStoreShortcutAttempt::BypassObserverTrace,
        LargeStoreShortcutAttempt::TestSupportOwnsMeaning,
    ] {
        let denial = LargeStorePressureEvidenceBundle::reject_shortcut(attempt)
            .expect_err("shortcut evidence must not certify");
        assert_eq!(
            denial,
            LargeStorePressureEvidenceDenial::ShortcutRejected(attempt.denial_boundary())
        );
    }
}

#[test]
fn evidence_rejects_transcript_paired_with_the_wrong_lowered_plan() {
    let harness = buffer_pool_pressure_harness();
    let first_plan = lower_pressure_plan(&harness, LargeStorePressureClass::BarelyOverBudget);
    let second_plan = lower_pressure_plan(&harness, LargeStorePressureClass::FarOverBudget);
    let first_buffer_plan =
        BufferPoolScenarioPlan::admit(&first_plan).expect("first pressure plan is admitted");
    let second_transcript =
        harness.transcribe(harness.judge(harness.observe(harness.execute(second_plan))));

    let denial = LargeStorePressureEvidenceBundle::from_harness_transcript(
        &first_buffer_plan,
        &second_transcript,
    )
    .expect_err("mismatched plan and transcript must not certify");
    assert_eq!(
        denial,
        LargeStorePressureEvidenceDenial::PlanTranscriptMismatch
    );
}

#[test]
fn evidence_rejects_pressure_transcript_without_observer_trace() {
    let harness = pressure_harness_without_memory_observers();
    let plan = lower_pressure_plan(&harness, LargeStorePressureClass::BarelyOverBudget);
    let buffer_plan =
        BufferPoolScenarioPlan::admit(&plan).expect("pressure plan shape is admitted");
    let transcript =
        harness.transcribe(harness.judge(harness.observe(harness.execute(plan.clone()))));

    let denial =
        LargeStorePressureEvidenceBundle::from_harness_transcript(&buffer_plan, &transcript)
            .expect_err("pressure evidence requires executed observer trace");
    assert_eq!(
        denial,
        LargeStorePressureEvidenceDenial::MissingObserverTrace(
            PhysicalScenarioObserverKind::ResidentBudget
        )
    );
}

#[test]
fn evidence_rejects_pressure_transcript_missing_required_counters() {
    let harness = buffer_pool_pressure_harness();
    let plan = lower_pressure_plan(&harness, LargeStorePressureClass::StreamingPressure);
    let buffer_plan =
        BufferPoolScenarioPlan::admit(&plan).expect("streaming pressure plan is admitted");
    let transcript =
        harness.transcribe(harness.judge(harness.observe(harness.execute(plan.clone()))));

    for counter in required_pressure_counters() {
        let weakened_transcript = transcript.without_counter_for_test(counter);
        let denial = LargeStorePressureEvidenceBundle::from_harness_transcript(
            &buffer_plan,
            &weakened_transcript,
        )
        .expect_err("pressure evidence requires every required counter proof");
        assert_eq!(
            denial,
            LargeStorePressureEvidenceDenial::MissingCounter(counter)
        );
    }
}

struct PressureProof {
    plan_identity: crate::PhysicalScenarioPlanIdentity,
    transcript_identity: BufferPoolPressureTranscriptIdentity,
    transcript: crate::PhysicalStoryTranscript,
    bundle: LargeStorePressureEvidenceBundle,
}

fn run_pressure_class(class: LargeStorePressureClass) -> PressureProof {
    let harness = buffer_pool_pressure_harness();
    let scenario = LargeStoreMemoryPressureScenario::for_class(class);
    let plan = lower_pressure_plan(&harness, class);
    let buffer_plan = BufferPoolScenarioPlan::admit(&plan)
        .expect("lowered plan carries admitted pressure fixture");
    assert_eq!(buffer_plan.pressure_class(), class);
    assert_eq!(
        buffer_plan.expected_counter(PhysicalCounterExpectationKind::PressureFixtureStoreBytes),
        Some(scenario.fixture().declared_store_bytes())
    );
    let execution = harness.execute(plan.clone());
    assert_eq!(
        execution
            .report()
            .observed_counter_value(PhysicalCounterExpectationKind::PressureFixtureStoreBytes),
        Some(scenario.fixture().declared_store_bytes())
    );
    let transcript = harness.transcribe(harness.judge(harness.observe(execution)));
    let transcript_identity =
        BufferPoolPressureTranscriptIdentity::from_transcript(&buffer_plan, &transcript);
    let bundle =
        LargeStorePressureEvidenceBundle::from_harness_transcript(&buffer_plan, &transcript)
            .expect("harness transcript carries pressure evidence");
    PressureProof {
        plan_identity: plan.identity().clone(),
        transcript_identity,
        transcript,
        bundle,
    }
}

fn buffer_pool_pressure_harness() -> PhysicalScenarioQualityHarness {
    PhysicalScenarioQualityHarness::roadmap_2()
        .with_buffer_pool_large_store_pressure_lanes()
        .expect("buffer pool pressure lanes are reserved roadmap lanes")
}

fn pressure_harness_without_memory_observers() -> PhysicalScenarioQualityHarness {
    required_pressure_oracles()
        .into_iter()
        .try_fold(
            PhysicalScenarioQualityHarness::roadmap_2(),
            |harness, oracle| {
                harness.with_lane_family_extension(LaneFamilyExtension::new(
                    RoadmapLaneFamily::BufferPool,
                    PhysicalScenarioDriverKind::MemoryPressureDriver,
                    oracle,
                ))
            },
        )
        .expect("pressure oracle lanes can be registered without pressure observers")
}

fn lower_pressure_plan(
    harness: &PhysicalScenarioQualityHarness,
    class: LargeStorePressureClass,
) -> crate::PhysicalScenarioPlan {
    LargeStoreMemoryPressureScenario::for_class(class)
        .definition()
        .and_then(|definition| {
            harness
                .lower(definition)
                .map_err(|_| crate::LargeStoreScenarioDenial::InvalidScenarioDefinition)
        })
        .expect("buffer pool pressure lanes lower through S.1 harness")
}

fn assert_no_unbounded_materialization(proof: &PressureProof) {
    for counter in [
        PhysicalCounterExpectationKind::DomainObjectConstructions,
        PhysicalCounterExpectationKind::UnboundedAllocationAttempts,
        PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
    ] {
        assert_eq!(
            proof.transcript.counter_trace().observed_value(counter),
            Some(0)
        );
    }
}

fn assert_protected_pressure_pin_counter(proof: &PressureProof) {
    let fixture =
        LargeStoreMemoryPressureScenario::for_class(LargeStorePressureClass::ProtectedPressure)
            .fixture();
    assert_eq!(
        proof
            .transcript
            .counter_trace()
            .observed_value(PhysicalCounterExpectationKind::PinnedPagesPeak),
        Some(fixture.protected_page_count())
    );
}

fn assert_streaming_pressure_copy_counter(proof: &PressureProof) {
    let fixture =
        LargeStoreMemoryPressureScenario::for_class(LargeStorePressureClass::StreamingPressure)
            .fixture();
    assert_eq!(
        proof
            .transcript
            .counter_trace()
            .observed_value(PhysicalCounterExpectationKind::CopiedPayloadBytes),
        Some(fixture.streaming_window_bytes())
    );
}

fn required_pressure_oracles() -> [PhysicalProofOracleKind; 4] {
    [
        PhysicalProofOracleKind::LargeStorePressureBounded,
        PhysicalProofOracleKind::OomAvoidanceBeforeMaterialization,
        PhysicalProofOracleKind::PressureTranscriptReplayStable,
        PhysicalProofOracleKind::ShortcutCertificationRejected,
    ]
}

fn required_pressure_observers() -> [PhysicalScenarioObserverKind; 5] {
    [
        PhysicalScenarioObserverKind::ResidentBudget,
        PhysicalScenarioObserverKind::AllocationEnvelope,
        PhysicalScenarioObserverKind::Materialization,
        PhysicalScenarioObserverKind::MaterializationShortcut,
        PhysicalScenarioObserverKind::CounterBundle,
    ]
}

fn required_pressure_counters() -> [PhysicalCounterExpectationKind; 11] {
    [
        PhysicalCounterExpectationKind::WholeStoreMaterializationAttempts,
        PhysicalCounterExpectationKind::PressureFixtureStoreBytes,
        PhysicalCounterExpectationKind::PressureFixtureResidentBudgetBytes,
        PhysicalCounterExpectationKind::ResidentBytesPeak,
        PhysicalCounterExpectationKind::PinnedPagesPeak,
        PhysicalCounterExpectationKind::DirtyPagesPeak,
        PhysicalCounterExpectationKind::AllocationBytesPeak,
        PhysicalCounterExpectationKind::CopiedPayloadBytes,
        PhysicalCounterExpectationKind::DomainObjectConstructions,
        PhysicalCounterExpectationKind::UnboundedAllocationAttempts,
        PhysicalCounterExpectationKind::DiagnosticMaterializationBytes,
    ]
}
