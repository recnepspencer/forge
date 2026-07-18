use worth_store_test_support::harness::recovery::coverage as coverage_support;
mod shortcut_boundary_coverage;
#[path = "../readiness/shortcut_report.rs"]
mod shortcut_report;

use std::collections::BTreeSet;

use worth_store_physical_certification::{
    accept_store_owned_physical_isolation_harness_readiness,
    reject_foundational_or_proof_projection_as_physical_isolation_harness_readiness,
    reject_future_slot_as_physical_isolation_harness_readiness,
    reject_generic_runner_as_physical_isolation_harness_readiness, CounterContractKind,
    CoverageGapDenial, CoverageRowDimension, CoverageSurfaceKind, OracleFamilyKind,
    PhysicalCertificationEvidenceBundle, PhysicalDriverKind,
    PhysicalIsolationCompactionMutationKind, PhysicalIsolationCorrectnessNonClaimEvidence,
    PhysicalIsolationCounterContractReadiness, PhysicalIsolationHarnessFutureExtensionReservation,
    PhysicalIsolationHarnessFutureExtensionSlot, PhysicalIsolationHarnessReadinessDenial,
    PhysicalIsolationHarnessReadinessReceipt, PhysicalIsolationInterleavingHarnessCapability,
    PhysicalIsolationMaintenanceActorCapability, PhysicalIsolationProductionDriverCapability,
    PhysicalIsolationRequiredYieldpoint, PhysicalIsolationReusableOracleReadiness,
    PhysicalProofOracleKind, PhysicalScenarioActorRole, ShortcutRejectionObservationKind,
    SimulationPlanDenial,
};

#[test]
fn physical_isolation_receives_store_owned_simulation_harness_readiness() {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    let shortcut_report = shortcut_report::complete_shortcut_report();

    let receipt = PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &shortcut_report,
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap();

    assert_exact_set(receipt.interleaving(), expected_interleaving_capabilities());
    assert_exact_set(receipt.maintenance_actors(), expected_maintenance_actors());
    assert_exact_set(receipt.yieldpoints(), expected_yieldpoints());
    assert_exact_set(receipt.production_drivers(), expected_production_drivers());
    assert_exact_set(receipt.oracle_families(), expected_oracle_families());
    assert_exact_set(receipt.counter_contracts(), expected_counter_contracts());
    assert_eq!(
        receipt.transcript_digest(),
        evidence.primary().transcript_digest()
    );
    assert_eq!(receipt.shortcut_denial_count(), 9);
    assert_compaction_mutation_rows(&matrix);

    let accepted = accept_store_owned_physical_isolation_harness_readiness(receipt);
    assert!(accepted.does_not_claim_physical_isolation_correctness());
}

#[test]
fn readiness_shape_probe_lowers_and_executes_with_explicit_non_claim() {
    let scenario = coverage_support::scenario();
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);

    assert_eq!(plan.scenario_identity(), scenario.identity());
    assert!(plan
        .actors()
        .contains_role(PhysicalScenarioActorRole::MaintenanceReclaimer));
    assert!(plan
        .drivers()
        .contains(PhysicalDriverKind::ProductionBoundaryYieldpoint));
    assert!(plan
        .drivers()
        .contains(PhysicalDriverKind::ShortcutRejectionBoundary));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::PhysicalIsolationReadinessShape));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::TranscriptReplayEvidence));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::BlockedReclaimAttempts));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::LatchWaits));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::EpochRetries));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::ProtectedReferences));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::PublicationSwaps));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::CompactionCandidateRanges));
    assert!(plan
        .counter_contracts()
        .contains(CounterContractKind::CopiedPages));
    assert!(replay.schedule().replay_identity_matches_plan(&plan));
    assert!(replay
        .trace()
        .shortcut_rejections()
        .iter()
        .any(|observation| {
            observation.kind() == ShortcutRejectionObservationKind::PrivateMutationDenied
        }));
    assert!(replay.oracle_verdicts().iter().any(|verdict| {
        verdict.family() == OracleFamilyKind::PhysicalIsolationReadinessShape
            && verdict
                .non_claims()
                .contains(
                    &worth_store_physical_certification::PhysicalOracleNonClaim::
                        PhysicalIsolationCorrectness,
                )
    }));
    for oracle in [
        PhysicalProofOracleKind::NoMixedRoot,
        PhysicalProofOracleKind::OldReaderSeesOldRoot,
        PhysicalProofOracleKind::PostSwapReaderSeesNewRoot,
        PhysicalProofOracleKind::BlockedReclaimUntilRelease,
    ] {
        assert!(
            replay
                .oracle_verdicts()
                .iter()
                .any(|verdict| verdict.oracle() == oracle),
            "missing compaction oracle {oracle:?}"
        );
    }
    assert_counter_row(&replay, CounterContractKind::CompactionCandidateRanges, 1);
    assert_counter_row(&replay, CounterContractKind::CopiedPages, 1);
}

#[test]
fn physical_isolation_handoff_denies_near_miss_store_owned_evidence() {
    assert_eq!(
        receipt_denial_for_developer_smoke_profile(),
        PhysicalIsolationHarnessReadinessDenial::UnsupportedProfileMaturityEvidence
    );
    assert!(matches!(
        receipt_denial_for_matrix_evidence_identity_mismatch(),
        PhysicalIsolationHarnessReadinessDenial::MissingDependency(_)
    ));
}

#[test]
fn physical_isolation_handoff_dependencies_deny_before_fake_receipts_exist() {
    assert_eq!(
        coverage_support::ci_plan_without_supported_driver(
            PhysicalDriverKind::ShortcutRejectionBoundary,
        )
        .unwrap_err(),
        SimulationPlanDenial::MissingPhysicalDriver(PhysicalDriverKind::ShortcutRejectionBoundary,)
    );
    assert_eq!(
        coverage_support::ci_plan_without_supported_oracle(
            OracleFamilyKind::ForbiddenShortcutRejection,
        )
        .unwrap_err(),
        SimulationPlanDenial::MissingOracleFamily(OracleFamilyKind::ForbiddenShortcutRejection)
    );
    assert_eq!(
        matrix_denial_for_missing_private_mutation_observation(),
        CoverageGapDenial::MissingMutationResult
    );
}

#[test]
fn generic_runners_and_future_slots_cannot_satisfy_physical_isolation_readiness() {
    assert_eq!(
        reject_generic_runner_as_physical_isolation_harness_readiness().unwrap_err(),
        PhysicalIsolationHarnessReadinessDenial::GenericRunnerCannotSatisfyReadiness
    );
    assert_eq!(
        reject_future_slot_as_physical_isolation_harness_readiness(
            PhysicalIsolationHarnessFutureExtensionReservation::reserved(
                PhysicalIsolationHarnessFutureExtensionSlot::BlobLifecycle,
            ),
        )
        .unwrap_err(),
        PhysicalIsolationHarnessReadinessDenial::FutureBehaviorSlotCannotSatisfyReadiness
    );
    assert_eq!(
        reject_foundational_or_proof_projection_as_physical_isolation_harness_readiness()
            .unwrap_err(),
        PhysicalIsolationHarnessReadinessDenial::FoundationalOrProofProjectionCannotSatisfyReadiness
    );
}

fn receipt_denial_for_developer_smoke_profile() -> PhysicalIsolationHarnessReadinessDenial {
    let plan = coverage_support::lowered_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &shortcut_report::complete_shortcut_report(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap_err()
}

fn matrix_denial_for_missing_private_mutation_observation() -> CoverageGapDenial {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle_without_mutation_denial(&plan);
    coverage_support::mutation_evidence_denial(&replay)
}

fn receipt_denial_for_matrix_evidence_identity_mismatch() -> PhysicalIsolationHarnessReadinessDenial
{
    let matrix_plan = coverage_support::lowered_ci_plan();
    let matrix_replay = coverage_support::replay_bundle(&matrix_plan);
    let matrix = coverage_support::complete_registry(&matrix_plan, &matrix_replay)
        .generate_matrix()
        .unwrap();
    let evidence_plan = coverage_support::shortcut_plan();
    let evidence = coverage_support::evidence_bundle_without_compaction_mutations(&evidence_plan);
    PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &shortcut_report::complete_shortcut_report(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap_err()
}

fn expected_interleaving_capabilities() -> [PhysicalIsolationInterleavingHarnessCapability; 12] {
    [
        PhysicalIsolationInterleavingHarnessCapability::DeterministicReplaySchedule,
        PhysicalIsolationInterleavingHarnessCapability::ProtectBeforeObserveShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::RootKindSeparationShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::TraversalAdmissionShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::ByteGuardUsageShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::NoHiddenLatchIoShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::PublicationMemoryOrderingShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::LeaseExpiryNonAuthorityShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::FreeReuseGenerationFenceShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::RestartDuringCutoverShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::ReadDuringCompactionShapeProbe,
        PhysicalIsolationInterleavingHarnessCapability::CompactionRangeInterlockShapeProbe,
    ]
}

fn expected_maintenance_actors() -> [PhysicalIsolationMaintenanceActorCapability; 3] {
    [
        PhysicalIsolationMaintenanceActorCapability::ReclaimBarrierParticipant,
        PhysicalIsolationMaintenanceActorCapability::RestartParticipant,
        PhysicalIsolationMaintenanceActorCapability::CompactionCutoverParticipant,
    ]
}

fn expected_yieldpoints() -> [PhysicalIsolationRequiredYieldpoint; 7] {
    [
        PhysicalIsolationRequiredYieldpoint::RootPublicationBeforeObserve,
        PhysicalIsolationRequiredYieldpoint::RootSwapPublication,
        PhysicalIsolationRequiredYieldpoint::ByteGuardAdmission,
        PhysicalIsolationRequiredYieldpoint::ReclaimBarrier,
        PhysicalIsolationRequiredYieldpoint::RestartDuringCutover,
        PhysicalIsolationRequiredYieldpoint::CompactionCutover,
        PhysicalIsolationRequiredYieldpoint::ShortcutRejectionBoundary,
    ]
}

fn expected_production_drivers() -> [PhysicalIsolationProductionDriverCapability; 2] {
    [
        PhysicalIsolationProductionDriverCapability::ProductionBoundaryYieldpoint,
        PhysicalIsolationProductionDriverCapability::ShortcutRejectionBoundary,
    ]
}

fn expected_oracle_families() -> [PhysicalIsolationReusableOracleReadiness; 3] {
    [
        PhysicalIsolationReusableOracleReadiness::PhysicalIsolationReadinessShape,
        PhysicalIsolationReusableOracleReadiness::TranscriptReplayEvidence,
        PhysicalIsolationReusableOracleReadiness::ForbiddenShortcutRejection,
    ]
}

fn expected_counter_contracts() -> [PhysicalIsolationCounterContractReadiness; 12] {
    [
        PhysicalIsolationCounterContractReadiness::ActorStepExact,
        PhysicalIsolationCounterContractReadiness::ReplayIdentityExact,
        PhysicalIsolationCounterContractReadiness::ForbiddenShortcutExact,
        PhysicalIsolationCounterContractReadiness::ProfileResourceEnvelope,
        PhysicalIsolationCounterContractReadiness::LatchWaits,
        PhysicalIsolationCounterContractReadiness::EpochRetries,
        PhysicalIsolationCounterContractReadiness::ProtectedReferences,
        PhysicalIsolationCounterContractReadiness::BlockedReclaimAttempts,
        PhysicalIsolationCounterContractReadiness::PublicationSwaps,
        PhysicalIsolationCounterContractReadiness::FutureS5SpecificCountersReserved,
        PhysicalIsolationCounterContractReadiness::CompactionCandidateRanges,
        PhysicalIsolationCounterContractReadiness::CopiedPages,
    ]
}

fn assert_exact_set<T>(actual: &[T], expected: impl IntoIterator<Item = T>)
where
    T: Copy + Ord + std::fmt::Debug,
{
    let actual = actual.iter().copied().collect::<BTreeSet<_>>();
    let expected = expected.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_counter_row(
    replay: &worth_store_physical_certification::SimulationReplayBundle,
    kind: CounterContractKind,
    observed_count: u64,
) {
    assert!(replay
        .counter_receipt()
        .rows()
        .iter()
        .any(|row| row.kind() == kind && row.observed_count() == observed_count));
}

fn assert_compaction_mutation_rows(
    matrix: &worth_store_physical_certification::GeneratedCoverageMatrix,
) {
    for kind in PhysicalIsolationCompactionMutationKind::REQUIRED_FOR_S5_INTERLEAVING {
        assert!(
            matrix.rows().iter().any(|row| {
                row.surface() == CoverageSurfaceKind::MutationResult
                    && row.has_dimension(&CoverageRowDimension::CompactionMutation(kind))
            }),
            "missing compaction mutation row {kind:?}"
        );
    }
}
