use super::*;
use crate::{
    AuthorizationDenial, AuthorizationReplayPolicy, AuthorizationRevocationObservation,
    BackupRestoreIntent, OperationalOperationId, OperationalTransitionId,
};

#[test]
fn provider_cannot_authorize_a_substituted_owner_plan() {
    let world = restore_world("phase-7-plan-substitution");
    let target_parent = world.restore_directory.path().join("never-staged");
    std::fs::create_dir_all(&target_parent).unwrap();
    let security_scope = recovery_security_scope(&world.admissible);
    let lowered = BackupRestoreIntent::from_verified_backup(
        OperationalOperationId::new("restore-plan-substitution").unwrap(),
        world.admissible,
        &target_parent,
        security_scope,
        u64::MAX,
        64,
    )
    .resolve()
    .lower()
    .expect("lower owner plan");
    let denial = lowered
        .authorize(
            &ExactAuthorizationPort {
                substitute_plan: Some([0xff; 32]),
            },
            &operator_assertion(),
            20,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 20 },
        )
        .expect_err("provider plan substitution must fail closed");
    assert!(matches!(denial, AuthorizationDenial::PlanBindingMismatch));
    assert!(std::fs::read_dir(target_parent).unwrap().next().is_none());
}

#[test]
fn protected_current_media_cannot_be_admitted_as_a_recovery_target() {
    let world = restore_world("phase-8-current-target-alias");
    let target_parent = world.scenario.source_root().to_path_buf();
    let source_before = media_snapshot(&target_parent);
    let control_before = world.control.observe_selection_coordinates().unwrap();
    let security_scope = recovery_security_scope(&world.admissible);
    let lowered = BackupRestoreIntent::from_verified_backup(
        OperationalOperationId::new("restore-current-target-alias").unwrap(),
        world.admissible,
        &target_parent,
        security_scope,
        u64::MAX,
        64,
    )
    .resolve()
    .lower()
    .expect("the physical plan remains inert until control-plane readiness");
    let authorized = lowered
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
        .expect("exact external authorization");
    let denial = authorized
        .ready(
            &world.control,
            OperationalTransitionId::new("reject-current-target").unwrap(),
            &world.authority,
            21,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
        )
        .expect_err("protected current media must never become a staging target");
    assert!(matches!(
        denial,
        crate::BackupRestoreReadinessDenial::Target(
            crate::NonCurrentRecoveryTargetDenial::ProtectedMediaOverlap { .. }
        )
    ));
    assert_eq!(
        world.control.observe_selection_coordinates().unwrap(),
        control_before
    );
    assert_eq!(media_snapshot(&target_parent), source_before);
}
