#[path = "../../../support/recovery/counter_strength/support.rs"]
mod counter_support;

use forge_foundational::{BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator};
use forge_store_physical_backend::ProductionStorageBoundarySeam;
use forge_store_physical_certification::PhysicalProofOracleVerdict;
use forge_store_physical_certification::{
    CounterContractOracle, DetachedSimulationReplayParts, ExecutedTranscriptParts,
    FixtureCapabilityDeclaration, FixtureMutationBoundary, IndependentVerifierObservation,
    LargeStoreFixtureProfile, ObservedPhysicalTrace, PhysicalArtifactFaultLocus,
    PhysicalCertificationEvidenceBundle, PhysicalFaultEvent, PhysicalFixtureBuilder,
    PhysicalInterleavingSchedule, PhysicalProofOracleKind, PhysicalSimulationObserver,
    PhysicalSimulationPlan, ProductionBackedPhysicalFixture, ReusablePhysicalOracleFamily,
    SimulationReplayBundle, StateSpaceBudget, TranscriptReplayDenial,
};
use forge_store_test_support::{
    developer_smoke_replay_seed, production_backed_physical_fixture_materialization,
};

#[test]
fn executed_transcript_replays_without_live_runtime_state() {
    let plan = counter_support::lower_physical_isolation_plan();
    let fixture = production_fixture();
    let transcript =
        forge_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            executed_parts_for_seed(&plan, developer_smoke_replay_seed())
                .with_faults([storage_no_fault_control(10)])
                .with_transcript_replay_verdict()
                .unwrap(),
        )
        .unwrap();
    let transcript_identity = transcript.identity().clone();
    let replay_basis_identity = transcript.replay_basis_identity().clone();
    let detached = DetachedSimulationReplayParts::from_transcript(&transcript);
    drop(transcript);

    let replay = detached.admit_replay_bundle().unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();

    assert_eq!(
        evidence.replay().transcript_identity(),
        &transcript_identity
    );
    assert_eq!(
        evidence.replay().replay_basis_identity(),
        &replay_basis_identity
    );
    assert_eq!(
        evidence.replay().plan().identity(),
        evidence.replay().trace().plan_identity()
    );
    assert_eq!(
        evidence.replay().fixture_manifest().semantic_digest(),
        fixture.manifest().semantic_digest()
    );
    assert_eq!(
        evidence.replay().schedule().seed(),
        developer_smoke_replay_seed()
    );
    assert_eq!(evidence.replay().fault_events().len(), 1);
    assert!(!evidence
        .replay()
        .plan()
        .driver_contracts()
        .iter()
        .flat_map(|driver| driver.profile().evidence_surfaces())
        .collect::<Vec<_>>()
        .is_empty());
    assert_eq!(evidence.replay().oracle_verdicts().len(), 2);
    assert!(evidence
        .replay()
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay));
    assert!(
        evidence
            .replay()
            .transcript_identity()
            .canonical_basis_entry_count()
            > 0
    );
}

#[test]
fn detached_replay_admission_denies_copied_schedule_authority() {
    let plan = counter_support::lower_physical_isolation_plan();
    let alternate_plan = counter_support::lower_physical_isolation_plan_for_profile(
        forge_store_physical_certification::PhysicalSimulationProfile::CiCertification,
    );
    let transcript =
        forge_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            executed_parts_for_seed(&plan, developer_smoke_replay_seed())
                .with_transcript_replay_verdict()
                .unwrap(),
        )
        .unwrap();
    let copied_schedule = schedule(
        &alternate_plan,
        forge_store_physical_certification::ReplaySeed::from_u64(
            developer_smoke_replay_seed().value() + 1,
        ),
    );
    let detached = DetachedSimulationReplayParts::from_transcript(&transcript)
        .with_replayed_schedule_candidate(copied_schedule);
    drop(transcript);

    let denial = detached.admit_replay_bundle().unwrap_err();

    assert_eq!(denial, TranscriptReplayDenial::PlanScheduleIdentityMismatch);
}

#[test]
fn transcript_identity_changes_when_replay_seed_changes() {
    let plan = counter_support::lower_physical_isolation_plan();
    let first = replay_bundle_for_seed(&plan, developer_smoke_replay_seed());
    let second = replay_bundle_for_seed(
        &plan,
        forge_store_physical_certification::ReplaySeed::from_u64(
            developer_smoke_replay_seed().value() + 1,
        ),
    );

    assert_ne!(
        first.transcript_identity().digest_bytes(),
        second.transcript_identity().digest_bytes()
    );
}

#[test]
fn transcript_identity_changes_when_runtime_verifier_or_fault_detail_changes() {
    let plan = counter_support::lower_physical_isolation_plan();
    let base = replay_bundle_for_seed(&plan, developer_smoke_replay_seed());
    let verifier = replay_bundle_from_parts(executed_parts_for_seed_with_trace(
        &plan,
        developer_smoke_replay_seed(),
        observed_trace_with_verifier(&plan),
    ));
    let first_fault =
        replay_bundle_from_parts(executed_parts(&plan).with_faults([storage_no_fault_control(11)]));
    let second_fault =
        replay_bundle_from_parts(executed_parts(&plan).with_faults([storage_no_fault_control(12)]));

    assert_ne!(
        base.transcript_identity().digest_bytes(),
        verifier.transcript_identity().digest_bytes()
    );
    assert_ne!(
        first_fault.transcript_identity().digest_bytes(),
        second_fault.transcript_identity().digest_bytes()
    );
}

#[test]
fn evidence_materializes_foundational_packaging_without_authority_promotion() {
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay_bundle_for_seed(
        &counter_support::lower_physical_isolation_plan(),
        developer_smoke_replay_seed(),
    ))
    .unwrap();

    let foundational = evidence.materialize_foundational_evidence();

    assert!(foundational.materialized().report().is_some());
    assert!(foundational.materialized().receipt().is_some());
    assert_eq!(
        foundational.materialized().source(),
        forge_foundational::FoundationalBoundaryMaterializationSource::NativeAuthority
    );
    assert_eq!(
        forge_store_physical_certification::reject_foundational_materialization_as_store_authority(
        )
        .unwrap_err(),
        forge_store_physical_certification::PhysicalEvidenceBundleDenial::FoundationalMaterializationIsNotStoreAuthority
    );
}

#[test]
fn boundary_bridged_foundational_evidence_requires_explicit_store_readmission() {
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay_bundle_for_seed(
        &counter_support::lower_physical_isolation_plan(),
        developer_smoke_replay_seed(),
    ))
    .unwrap();

    let bridged = evidence
        .materialize_foundational_evidence()
        .bridge_trust_boundary();
    let readmitted =
        forge_store_physical_certification::readmit_foundational_physical_evidence_after_boundary(
            bridged,
        );

    assert!(readmitted.supports_later_certification_comparison());
    assert!(readmitted.materialized().report().is_some());
    assert_eq!(
        forge_store_physical_certification::reject_foundational_materialization_as_store_authority(
        )
        .unwrap_err(),
        forge_store_physical_certification::PhysicalEvidenceBundleDenial::FoundationalMaterializationIsNotStoreAuthority
    );
}

fn replay_bundle_for_seed(
    plan: &PhysicalSimulationPlan,
    seed: forge_store_physical_certification::ReplaySeed,
) -> SimulationReplayBundle {
    replay_bundle_from_parts(executed_parts_for_seed(plan, seed))
}

fn replay_bundle_from_parts(parts: ExecutedTranscriptParts) -> SimulationReplayBundle {
    let transcript =
        forge_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            parts.with_transcript_replay_verdict().unwrap(),
        )
        .unwrap();
    let detached = DetachedSimulationReplayParts::from_transcript(&transcript);
    drop(transcript);
    detached.admit_replay_bundle().unwrap()
}

fn executed_parts(plan: &PhysicalSimulationPlan) -> ExecutedTranscriptParts {
    executed_parts_for_seed(plan, developer_smoke_replay_seed())
}

fn executed_parts_for_seed(
    plan: &PhysicalSimulationPlan,
    seed: forge_store_physical_certification::ReplaySeed,
) -> ExecutedTranscriptParts {
    let trace = counter_support::observed_trace(plan);
    let counter_receipt = counter_support::counter_receipt(plan, trace.clone());
    let readiness_verdict = physical_isolation_readiness_verdict(plan, &trace);
    ExecutedTranscriptParts::new(
        plan,
        schedule(plan, seed),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_oracle_verdict(readiness_verdict)
}

fn executed_parts_for_seed_with_trace(
    plan: &PhysicalSimulationPlan,
    seed: forge_store_physical_certification::ReplaySeed,
    trace: ObservedPhysicalTrace,
) -> ExecutedTranscriptParts {
    let counter_receipt = counter_support::counter_receipt(plan, trace.clone());
    let readiness_verdict = physical_isolation_readiness_verdict(plan, &trace);
    ExecutedTranscriptParts::new(
        plan,
        schedule(plan, seed),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_oracle_verdict(readiness_verdict)
}

fn physical_isolation_readiness_verdict(
    plan: &PhysicalSimulationPlan,
    trace: &ObservedPhysicalTrace,
) -> PhysicalProofOracleVerdict {
    ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(CounterContractOracle)
        .judge(plan, trace)
        .unwrap()
}

fn observed_trace_with_verifier(plan: &PhysicalSimulationPlan) -> ObservedPhysicalTrace {
    let execution =
        forge_store_physical_certification::ExecutedPhysicalSimulationObservation::from_executed_plan(
            plan,
        )
        .unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_independent_verifier_observation(IndependentVerifierObservation::agreement(
            forge_store_physical_certification::OfflineVerifierBoundarySeam::LayoutWalkBeforeRuntimeRecovery,
        ))
        .with_compaction_interlock_observation(counter_support::compaction_observation())
        .complete()
        .unwrap()
}

fn storage_no_fault_control(artifact_id: u64) -> PhysicalFaultEvent {
    PhysicalFaultEvent::no_fault_control(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        PhysicalArtifactFaultLocus::root_pointer(
            BoundaryArtifactLocator::new(
                BoundaryArtifactId::new(artifact_id),
                BoundaryArtifactField::Basis,
            ),
            forge_store_physical_certification::ExpectedFaultLocalization::ProductionDriverBoundary,
        ),
    )
    .unwrap()
}

fn schedule(
    plan: &PhysicalSimulationPlan,
    seed: forge_store_physical_certification::ReplaySeed,
) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        seed,
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap()
}

fn production_fixture() -> ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase10-transcript-fixture")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                10,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}
