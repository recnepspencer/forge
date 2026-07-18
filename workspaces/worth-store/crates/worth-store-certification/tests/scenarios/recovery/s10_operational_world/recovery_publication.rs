use worth_store_authority::RecoveryAuthorityAdmissionPolicy;
use worth_store_operations::certification_scenario::{
    certification_operator_assertion, ExactScenarioAuthorizationPort,
    ExactScenarioRecoveryFencePort, OwnerBackedBackupScenario,
};
use worth_store_operations::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation,
    CurrentRecoveryAuthoritySnapshot, ExecutedAuthorityAffectingRepair, ExecutedBackupRestore,
    ExecutedPointInTimeRecovery, ExecutedRollback, OperationalControlStore,
    OperationalTransitionId, RecoveryAuthorityFrontier,
};
use worth_store_physical_certification::DrivenOperationalControlStore;
use worth_store_physical_certification::OperationalRecoveryProductionDriver;

pub fn publish_restore(
    identity: &str,
    executed: ExecutedBackupRestore,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driven: &DrivenOperationalControlStore<'_, '_>,
) {
    pending_restore(identity, executed, scenario, control, driven)
        .readmit_with_certification_control_store(
            driven,
            transition(identity, "restore-readmission"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
        )
        .unwrap();
}

fn pending_restore(
    identity: &str,
    executed: ExecutedBackupRestore,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driven: &DrivenOperationalControlStore<'_, '_>,
) -> worth_store_operations::PublishedBackupRestorePendingReadmission {
    let verified = executed.post_verify(verification_budget()).unwrap();
    let policy = RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
        verified.authority_posture(),
        [0xc2; 32],
    )
    .unwrap();
    verified
        .resolve_cutover(current_snapshot("restore", scenario, 71), policy)
        .unwrap()
        .lower_cutover(scenario.authority())
        .unwrap()
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            30,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 30 },
        )
        .unwrap()
        .establish_write_fence_with_certification_control_store(
            control,
            driven,
            transition(identity, "restore-cutover-authorization"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap()
        .publish(driven, transition(identity, "restore-publication"))
        .unwrap()
}

pub fn certify_published_readmission_recovery(
    identity: &str,
) -> worth_store_certification::courtroom::operational_recovery::PublishedReadmissionRecoveryReceipt
{
    let first = OwnerBackedBackupScenario::materialize(&format!("{identity}/rejected"));
    let first_control = first.control_store();
    let first_driver = OperationalRecoveryProductionDriver::uninterrupted();
    let first_driven = DrivenOperationalControlStore::new(&first_control, &first_driver);
    let first_source = first.execute_named(identity, "rejected-source", &first_driven);
    let first_staged =
        worth_store_operations::certification_scenario::execute_scenario_restore_staging(
            &format!("{identity}/rejected-restore"),
            first_source.into_restore_source(),
            &first.workspace_root().join("rejected-restore"),
            first.authority(),
            &first_control,
            &first_driven,
        );
    let changed =
        worth_store_operations::certification_scenario::OwnerBackedBackupScenario::materialize(
            &format!("{identity}/changed-authority"),
        );
    let rejected = pending_restore(
        identity,
        first_staged,
        &first,
        &first_control,
        &first_driven,
    )
    .attempt_readmission(
        &first_control,
        transition(identity, "rejected-readmission"),
        changed.authority(),
        &ExactScenarioRecoveryFencePort,
    )
    .unwrap();
    let worth_store_operations::BackupRestoreReadmissionOutcome::RejectedByAuthority(rejected) =
        rejected
    else {
        panic!("changed authority must reject the first publication");
    };

    let retry = OwnerBackedBackupScenario::materialize(&format!("{identity}/retry"));
    let retry_control = retry.control_store();
    let retry_driver = OperationalRecoveryProductionDriver::uninterrupted();
    let retry_driven = DrivenOperationalControlStore::new(&retry_control, &retry_driver);
    let retry_source = retry.execute_named(identity, "retry-source", &retry_driven);
    let retry_staged =
        worth_store_operations::certification_scenario::execute_scenario_restore_staging(
            &format!("{identity}/retry-restore"),
            retry_source.into_restore_source(),
            &retry.workspace_root().join("retry-restore"),
            retry.authority(),
            &retry_control,
            &retry_driven,
        );
    let pending = pending_restore(
        identity,
        retry_staged,
        &retry,
        &retry_control,
        &retry_driven,
    );
    let current_root = pending.publication().current_root();
    drop(pending);
    let selection =
        worth_store_operations::certification_scenario::ExactScenarioControlSelection::current(
            retry.authority(),
            &retry_control,
        );
    let fencing = worth_store_authority::ControlStoreFencingAuthority::for_current_store(
        retry.authority(),
        &selection,
    );
    let worth_store_operations::ControlStoreTrustPosture::Selected(selected) =
        retry_control.inspect_generations(&fencing)
    else {
        panic!("retry publication control history must remain selected");
    };
    let [handle] = selected.pending_recovery_publication_handles() else {
        panic!("one retry publication recovery handle required");
    };
    let recovered = handle
        .recover(
            &retry.workspace_root().join("publication-restore"),
            current_root,
            retry.authority(),
            &ExactScenarioRecoveryFencePort,
        )
        .unwrap();
    let worth_store_operations::RecoveredPendingRecoveryPublication::BackupRestore(recovered) =
        recovered
    else {
        panic!("retry operation must recover as backup restore");
    };
    let readmitted = recovered
        .attempt_readmission(
            &retry_control,
            transition(identity, "retry-readmission"),
            retry.authority(),
            &ExactScenarioRecoveryFencePort,
        )
        .unwrap();
    let worth_store_operations::RecoveredBackupRestoreReadmissionOutcome::Readmitted(readmitted) =
        readmitted
    else {
        panic!("freshly authorized retry must readmit");
    };
    worth_store_certification::courtroom::operational_recovery::PublishedReadmissionRecoveryReceipt::from_owner_outcomes(
        &rejected,
        &readmitted,
    )
    .expect("rejected publication and crash-recovered retry bind one receipt")
}

pub fn publish_pitr(
    identity: &str,
    executed: ExecutedPointInTimeRecovery,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driven: &DrivenOperationalControlStore<'_, '_>,
) {
    let verified = executed.post_verify(verification_budget()).unwrap();
    let policy = RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
        verified.authority_posture(),
        [0xd2; 32],
    )
    .unwrap();
    let current = verified
        .resolve_cutover(current_snapshot("pitr", scenario, 72), policy)
        .unwrap()
        .lower_cutover(scenario.authority())
        .unwrap()
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            30,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 30 },
        )
        .unwrap()
        .establish_write_fence_with_certification_control_store(
            control,
            driven,
            transition(identity, "pitr-cutover-authorization"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap()
        .publish(driven, transition(identity, "pitr-publication"))
        .unwrap()
        .readmit_with_certification_control_store(
            driven,
            transition(identity, "pitr-readmission"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
        )
        .unwrap();
    current.release_source_lease().unwrap();
}

pub fn publish_rollback(
    identity: &str,
    executed: ExecutedRollback,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driven: &DrivenOperationalControlStore<'_, '_>,
) -> worth_store_operations::CompletedRetainedAuthorityRollback {
    let verified = executed.post_verify(verification_budget()).unwrap();
    let policy = RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
        verified.authority_posture(),
        [0xd3; 32],
    )
    .unwrap();
    let current = verified
        .resolve_cutover(current_snapshot("rollback", scenario, 73), policy)
        .unwrap()
        .lower_cutover(scenario.authority())
        .unwrap()
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            30,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 30 },
        )
        .unwrap()
        .establish_write_fence_with_certification_control_store(
            control,
            driven,
            transition(identity, "rollback-cutover-authorization"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap()
        .publish(driven, transition(identity, "rollback-publication"))
        .unwrap()
        .readmit_with_certification_control_store(
            driven,
            transition(identity, "rollback-readmission"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
        )
        .unwrap();
    current.complete_retained_authority_rollback().unwrap()
}

pub fn publish_authority_repair(
    identity: &str,
    executed: ExecutedAuthorityAffectingRepair,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driven: &DrivenOperationalControlStore<'_, '_>,
) {
    let verified = executed.post_verify(verification_budget()).unwrap();
    let policy = RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
        verified.authority_posture(),
        [0xd4; 32],
    )
    .unwrap();
    let _current = verified
        .resolve_cutover(current_snapshot("authority-repair", scenario, 74), policy)
        .unwrap()
        .lower_cutover(scenario.authority())
        .unwrap()
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            30,
            80,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 30 },
        )
        .unwrap()
        .establish_write_fence_with_certification_control_store(
            control,
            driven,
            transition(identity, "authority-repair-cutover-authorization"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap()
        .publish(driven, transition(identity, "authority-repair-publication"))
        .unwrap()
        .readmit_with_certification_control_store(
            driven,
            transition(identity, "authority-repair-readmission"),
            scenario.authority(),
            &ExactScenarioRecoveryFencePort,
        )
        .unwrap();
}

fn current_snapshot(
    operation: &str,
    scenario: &OwnerBackedBackupScenario,
    seed: u8,
) -> CurrentRecoveryAuthoritySnapshot {
    let store = worth_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        scenario.authority().identity().clone(),
    );
    let roots = worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store(
        &store,
        "s10-owner-world-restore",
        91,
    );
    let publication_root = scenario
        .workspace_root()
        .join(format!("publication-{operation}"));
    std::fs::create_dir_all(&publication_root).unwrap();
    let frontier =
        RecoveryAuthorityFrontier::observed(scenario.authority(), 10, 12, 15, 14, 13, [seed; 32])
            .unwrap();
    CurrentRecoveryAuthoritySnapshot::observe(
        scenario.authority(),
        publication_root,
        roots.old_candidate,
        roots.old_reachability,
        frontier,
    )
    .unwrap()
}

fn verification_budget() -> worth_store_offline_verifier::BackupVerificationBudget {
    worth_store_offline_verifier::BackupVerificationBudget::from_inspection(
        worth_store_offline_verifier::OfflineInspectionBudget::bounded(64 * 1024, u64::MAX)
            .unwrap(),
    )
}

fn transition(identity: &str, label: &str) -> OperationalTransitionId {
    OperationalTransitionId::new(format!("{identity}/{label}")).unwrap()
}
