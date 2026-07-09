use crate::{
    ExpectedPhysicalFootprint, LaneFamilyExtension, PhysicalOracleDenialKind,
    PhysicalOracleOutcome, PhysicalProofOracleKind, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioDefinition, PhysicalScenarioDriverKind,
    PhysicalScenarioHarnessDenial, PhysicalScenarioObserverKind, PhysicalScenarioPlanDenial,
    PhysicalScenarioQualityHarness, PhysicalStoryStep, PhysicalSubstrateLane, RoadmapLaneFamily,
    RuntimeVerifierRelationship, S1CertificationRow, ScenarioDenialBoundary,
};

#[test]
fn physical_story_transcript_replay_is_stable_across_independent_observers() {
    let first = run_story_to_transcript(happy_authority_definition());
    let second = run_story_to_transcript(happy_authority_definition());

    assert_eq!(first, second);
    assert_eq!(
        first.plan_identity().lane_family(),
        RoadmapLaneFamily::PhysicalSubstrate
    );
    assert!(!first.counter_trace().observed_counters().is_empty());
    assert_eq!(
        first.parity_trace().relationship(),
        RuntimeVerifierRelationship::RuntimeMustMatchVerifier
    );
}

#[test]
fn scenario_definition_lowers_into_stable_plan() {
    let harness = PhysicalScenarioQualityHarness::roadmap_2();
    let left = harness.lower(happy_authority_definition()).unwrap();
    let right = harness.lower(happy_authority_definition()).unwrap();

    assert_eq!(left, right);
    assert_eq!(
        left.identity().lane_family(),
        RoadmapLaneFamily::PhysicalSubstrate
    );
    assert_eq!(
        left.resolved_capability(),
        PhysicalScenarioCapabilityTier::PlatformGradePhysicalSubstrate
    );
    assert_eq!(
        left.cost_class(),
        PhysicalScenarioCostClass::BoundedPhysicalLocate
    );
    assert_eq!(
        left.expected_physical_footprint(),
        ExpectedPhysicalFootprint::SinglePageAuthority
    );
    assert!(left
        .required_oracles()
        .contains(&PhysicalProofOracleKind::ScenarioPlanOwnsStrategy));
    assert_eq!(left.artifact_policy(), right.artifact_policy());
    assert_eq!(left.expected_counters(), right.expected_counters());
    assert_eq!(
        left.storage_boundary_crossings(),
        right.storage_boundary_crossings()
    );
}

#[test]
fn execution_preserves_plan_strategy_without_redecision() {
    let harness = PhysicalScenarioQualityHarness::roadmap_2();
    let plan = harness.lower(happy_authority_definition()).unwrap();
    let execution = harness.execute(plan.clone());

    assert_eq!(
        execution.report().executed_driver_requirements(),
        plan.driver_requirements()
    );
    assert_eq!(
        execution.report().executed_observer_requirements(),
        plan.observer_requirements()
    );
    assert_eq!(execution.report().judged_oracles(), plan.required_oracles());
    assert_eq!(
        execution.report().resolved_capability(),
        plan.resolved_capability()
    );
    assert_eq!(execution.report().cost_class(), plan.cost_class());
    assert_eq!(
        execution.report().expected_physical_footprint(),
        plan.expected_physical_footprint()
    );
}

#[test]
fn all_s1_certification_rows_map_to_physical_substrate_lanes() {
    for row in S1CertificationRow::required_for_s1() {
        assert!(!row.physical_substrate_lanes().is_empty(), "{row:?}");
        for lane in row.physical_substrate_lanes() {
            assert_eq!(lane.family(), RoadmapLaneFamily::PhysicalSubstrate);
        }
    }
}

#[test]
fn roadmap_follow_on_lanes_extend_without_forking_harness() {
    let harness = roadmap_extension_harness();

    assert_eq!(harness.lane_family_extensions().len(), 5);
    assert_follow_on_lane_lowers(&harness, RoadmapLaneFamily::BufferPool);
    assert_follow_on_lane_lowers(&harness, RoadmapLaneFamily::WalRecovery);
    assert_follow_on_lane_lowers(&harness, RoadmapLaneFamily::BlobChunks);
    assert_follow_on_lane_lowers(&harness, RoadmapLaneFamily::PhysicalCertification);
    assert_eq!(
        PhysicalScenarioQualityHarness::roadmap_2()
            .with_lane_family_extension(LaneFamilyExtension::new(
                RoadmapLaneFamily::PhysicalSubstrate,
                PhysicalScenarioDriverKind::PlatformBackendCandidate,
                PhysicalProofOracleKind::BoundedPhysicalLocate,
            ))
            .unwrap_err(),
        PhysicalScenarioHarnessDenial::PhysicalSubstrateIsBuiltIn
    );
}

#[test]
fn unregistered_follow_on_lane_is_denied_before_execution() {
    let denial = PhysicalScenarioQualityHarness::roadmap_2()
        .lower(roadmap_family_definition(RoadmapLaneFamily::BufferPool))
        .unwrap_err();

    assert_eq!(denial, PhysicalScenarioPlanDenial::UnregisteredLaneFamily);
}

#[test]
fn legacy_overclaim_story_records_typed_denial_trace() {
    let harness = PhysicalScenarioQualityHarness::roadmap_2();
    let plan = harness.lower(legacy_overclaim_definition()).unwrap();
    let observed = harness.observe(harness.execute(plan));

    assert_eq!(
        observed.denial_trace().expected_denial(),
        Some(ScenarioDenialBoundary::LegacyPlatformClaim)
    );
    assert!(observed
        .denial_trace()
        .observed_denials()
        .contains(&ScenarioDenialBoundary::LegacyPlatformClaim));

    let transcript = harness.transcribe(harness.judge(observed));
    assert!(transcript
        .judgments()
        .iter()
        .all(|judgment| judgment.outcome() == PhysicalOracleOutcome::Satisfied));
}

#[test]
fn oracle_denies_required_parity_when_plan_does_not_support_it() {
    let harness = PhysicalScenarioQualityHarness::roadmap_2();
    let plan = harness.lower(unsupported_parity_definition()).unwrap();
    let transcript = harness.transcribe(harness.judge(harness.observe(harness.execute(plan))));

    assert!(transcript.judgments().iter().any(|judgment| {
        judgment.oracle() == PhysicalProofOracleKind::VerifierRuntimeLayoutParity
            && judgment.outcome()
                == PhysicalOracleOutcome::Denied(
                    PhysicalOracleDenialKind::MissingRuntimeVerifierParity,
                )
    }));
}

#[test]
fn offline_verifier_lane_keeps_observer_and_oracle_responsibilities_separate() {
    let harness = PhysicalScenarioQualityHarness::roadmap_2();
    let plan = harness.lower(offline_verifier_definition()).unwrap();

    assert!(plan
        .observer_requirements()
        .iter()
        .any(|observer| observer.kind() == PhysicalScenarioObserverKind::OfflineVerifier));
    assert!(plan
        .required_oracles()
        .contains(&PhysicalProofOracleKind::VerifierRuntimeLayoutParity));

    let transcript = harness.transcribe(harness.judge(harness.observe(harness.execute(plan))));
    assert_eq!(
        transcript.parity_trace().relationship(),
        RuntimeVerifierRelationship::RuntimeMustMatchVerifier
    );
}

fn run_story_to_transcript(
    definition: PhysicalScenarioDefinition,
) -> crate::PhysicalStoryTranscript {
    let harness = PhysicalScenarioQualityHarness::roadmap_2();
    let plan = harness.lower(definition).unwrap();
    let execution = harness.execute(plan);
    let observed = harness.observe(execution);
    let verdict = harness.judge(observed);
    harness.transcribe(verdict)
}

fn assert_follow_on_lane_lowers(
    harness: &PhysicalScenarioQualityHarness,
    family: RoadmapLaneFamily,
) {
    let plan = harness.lower(roadmap_family_definition(family)).unwrap();

    assert_eq!(plan.identity().lane_family(), family);
    assert_eq!(
        plan.resolved_capability(),
        PhysicalScenarioCapabilityTier::RoadmapFollowOnExtension
    );
    assert_eq!(
        plan.expected_physical_footprint(),
        ExpectedPhysicalFootprint::RoadmapFamilyExtension(family)
    );
    if family == RoadmapLaneFamily::BufferPool {
        assert!(plan
            .driver_requirements()
            .iter()
            .any(|driver| driver.kind() == PhysicalScenarioDriverKind::CrashInterposer));
        assert!(plan
            .driver_requirements()
            .iter()
            .any(|driver| driver.kind() == PhysicalScenarioDriverKind::VerifierOnlyReader));
        assert!(plan
            .required_oracles()
            .contains(&PhysicalProofOracleKind::TranscriptPreservesEvidence));
    }
}

fn roadmap_extension_harness() -> PhysicalScenarioQualityHarness {
    PhysicalScenarioQualityHarness::roadmap_2()
        .with_lane_family_extension(LaneFamilyExtension::new(
            RoadmapLaneFamily::BufferPool,
            PhysicalScenarioDriverKind::CrashInterposer,
            PhysicalProofOracleKind::ScenarioPlanOwnsStrategy,
        ))
        .unwrap()
        .with_lane_family_extension(LaneFamilyExtension::new(
            RoadmapLaneFamily::BufferPool,
            PhysicalScenarioDriverKind::VerifierOnlyReader,
            PhysicalProofOracleKind::TranscriptPreservesEvidence,
        ))
        .unwrap()
        .with_lane_family_extension(LaneFamilyExtension::new(
            RoadmapLaneFamily::WalRecovery,
            PhysicalScenarioDriverKind::CrashInterposer,
            PhysicalProofOracleKind::TranscriptPreservesEvidence,
        ))
        .unwrap()
        .with_lane_family_extension(LaneFamilyExtension::new(
            RoadmapLaneFamily::BlobChunks,
            PhysicalScenarioDriverKind::AdversarialByteDevice,
            PhysicalProofOracleKind::TranscriptPreservesEvidence,
        ))
        .unwrap()
        .with_lane_family_extension(LaneFamilyExtension::new(
            RoadmapLaneFamily::PhysicalCertification,
            PhysicalScenarioDriverKind::VerifierOnlyReader,
            PhysicalProofOracleKind::ScenarioPlanOwnsStrategy,
        ))
        .unwrap()
}

fn happy_authority_definition() -> PhysicalScenarioDefinition {
    PhysicalScenarioDefinition::story("single_page_authority_reopen")
        .physical_substrate_lane(PhysicalSubstrateLane::HappyAuthority)
        .proves_law("record locates by physical reference after reopen")
        .step(PhysicalStoryStep::GivenCleanPhysicalStore)
        .step(PhysicalStoryStep::WhenAuthoritativeRecordIsAppended)
        .step(PhysicalStoryStep::WhenStoreClosesAndReopensFromBytes)
        .step(PhysicalStoryStep::ThenRecordLocatesByPhysicalReference)
        .requires_oracle(PhysicalProofOracleKind::VerifierRuntimeLayoutParity)
        .requires_oracle(PhysicalProofOracleKind::NoWholeStoreMaterialization)
        .define()
        .unwrap()
}

fn legacy_overclaim_definition() -> PhysicalScenarioDefinition {
    PhysicalScenarioDefinition::story("legacy_backend_overclaim_denial")
        .physical_substrate_lane(PhysicalSubstrateLane::LegacyOverclaim)
        .proves_law("legacy heap file and sqlite claims cannot become platform grade")
        .step(PhysicalStoryStep::GivenLegacyBackendClaim)
        .step(PhysicalStoryStep::WhenLegacyClaimAsksForPlatformGrade)
        .step(PhysicalStoryStep::ThenForbiddenClaimIsDenied)
        .requires_oracle(PhysicalProofOracleKind::ForbiddenLegacyPlatformClaim)
        .define()
        .unwrap()
}

fn offline_verifier_definition() -> PhysicalScenarioDefinition {
    PhysicalScenarioDefinition::story("offline_verifier_layout_parity")
        .physical_substrate_lane(PhysicalSubstrateLane::OfflineVerifier)
        .proves_law("offline verifier reads physical layout without runtime authority")
        .step(PhysicalStoryStep::GivenCleanPhysicalStore)
        .step(PhysicalStoryStep::WhenOfflineVerifierReadsManifest)
        .step(PhysicalStoryStep::ThenRuntimeVerifierParityIsPreserved)
        .requires_oracle(PhysicalProofOracleKind::VerifierRuntimeLayoutParity)
        .define()
        .unwrap()
}

fn roadmap_family_definition(family: RoadmapLaneFamily) -> PhysicalScenarioDefinition {
    PhysicalScenarioDefinition::story(format!("{}_lane_smoke", family.as_str()))
        .roadmap_lane_family(family)
        .proves_law(format!(
            "{} certification extends roadmap harness without forking",
            family.as_str()
        ))
        .step(PhysicalStoryStep::GivenCleanPhysicalStore)
        .requires_oracle(PhysicalProofOracleKind::ScenarioPlanOwnsStrategy)
        .define()
        .unwrap()
}

fn unsupported_parity_definition() -> PhysicalScenarioDefinition {
    PhysicalScenarioDefinition::story("unsupported_runtime_parity_request")
        .physical_substrate_lane(PhysicalSubstrateLane::HostileFormat)
        .proves_law("oracles deny parity when a lane lacks parity evidence")
        .step(PhysicalStoryStep::GivenHostilePhysicalBytes)
        .requires_oracle(PhysicalProofOracleKind::VerifierRuntimeLayoutParity)
        .define()
        .unwrap()
}
