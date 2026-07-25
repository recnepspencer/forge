use super::restore_pipeline::execute_restore;
use super::{
    operator_assertion, restore_world, ExactAuthorizationPort, ExactControlSelection,
    ExactRecoveryFencePort,
};
use crate::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, OperationalTransitionId,
};
use worth_store_authority::ControlStoreFencingAuthority;
use worth_store_offline_verifier::{BackupVerificationBudget, OfflineInspectionBudget};
use worth_store_physical_format::PhysicalStoreIdentity;
use worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store;

#[test]
fn prepared_publication_can_abandon_only_before_a_durable_locator_exists() {
    let world = execute_restore(restore_world("prepared-publication-abandonment"));
    let verified = world
        .executed
        .post_verify(BackupVerificationBudget::from_inspection(
            OfflineInspectionBudget::bounded(4 * 1024, u64::MAX).unwrap(),
        ))
        .expect("independent staged verification");
    let store = PhysicalStoreIdentity::from_aspect_identity(world.authority.identity().clone());
    let roots = publication_inputs_for_store(&store, 81);
    let publication_directory = tempfile::tempdir().unwrap();
    let old_root = roots.old_candidate.root();
    let frontier = crate::RecoveryAuthorityFrontier::observed(
        &world.authority,
        10,
        12,
        15,
        14,
        13,
        [0xc2; 32],
    )
    .unwrap();
    let current = crate::CurrentRecoveryAuthoritySnapshot::observe(
        &world.authority,
        publication_directory.path(),
        roots.old_candidate,
        roots.old_reachability,
        frontier,
    )
    .unwrap();
    let admission_policy =
        worth_store_authority::RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
            verified.authority_posture(),
            [93; 32],
        )
        .unwrap();
    let fenced = verified
        .resolve_cutover(current, admission_policy)
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
            OperationalTransitionId::new("consume-prepared-abandonment").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap();

    let locator_directory = publication_directory
        .path()
        .join("recovery-publication-locators");
    std::fs::write(&locator_directory, b"deny locator directory creation").unwrap();
    assert!(matches!(
        fenced.publish(
            &world.control,
            OperationalTransitionId::new("publication-fails-before-locator").unwrap(),
        ),
        Err(crate::RecoveryCutoverExecutionDenial::Publication(_))
    ));
    std::fs::remove_file(locator_directory).unwrap();

    let _runtime = worth_store_physical_isolation::PhysicalRootPublicationRuntime::open(
        publication_directory.path(),
        old_root,
    )
    .expect("failed publication must preserve old current root");

    let selection = ExactControlSelection::current(&world.authority, &world.control);
    let fencing = ControlStoreFencingAuthority::for_current_store(&world.authority, &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) =
        world.control.inspect_generations(&fencing)
    else {
        panic!("prepared state must survive");
    };
    let [prepared] = selected.prepared_recovery_publication_handles() else {
        panic!("exact prepared handle required");
    };
    let abandoned = prepared
        .clone()
        .abandon_before_publication(
            publication_directory.path(),
            [0xa4; 32],
            &world.control,
            OperationalTransitionId::new("abandon-before-publication").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
        )
        .expect("locator absence proves safe abandonment");
    assert_eq!(abandoned.reason_identity(), [0xa4; 32]);

    let selection = ExactControlSelection::current(&world.authority, &world.control);
    let fencing = ControlStoreFencingAuthority::for_current_store(&world.authority, &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) =
        world.control.inspect_generations(&fencing)
    else {
        panic!("terminal state must remain selectable");
    };
    assert!(selected.prepared_recovery_publication_handles().is_empty());
    assert!(selected.pending_recovery_publication_handles().is_empty());
}
