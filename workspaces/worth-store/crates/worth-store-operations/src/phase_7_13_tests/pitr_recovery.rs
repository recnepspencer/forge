use super::*;
use crate::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, OperationalOperationId,
    OperationalTransitionId, PointInTimeRecoveryIntent,
};
use worth_store_physical_isolation::RecoverySourceLeaseRegistry;
use worth_store_recovery_physics::{
    PitrCandidatePosture, PitrRoundingPolicy, RecoveryPhysicsTimelineAuthority,
};

#[test]
fn exact_frontier_pitr_executes_only_after_a_durable_source_lease() {
    let world = restore_world("phase-9-exact-pitr");
    let target = world.restore_directory.path().join("pitr-target");
    let leases = RecoverySourceLeaseRegistry::open(
        world.restore_directory.path().join("pitr-source-leases"),
    )
    .unwrap();
    std::fs::create_dir_all(&target).unwrap();
    let materialized = world.admissible.custody().structural().materialized();
    let manifest = materialized.manifest();
    let wal_end = manifest.wal_half_open_interval().1;
    let selected_client_ack = manifest.acknowledged_frontier();
    let observation = RecoveryPhysicsTimelineAuthority::admit_observation(
        100,
        0,
        0,
        manifest.durable_checkpoint_lsn(),
        wal_end,
        wal_end,
        selected_client_ack,
        manifest.acknowledged_frontier(),
        world.admissible.admission().admitting_authority(),
        [0x91; 32],
        materialized.manifest_digest(),
        PitrCandidatePosture::Available,
    )
    .unwrap();
    let candidates = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::ExactOnly,
        vec![observation],
    )
    .unwrap();
    let security_scope = recovery_security_scope(&world.admissible);
    let lowered = PointInTimeRecoveryIntent::near(
        OperationalOperationId::new("pitr-exact-frontier").unwrap(),
        world.admissible,
        candidates,
        &target,
        security_scope,
        u64::MAX,
        31,
    )
    .resolve()
    .expect("select exact frontier")
    .admit_source_cut(&leases)
    .expect("durably admit exact source closure")
    .lease()
    .lower()
    .expect("lower leased PITR owner DAG");
    assert_recovery_lifecycle_dag(lowered.explanation());
    let expected = lowered
        .authorize(
            &ExactAuthorizationPort {
                substitute_plan: None,
            },
            &operator_assertion(),
            20,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 20 },
        )
        .unwrap()
        .ready(
            &world.control,
            OperationalTransitionId::new("consume-pitr-staging").unwrap(),
            &world.authority,
            21,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
        )
        .unwrap()
        .execute(&CurrentStagingAuthorizationPort)
        .expect("execute exact-frontier PITR");
    assert_eq!(
        expected
            .receipt()
            .recovery()
            .exact_frontier()
            .wal_structural(),
        wal_end
    );
    assert_eq!(
        expected
            .receipt()
            .recovery()
            .exact_frontier()
            .client_acknowledged(),
        selected_client_ack
    );
    assert!(expected
        .staged_media()
        .root()
        .starts_with(std::fs::canonicalize(&target).unwrap()));
    assert_eq!(
        selected_staging_kind(&world.authority, &world.control),
        crate::RecoveryStagingOperationKind::PointInTimeRecovery
    );
    let verified = expected
        .post_verify(verification_budget())
        .expect("post-verification preserves the selected client-ack frontier");
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        world.authority.identity().clone(),
    );
    let roots = worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        "pitr-full-cutover",
        101,
    );
    let publication_directory = tempfile::tempdir().unwrap();
    let current_frontier = crate::RecoveryAuthorityFrontier::observed(
        &world.authority,
        10,
        12,
        20,
        19,
        18,
        [0xa1; 32],
    )
    .unwrap();
    let current = crate::CurrentRecoveryAuthoritySnapshot::observe(
        &world.authority,
        publication_directory.path(),
        roots.old_candidate,
        roots.old_reachability,
        current_frontier,
    )
    .unwrap();
    let policy = worth_store_authority::RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
        verified.authority_posture(),
        [0xa2; 32],
    )
    .unwrap();
    let resolved = verified.resolve_cutover(current, policy).unwrap();
    assert_eq!(resolved.authority_delta().local_durable_loss(), 8);
    let readmitted = resolved
        .lower_cutover(&world.authority)
        .unwrap()
        .authorize(
            &ExactAuthorizationPort {
                substitute_plan: None,
            },
            &operator_assertion(),
            30,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 30 },
        )
        .unwrap()
        .establish_write_fence(
            &world.control,
            OperationalTransitionId::new("consume-pitr-cutover").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap()
        .publish(
            &world.control,
            OperationalTransitionId::new("publish-pitr-root").unwrap(),
        )
        .unwrap()
        .readmit(
            &world.control,
            OperationalTransitionId::new("readmit-pitr-root").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
        )
        .unwrap();
    let released = readmitted.release_source_lease().unwrap();
    assert_ne!(released.lease_identity(), [0; 32]);
    assert!(leases.recover_active().unwrap().is_empty());
}

#[test]
fn pitr_rejects_timeline_evidence_from_a_different_store_authority() {
    let world = restore_world("phase-9-foreign-authority");
    let materialized = world.admissible.custody().structural().materialized();
    let manifest = materialized.manifest();
    let wal_end = manifest.wal_half_open_interval().1;
    let foreign = worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint(
        [0xe1; 32],
    );
    let observation = RecoveryPhysicsTimelineAuthority::admit_observation(
        100,
        0,
        0,
        manifest.durable_checkpoint_lsn(),
        wal_end,
        wal_end,
        manifest.acknowledged_frontier(),
        manifest.acknowledged_frontier(),
        foreign,
        [0x91; 32],
        materialized.manifest_digest(),
        PitrCandidatePosture::Available,
    )
    .unwrap();
    let candidates = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::ExactOnly,
        vec![observation],
    )
    .unwrap();
    let target = world
        .restore_directory
        .path()
        .join("foreign-authority-target");
    let security_scope = recovery_security_scope(&world.admissible);
    let denial = PointInTimeRecoveryIntent::near(
        OperationalOperationId::new("pitr-foreign-authority").unwrap(),
        world.admissible,
        candidates,
        target,
        security_scope,
        u64::MAX,
        31,
    )
    .resolve()
    .expect_err("foreign timeline authority cannot select this backup");
    assert_eq!(denial, crate::PitrResolutionDenial::SourceAuthorityMismatch);
}

#[test]
fn pitr_rejects_an_acknowledgement_frontier_not_represented_by_the_source_cut() {
    let world = restore_world("phase-9-impossible-ack-frontier");
    let materialized = world.admissible.custody().structural().materialized();
    let manifest = materialized.manifest();
    let wal_end = manifest.wal_half_open_interval().1;
    let observation = RecoveryPhysicsTimelineAuthority::admit_observation(
        100,
        0,
        0,
        manifest.durable_checkpoint_lsn(),
        wal_end,
        wal_end,
        wal_end.saturating_sub(1),
        wal_end.saturating_sub(1),
        world.admissible.admission().admitting_authority(),
        [0x91; 32],
        materialized.manifest_digest(),
        PitrCandidatePosture::Available,
    )
    .unwrap();
    let candidates = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::ExactOnly,
        vec![observation],
    )
    .unwrap();
    let target = world.restore_directory.path().join("impossible-ack-target");
    let security_scope = recovery_security_scope(&world.admissible);
    let denial = PointInTimeRecoveryIntent::near(
        OperationalOperationId::new("pitr-impossible-ack").unwrap(),
        world.admissible,
        candidates,
        target,
        security_scope,
        u64::MAX,
        31,
    )
    .resolve()
    .expect_err("the source cut cannot promise acknowledgements outside staged WAL truth");
    assert_eq!(denial, crate::PitrResolutionDenial::FrontierOutsideSource);
}
