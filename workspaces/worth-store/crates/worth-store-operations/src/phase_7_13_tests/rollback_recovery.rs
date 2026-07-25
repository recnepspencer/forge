use super::*;
use crate::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, OperationalOperationId,
    OperationalTransitionId, RollbackIntent,
};
use worth_store_authority::report_retained_store_authority_evidence;
use worth_store_physical_isolation::RecoverySourceLeaseRegistry;
use worth_store_recovery_physics::{
    PitrCandidatePosture, PitrRoundingPolicy, RecoveryPhysicsRollbackOwner,
    RecoveryPhysicsTimelineAuthority,
};

#[test]
fn retained_authority_rollback_stages_forward_then_completes_its_own_cutover() {
    let world = restore_world("phase-10-retained-rollback");
    let target = world.restore_directory.path().join("rollback-target");
    let leases = RecoverySourceLeaseRegistry::open(
        world
            .restore_directory
            .path()
            .join("rollback-source-leases"),
    )
    .unwrap();
    std::fs::create_dir_all(&target).unwrap();
    let retained = report_retained_store_authority_evidence(&world.authority);
    let materialized = world.admissible.custody().structural().materialized();
    let manifest = materialized.manifest();
    let wal_end = manifest.wal_half_open_interval().1;
    let lineage = RecoveryPhysicsRollbackOwner::source_lineage(&retained, manifest);
    let frontier = RecoveryPhysicsTimelineAuthority::admit_observation(
        100,
        0,
        0,
        manifest.durable_checkpoint_lsn(),
        wal_end,
        wal_end,
        manifest.acknowledged_frontier(),
        manifest.acknowledged_frontier(),
        world.admissible.admission().admitting_authority(),
        lineage,
        materialized.manifest_digest(),
        PitrCandidatePosture::Available,
    )
    .unwrap();
    let frontier = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::ExactOnly,
        vec![frontier],
    )
    .unwrap()
    .select()
    .unwrap()
    .exact_frontier();
    let current_before = media_snapshot(world.scenario.source_root());
    let security_scope = recovery_security_scope(&world.admissible);
    let lowered = RollbackIntent::from_retained_authority(
        OperationalOperationId::new("rollback-retained-authority").unwrap(),
        retained,
        world.admissible,
        frontier,
        &target,
        security_scope,
        u64::MAX,
        31,
    )
    .resolve()
    .expect("resolve retained source")
    .admit_source_cut(&leases)
    .expect("durably lease retained closure")
    .lease()
    .lower()
    .expect("lower rollback owner DAG");
    assert_recovery_lifecycle_dag(lowered.explanation());
    let executed = lowered
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
            OperationalTransitionId::new("consume-rollback-staging").unwrap(),
            &world.authority,
            21,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
        )
        .unwrap()
        .execute(&CurrentStagingAuthorizationPort)
        .expect("stage retained authority as a new root");
    assert_eq!(executed.receipt().recovery().frontier(), frontier);
    assert_eq!(media_snapshot(world.scenario.source_root()), current_before);
    assert_eq!(
        selected_staging_kind(&world.authority, &world.control),
        crate::RecoveryStagingOperationKind::Rollback
    );

    let verified = executed.post_verify(verification_budget()).unwrap();
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        world.authority.identity().clone(),
    );
    let roots = worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        111,
    );
    let publication_directory = tempfile::tempdir().unwrap();
    let current_frontier = crate::RecoveryAuthorityFrontier::observed(
        &world.authority,
        10,
        12,
        20,
        19,
        18,
        [0xb1; 32],
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
        [0xb2; 32],
    )
    .unwrap();
    let readmitted = verified
        .resolve_cutover(current, policy)
        .unwrap()
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
            OperationalTransitionId::new("consume-rollback-cutover").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap()
        .publish(
            &world.control,
            OperationalTransitionId::new("publish-rollback-root").unwrap(),
        )
        .unwrap()
        .readmit(
            &world.control,
            OperationalTransitionId::new("readmit-rollback-root").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
        )
        .unwrap();
    readmitted.release_source_lease().unwrap();
    assert!(leases.recover_active().unwrap().is_empty());
}
