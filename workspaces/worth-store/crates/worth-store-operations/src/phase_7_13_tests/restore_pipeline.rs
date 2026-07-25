use super::*;
use crate::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, BackupRestoreIntent,
    ExecutedBackupRestore, OperationalOperationId, OperationalTransitionId,
};

#[test]
fn authorized_restore_runs_owner_dag_to_independently_verified_non_current_media() {
    let world = execute_restore(restore_world("phase-7-13-restore"));
    assert_ne!(world.executed.staged_media().root(), world.backup_root);
    assert_eq!(media_snapshot(&world.backup_root), world.source_before);
    assert!(world
        .executed
        .staged_media()
        .root()
        .join(".closed-staging")
        .is_file());
    world
        .executed
        .post_verify(verification_budget())
        .expect("fresh independent post-verification");
    assert_eq!(
        world
            .scenario
            .leases
            .live_index_snapshot()
            .unwrap()
            .active_leases(),
        0
    );
}

#[test]
fn post_verification_rejects_an_owner_footprint_escape_before_cutover() {
    let world = execute_restore(restore_world("phase-13-footprint-escape"));
    std::fs::write(
        world
            .executed
            .staged_media()
            .root()
            .join("undeclared-owner-write"),
        b"outside the promised owner footprint",
    )
    .unwrap();
    let denial = match world.executed.post_verify(verification_budget()) {
        Ok(_) => panic!("independent verification must catch owner footprint escape"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        crate::RecoveryCutoverDenial::PostVerification(
            worth_store_offline_verifier::StagedRecoveryPostVerificationDenial::Structural(
                worth_store_offline_verifier::BackupStructuralVerificationDenial::Defects(_)
            )
        )
    ));
}

#[test]
fn published_restore_reopens_after_crash_then_readmits_and_certifies_the_drill() {
    use std::cell::Cell;
    use worth_store_authority::{ControlStoreFencingAuthority, RecoveryWriteFenceDisposition};
    use worth_store_physical_backend::{
        ControlMediaFault, ControlRecoveryObjectHandle, PhysicalControlAppendReceipt,
    };
    use worth_store_physical_format::PhysicalStoreIdentity;
    use worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store;

    let world = execute_restore(restore_world("phase-13-crash-reopen-certify"));
    let expectation = world.executed.prepare_restore_drill_expectation();
    let verified = world
        .executed
        .post_verify(verification_budget())
        .expect("first fresh-process-equivalent verification");
    let store = PhysicalStoreIdentity::from_aspect_identity(world.authority.identity().clone());
    let roots = publication_inputs_for_store(&store, 71);
    let publication_directory = tempfile::tempdir().expect("publication directory");
    let frontier = crate::RecoveryAuthorityFrontier::observed(
        &world.authority,
        10,
        12,
        15,
        14,
        13,
        [0xc1; 32],
    )
    .expect("ordered current frontier");
    let current = crate::CurrentRecoveryAuthoritySnapshot::observe(
        &world.authority,
        publication_directory.path(),
        roots.old_candidate,
        roots.old_reachability,
        frontier,
    )
    .expect("current physical authority snapshot");
    let admission_policy =
        worth_store_authority::RecoveryAuthorityAdmissionPolicy::admit_exact_declared_residual_posture(
            verified.authority_posture(),
            [93; 32],
        )
        .expect("explicit residual authority policy");
    let resolved = verified
        .resolve_cutover(current, admission_policy)
        .expect("policy-bound cutover resolution");
    assert_eq!(resolved.authority_delta().local_durable_loss(), 3);
    assert_eq!(resolved.authority_delta().client_acknowledged_loss(), 2);
    let lowered = resolved
        .lower_cutover(&world.authority)
        .expect("distinct canonical cutover DAG");
    assert_eq!(lowered.explanation().node_count(), 3);
    assert_cutover_dag_semantics(lowered.explanation());
    let authorized = lowered
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
        .expect("fresh cutover authorization");
    let fenced = authorized
        .establish_write_fence(
            &world.control,
            OperationalTransitionId::new("consume-cutover-authorization").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .expect("quiescent exact-authority fence");
    let interrupted_control = FailAfterPreparedPublication {
        durable: &world.control,
        append_count: Cell::new(0),
    };
    let denial = match fenced.publish(
        &interrupted_control,
        OperationalTransitionId::new("publish-recovered-root").unwrap(),
    ) {
        Ok(_) => panic!("second control append must be interrupted"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        crate::RecoveryCutoverExecutionDenial::Control(_)
    ));

    let selection = ExactControlSelection::current(&world.authority, &world.control);
    let fencing = ControlStoreFencingAuthority::for_current_store(&world.authority, &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) =
        world.control.inspect_generations(&fencing)
    else {
        panic!("fresh process must select the prepared publication");
    };
    let [prepared] = selected.prepared_recovery_publication_handles() else {
        panic!("root swap without final journal append must remain recoverable");
    };
    let publication_identity = prepared.publication_identity();
    let published = prepared
        .clone()
        .complete_already_published(
            publication_directory.path(),
            &world.control,
            &world.authority,
            &ExactRecoveryFencePort,
        )
        .expect("durable locator proves and completes the interrupted publication");
    assert_eq!(published.publication_identity(), publication_identity);
    let published_root = published.current_root();

    let selection = ExactControlSelection::current(&world.authority, &world.control);
    let fencing = ControlStoreFencingAuthority::for_current_store(&world.authority, &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) =
        world.control.inspect_generations(&fencing)
    else {
        panic!("completed publication must replay");
    };
    let [pending] = selected.pending_recovery_publication_handles() else {
        panic!("exactly one pending recovery publication must survive");
    };
    let expected_posture = pending.authority_posture();
    assert_eq!(pending.admission_policy(), admission_policy);
    let recovered = pending
        .recover(
            publication_directory.path(),
            published_root,
            &world.authority,
            &ExactRecoveryFencePort,
        )
        .expect("reopen exact physical root and active fence");
    let crate::RecoveredPendingRecoveryPublication::BackupRestore(recovered) = recovered else {
        panic!("operation-specific restore recovery typestate must survive");
    };
    let readmitted = recovered
        .readmit(
            &world.control,
            OperationalTransitionId::new("readmit-reopened-restore").unwrap(),
            &world.authority,
            &ExactRecoveryFencePort,
        )
        .expect("authority readmission after reopen");
    assert_eq!(
        readmitted.publication().publication_identity(),
        publication_identity
    );
    assert_eq!(
        readmitted.fence_release().disposition(),
        RecoveryWriteFenceDisposition::Readmitted
    );
    let posture = readmitted.readmission().authority_posture();
    assert_eq!(posture, expected_posture);
    assert_eq!(
        readmitted.readmission().admission_policy(),
        admission_policy
    );
    let observed_regions = posture.regions().iter().map(|set| set.count()).sum::<u64>();
    assert!(observed_regions > 0);
    assert!(posture.unavailable().count() > 0);
    let certification = readmitted
        .certify_restore_drill(expectation, verification_budget())
        .expect("final independent truth certifies the drill");
    assert_eq!(certification.publication_identity(), publication_identity);
    assert_ne!(certification.final_verification_identity(), [0; 32]);

    struct FailAfterPreparedPublication<'a> {
        durable: &'a crate::OperationalControlStore,
        append_count: Cell<u8>,
    }
    impl crate::OperationalControlStorePort for FailAfterPreparedPublication<'_> {
        fn publish_recovery_object(
            &self,
            content: &[u8],
        ) -> Result<ControlRecoveryObjectHandle, crate::OperationalControlAppendDenial> {
            self.durable.publish_recovery_object(content)
        }
        fn append(
            &self,
            record: &crate::OperationalControlRecord,
        ) -> Result<PhysicalControlAppendReceipt, crate::OperationalControlAppendDenial> {
            let count = self.append_count.get();
            self.append_count.set(count + 1);
            if count == 0 {
                self.durable.append(record)
            } else {
                Err(crate::OperationalControlAppendDenial::Media(
                    ControlMediaFault::AllocationFailed,
                ))
            }
        }

        fn compare_exchange_authorization_consumption(
            &self,
            expected: Option<worth_store_authority::ControlStoreGeneration>,
            record: &crate::OperationalControlRecord,
        ) -> Result<PhysicalControlAppendReceipt, crate::OperationalControlAppendDenial> {
            self.durable
                .compare_exchange_authorization_consumption(expected, record)
        }
    }
}

pub(super) struct ExecutedRestoreCase {
    pub(super) executed: ExecutedBackupRestore,
    pub(super) scenario: crate::phase_1_6_tests::support::BackupScenario,
    pub(super) authority: worth_store_authority::StoreCurrentAuthorityWitness,
    pub(super) control: crate::OperationalControlStore,
    pub(super) backup_root: std::path::PathBuf,
    pub(super) source_before: Vec<(String, Vec<u8>)>,
    pub(super) _restore_directory: tempfile::TempDir,
}

pub(super) fn execute_restore(world: RestoreWorld) -> ExecutedRestoreCase {
    let source_before = media_snapshot(&world.backup_root);
    let target_parent = world.restore_directory.path().join("non-current");
    std::fs::create_dir_all(&target_parent).unwrap();
    let admitted_capacity = source_before
        .iter()
        .map(|(_, bytes)| bytes.len() as u64)
        .sum::<u64>()
        .saturating_add(1024 * 1024);
    let security_scope = recovery_security_scope(&world.admissible);
    let lowered = BackupRestoreIntent::from_verified_backup(
        OperationalOperationId::new("restore-phase-7-13").unwrap(),
        world.admissible,
        &target_parent,
        security_scope,
        admitted_capacity,
        31,
    )
    .resolve()
    .lower()
    .expect("canonical owner DAG lowering");
    assert_recovery_lifecycle_dag(lowered.explanation());
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
    let ready = authorized
        .ready(
            &world.control,
            OperationalTransitionId::new("consume-staging-authorization").unwrap(),
            &world.authority,
            21,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
        )
        .expect("durable single-use authorization consumption");
    ExecutedRestoreCase {
        executed: ready
            .execute(&CurrentStagingAuthorizationPort)
            .expect("non-current staging and replay"),
        scenario: world.scenario,
        authority: world.authority,
        control: world.control,
        backup_root: world.backup_root,
        source_before,
        _restore_directory: world.restore_directory,
    }
}
