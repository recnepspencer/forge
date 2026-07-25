use super::restore_pipeline::execute_restore;
use super::*;
use crate::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, OperationalTransitionId,
};
use worth_store_physical_format::PhysicalStoreIdentity;
use worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store;

#[test]
fn authority_may_advance_during_staging_but_not_after_cutover_authorization() {
    let staged_world = execute_restore(restore_world("cutover-authority-advancement"));
    let verified = staged_world
        .executed
        .post_verify(verification_budget())
        .unwrap();
    let advanced = crate::backup::export::current_authority("advanced-during-staging");
    let store = PhysicalStoreIdentity::from_aspect_identity(advanced.identity().clone());
    let roots = publication_inputs_for_store(&store, 121);
    let publication_directory = tempfile::tempdir().unwrap();
    let frontier =
        crate::RecoveryAuthorityFrontier::observed(&advanced, 10, 12, 21, 20, 19, [0xc1; 32])
            .unwrap();
    let current = crate::CurrentRecoveryAuthoritySnapshot::observe(
        &advanced,
        publication_directory.path(),
        roots.old_candidate,
        roots.old_reachability,
        frontier,
    )
    .unwrap();
    let policy = worth_store_authority::RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
        verified.authority_posture(),
        [0xc2; 32],
    )
    .unwrap();
    let resolved = verified.resolve_cutover(current, policy).unwrap();
    assert!(resolved.authority_delta().authority_changed());
    assert!(resolved.authority_delta().local_durable_loss() > 0);
    let authorized = resolved
        .lower_cutover(&advanced)
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
        .unwrap();

    let later = crate::backup::export::current_authority("advanced-after-authorization");
    let denial = match authorized.establish_write_fence(
        &staged_world.control,
        OperationalTransitionId::new("reject-stale-cutover-authorization").unwrap(),
        &later,
        &ExactRecoveryFencePort,
        31,
        AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
    ) {
        Ok(_) => panic!("post-authorization authority advance invalidates the exact plan"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        crate::RecoveryCutoverExecutionDenial::StaleAuthority
    ));
}
