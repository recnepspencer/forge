use crate::{
    lower_blob_simulation_seed_plan, lower_physical_simulation_plan, BlobHarnessProfile,
    BlobHarnessProfileSet, BlobHarnessScenarioSeed, BlobHarnessShortcutAttempt,
    BlobHarnessShortcutDenial, CounterContractKind, CoverageRowDimension,
    PhysicalSimulationCapability, PhysicalSimulationScenarioFamily, SimulationPlanDenial,
    SimulationPlanningContext,
};
use worth_store_blob_chunks::{
    deny_ambient_chaos_corpus_as_canonical, deny_hidden_temporary_sidecar,
    deny_logical_size_only_heavy_qualification, deny_sparse_only_heavy_qualification,
    deny_whole_object_expected_buffer, BlobHarnessAccessMode, BlobHarnessActorMix,
    BlobHarnessChunkSizeClass, BlobHarnessChunkTopology, BlobHarnessFailurePoint,
    BlobHarnessPlacementClass, BlobHarnessSecurityScopeClass, BlobHarnessSizeClass,
    BlobHarnessTopologyDenial, HeavyBlobFixtureMaterializationMode,
};
use worth_store_budgets::BlobHarnessEnvelopeProfile;
use worth_store_test_support::{
    execute_blob_harness_real_multi_gb_temp_file_fixture,
    execute_blob_harness_temp_file_fixture_smoke,
};

use super::{
    heavy_qualification::canonical_heavy_fixture_plan_for_seed,
    synthetic_blob_harness_coverage_matrix_for_test_support,
    synthetic_blob_harness_replay_bundle_for_test_support,
};

#[test]
fn seed_blob_scenario_lowers_to_stable_simulation_harness_plan_identity() {
    let seed = BlobHarnessScenarioSeed::builder()
        .profile(BlobHarnessProfile::ci_memory_envelope_exceeding())
        .placement_external()
        .security_scope_preserving()
        .read_only_access()
        .seed_actor_mix()
        .build()
        .unwrap();

    let first = lower_blob_simulation_seed_plan(seed.clone()).unwrap();
    let second = lower_blob_simulation_seed_plan(seed).unwrap();

    assert_ne!(first.replay_identity(), &[0; 32]);
    assert_eq!(first.replay_identity(), second.replay_identity());
    assert_eq!(
        first.materialized_profile().foundational_identity(),
        second.materialized_profile().foundational_identity()
    );
    assert_eq!(
        first.plan().scenario_family(),
        PhysicalSimulationScenarioFamily::BlobHarnessSeed
    );
    assert!(first
        .plan()
        .counter_contracts()
        .contains(CounterContractKind::BlobChunkCountExact));
    assert!(first
        .plan()
        .counter_contracts()
        .contains(CounterContractKind::BlobLogicalBytesExact));
}

#[test]
fn seed_blob_scenario_shape_is_gated_by_ordinary_simulation_harness_lowerer() {
    let seed = BlobHarnessScenarioSeed::builder()
        .profile(BlobHarnessProfile::local())
        .placement_external()
        .security_scope_preserving()
        .read_only_access()
        .seed_actor_mix()
        .build()
        .unwrap();
    let lowered = lower_blob_simulation_seed_plan(seed).unwrap();

    let denial = lower_physical_simulation_plan(
        lowered.scenario().clone(),
        SimulationPlanningContext::developer_smoke(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        SimulationPlanDenial::MissingCapability(
            PhysicalSimulationCapability::ProductionBoundaryDriver
        )
    );
}

#[test]
fn local_ci_and_heavy_profiles_share_blob_counter_topology() {
    let mut observed = Vec::new();
    for profile in BlobHarnessProfileSet::required_qualification_profiles().iter() {
        if profile != BlobHarnessProfile::local() {
            assert!(profile.envelope().exceeds_resident_memory_envelope());
        }
        let seed = BlobHarnessScenarioSeed::builder()
            .profile(profile)
            .placement_external()
            .security_scope_preserving()
            .read_only_access()
            .seed_actor_mix()
            .build()
            .unwrap();
        let lowered = lower_blob_simulation_seed_plan(seed).unwrap();
        observed.push(
            lowered
                .plan()
                .counter_contracts()
                .iter()
                .map(|contract| contract.kind())
                .collect::<Vec<_>>(),
        );
    }

    assert_eq!(observed[0], observed[1]);
    assert_eq!(observed[1], observed[2]);
}

#[test]
fn profiles_materialize_foundational_identity_for_each_blob_profile() {
    for profile in BlobHarnessProfileSet::required_qualification_profiles().iter() {
        let seed = BlobHarnessScenarioSeed::builder()
            .profile(profile)
            .placement_external()
            .security_scope_preserving()
            .read_only_access()
            .seed_actor_mix()
            .build()
            .unwrap();
        let lowered = lower_blob_simulation_seed_plan(seed).unwrap();
        let materialized = lowered.materialized_profile();
        assert_eq!(materialized.blob_profile(), profile);
        assert_ne!(
            materialized
                .foundational_identity()
                .digest()
                .value()
                .bytes(),
            &[0; 32]
        );
        assert_eq!(
            materialized
                .materialized()
                .payload()
                .materialized()
                .certification_posture(),
            worth_foundational::CertificationPostureProfile::EvidenceBacked
        );
    }
}

#[test]
fn shortcut_lanes_are_typed_denials() {
    assert_eq!(
        BlobHarnessChunkTopology::from_classes(
            BlobHarnessSizeClass::TinyShortcut,
            BlobHarnessChunkSizeClass::Fixed64KiB,
        )
        .unwrap_err(),
        BlobHarnessTopologyDenial::TinyBlobShortcut
    );
    assert_eq!(
        BlobHarnessScenarioSeed::builder()
            .tiny_blob_shortcut()
            .build()
            .unwrap_err(),
        BlobHarnessShortcutDenial::TinyBlobCannotSatisfyProfileEnvelope
    );
    assert_eq!(
        BlobHarnessScenarioSeed::builder()
            .without_chunk_counters()
            .build()
            .unwrap_err(),
        BlobHarnessShortcutDenial::MissingChunkCounters
    );
    assert_eq!(
        BlobHarnessShortcutAttempt::whole_object_helper().deny_for_blob_harness(),
        BlobHarnessShortcutDenial::WholeObjectHelperNotHarnessAuthority
    );
    assert_eq!(
        BlobHarnessShortcutAttempt::logs_as_proof().deny_for_blob_harness(),
        BlobHarnessShortcutDenial::LogsAreNotProof
    );
    assert_eq!(
        BlobHarnessShortcutAttempt::synthetic_success_row().deny_for_blob_harness(),
        BlobHarnessShortcutDenial::SyntheticSuccessRowNotEvidence
    );
    assert_eq!(
        BlobHarnessShortcutAttempt::private_harness_state_mutation().deny_for_blob_harness(),
        BlobHarnessShortcutDenial::PrivateMutationNotHarnessAuthority
    );
}

#[test]
#[ignore = "release-scale blob qualification"]
fn replay_bundle_is_stable_and_carries_exact_blob_counter_evidence() {
    let seed = BlobHarnessScenarioSeed::builder()
        .profile(BlobHarnessProfile::ci_memory_envelope_exceeding())
        .placement_external()
        .security_scope_preserving()
        .read_only_access()
        .placement_move_partial_replication_actor_mix()
        .fault_during_tier_move()
        .build()
        .unwrap();

    let first = synthetic_blob_harness_replay_bundle_for_test_support(seed.clone());
    let second = synthetic_blob_harness_replay_bundle_for_test_support(seed.clone());

    assert_eq!(first.plan().identity(), second.plan().identity());
    assert_eq!(first.schedule().identity(), second.schedule().identity());
    assert_eq!(first.transcript_identity(), second.transcript_identity());
    assert_eq!(first.oracle_verdicts(), second.oracle_verdicts());
    assert_eq!(
        first.counter_receipt().rows(),
        second.counter_receipt().rows()
    );
    assert_eq!(
        first
            .counter_receipt()
            .rows()
            .iter()
            .find(|row| row.kind() == CounterContractKind::BlobChunkCountExact)
            .map(|row| row.observed_count()),
        Some(seed.topology().chunk_count())
    );
    assert_eq!(
        first
            .counter_receipt()
            .rows()
            .iter()
            .find(|row| row.kind() == CounterContractKind::BlobLogicalBytesExact)
            .map(|row| row.observed_count()),
        Some(seed.topology().logical_bytes())
    );
}

#[test]
fn coverage_rows_include_blob_axes() {
    let seed = BlobHarnessScenarioSeed::builder()
        .profile(BlobHarnessProfile::heavy_multi_gb())
        .placement_cold_tier()
        .security_scope_cross_scope_denied()
        .partial_replication_access()
        .placement_move_partial_replication_actor_mix()
        .fault_during_tier_move()
        .build()
        .unwrap();

    let coverage = synthetic_blob_harness_coverage_matrix_for_test_support(seed.clone());
    let rows = coverage.rows();

    assert!(rows.iter().any(|row| {
        row.has_dimension(&CoverageRowDimension::BlobSizeClass(
            BlobHarnessSizeClass::HeavyMultiGbDeclared,
        )) && row.has_dimension(&CoverageRowDimension::BlobChunkCount(
            seed.topology().chunk_count(),
        )) && row.has_dimension(&CoverageRowDimension::BlobChunkSizeClass(
            BlobHarnessChunkSizeClass::Fixed8MiB,
        )) && row.has_dimension(&CoverageRowDimension::BlobSecurityScopeClass(
            BlobHarnessSecurityScopeClass::CrossScopeDenied,
        )) && row.has_dimension(&CoverageRowDimension::BlobPlacementClass(
            BlobHarnessPlacementClass::ColdTierObserved,
        )) && row.has_dimension(&CoverageRowDimension::BlobAccessMode(
            BlobHarnessAccessMode::PartialReplication,
        )) && row.has_dimension(&CoverageRowDimension::BlobFailurePoint(
            BlobHarnessFailurePoint::DuringTierMove,
        )) && row.has_dimension(&CoverageRowDimension::BlobMemoryEnvelopeProfile(
            BlobHarnessEnvelopeProfile::HeavyMultiGb,
        )) && row.has_dimension(&CoverageRowDimension::BlobActorMix(
            BlobHarnessActorMix::PlacementMovePartialReplication,
        ))
    }));
}

#[test]
fn canonical_heavy_fixture_identity_binds_basis() {
    let seed = heavy_multi_gb_scenario_seed();
    let plan = canonical_heavy_fixture_plan_for_seed(
        &seed,
        HeavyBlobFixtureMaterializationMode::StreamOnly,
    )
    .expect("heavy plan should admit");
    let digest_basis = plan.expected_digest_basis();

    assert_eq!(digest_basis.seed(), 22);
    assert_eq!(
        digest_basis.logical_bytes(),
        seed.topology().logical_bytes()
    );
    assert_eq!(digest_basis.chunk_bytes(), seed.topology().chunk_bytes());
    assert_eq!(
        digest_basis.expected_chunk_count(),
        seed.topology().chunk_count()
    );
    assert_eq!(
        plan.materialization_mode(),
        HeavyBlobFixtureMaterializationMode::StreamOnly
    );
    assert_eq!(
        plan.backend_profile(),
        worth_store_physical_backend::HeavyFixtureBackendProfile::StoreOwnedLocalDisk
    );
}

#[test]
fn temp_file_materialization_keeps_schema_and_emits_cleanup_receipt() {
    let witness = execute_blob_harness_temp_file_fixture_smoke();
    let evidence = witness
        .heavy_fixture_evidence()
        .expect("temp file fixture evidence");
    let cleanup = evidence.cleanup_receipt().expect("cleanup receipt");
    let preflight = evidence
        .disk_preflight_receipt()
        .expect("disk preflight receipt");

    assert_eq!(
        evidence.materialization_mode(),
        HeavyBlobFixtureMaterializationMode::TempFile
    );
    assert!(evidence.temporary_file_bytes() > 0);
    assert_eq!(
        evidence.temporary_file_bytes(),
        evidence.disk_bytes_written()
    );
    assert!(preflight.available_bytes() >= preflight.required_bytes());
    assert!(cleanup.completed());
    assert!(!cleanup.path().exists());
}

#[test]
fn hostile_lanes_are_typed_denials() {
    assert_eq!(
        deny_sparse_only_heavy_qualification(),
        worth_store_blob_chunks::HeavyBlobQualificationDenial::SparseOnlyProofNotCanonical
    );
    assert_eq!(
        deny_logical_size_only_heavy_qualification(),
        worth_store_blob_chunks::HeavyBlobQualificationDenial::LogicalSizeOnlyProofNotCanonical
    );
    assert_eq!(
        deny_whole_object_expected_buffer(),
        worth_store_blob_chunks::HeavyBlobQualificationDenial::WholeObjectExpectedBufferNotCanonical
    );
    assert_eq!(
        deny_hidden_temporary_sidecar(),
        worth_store_blob_chunks::HeavyBlobQualificationDenial::HiddenTemporarySidecarNotCanonical
    );
    assert_eq!(
        deny_ambient_chaos_corpus_as_canonical(),
        worth_store_blob_chunks::HeavyBlobQualificationDenial::AmbientCorpusNotCanonical
    );
}

#[test]
#[ignore = "release-scale blob qualification"]
fn real_multi_gb_temp_file_fixture_emits_heavy_topology_evidence() {
    let witness = execute_blob_harness_real_multi_gb_temp_file_fixture();
    let evidence = witness
        .heavy_fixture_evidence()
        .expect("heavy fixture evidence");

    assert_eq!(
        witness.executed_topology().logical_bytes(),
        BlobHarnessSizeClass::HeavyMultiGbDeclared.declared_logical_bytes()
    );
    assert_eq!(
        evidence.expected_digest_basis().expected_chunk_count(),
        witness.executed_topology().chunk_count()
    );
    assert!(evidence.peak_allocation_count() > 0);
    assert_eq!(
        evidence.temporary_file_bytes(),
        evidence.disk_bytes_written()
    );
    assert!(evidence.cleanup_receipt().expect("cleanup").completed());
}

fn heavy_multi_gb_scenario_seed() -> BlobHarnessScenarioSeed {
    BlobHarnessScenarioSeed::builder()
        .profile(BlobHarnessProfile::heavy_multi_gb())
        .placement_external()
        .security_scope_preserving()
        .read_only_access()
        .seed_actor_mix()
        .build()
        .unwrap()
}
