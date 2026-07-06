use crate::{
    lower_blob_simulation_seed_plan, lower_physical_simulation_plan, BlobHarnessProfile,
    BlobHarnessProfileSet, BlobHarnessScenarioSeed, BlobHarnessShortcutAttempt,
    BlobHarnessShortcutDenial, CounterContractKind, PhysicalSimulationCapability,
    PhysicalSimulationScenarioFamily, SimulationPlanDenial, SimulationPlanningContext,
};
use forge_store_blob_chunks::{
    BlobHarnessChunkSizeClass, BlobHarnessChunkTopology, BlobHarnessSizeClass,
    BlobHarnessTopologyDenial,
};

#[test]
fn seed_blob_scenario_lowers_to_stable_s45_plan_identity() {
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
        PhysicalSimulationScenarioFamily::S7BlobHarnessSeed
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
fn seed_blob_scenario_shape_is_gated_by_ordinary_s45_lowerer() {
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
    for profile in BlobHarnessProfileSet::phase8_required().iter() {
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
    for profile in BlobHarnessProfileSet::phase8_required().iter() {
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
            forge_foundational::CertificationPostureProfile::EvidenceBacked
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
