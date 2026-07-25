use super::restore_pipeline::{execute_restore, ExecutedRestoreCase};
use super::*;
use crate::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, OperationalTransitionId,
};
use worth_store_authority::{
    ControlStoreFencingAuthority, RecoveryWriteFenceDenial, RecoveryWriteFenceDisposition,
    RecoveryWriteFencePort, RecoveryWriteFenceProviderReceipt, RecoveryWriteFenceRecoveryRequest,
    RecoveryWriteFenceReleaseProviderReceipt, RecoveryWriteFenceReleaseRequest,
    RecoveryWriteFenceRequest, StoreCurrentAuthorityWitness,
};
use worth_store_offline_verifier::{BackupVerificationBudget, OfflineInspectionBudget};
use worth_store_physical_format::PhysicalStoreIdentity;
use worth_store_test_support::harness::physical_isolation::publication::publication_inputs_for_store;

#[test]
fn readmission_denial_becomes_a_durable_rejected_publication_state() {
    let published = published_restore("phase-13-rejected-publication");
    let changed_authority = crate::backup::export::current_authority("changed-after-publication");
    let outcome = published
        .published
        .attempt_readmission(
            &published.control,
            OperationalTransitionId::new("reject-published-root").unwrap(),
            &changed_authority,
            &ExactRecoveryFencePort,
        )
        .expect("authority denial is a durable workflow outcome");
    let crate::BackupRestoreReadmissionOutcome::RejectedByAuthority(rejected) = outcome else {
        panic!("a changed authority must not readmit the published root");
    };
    assert_eq!(
        rejected.denial(),
        worth_store_authority::RecoveryAuthorityReadmissionDenial::StaleCurrentAuthority
    );
    assert_eq!(
        rejected.fence_release().disposition(),
        RecoveryWriteFenceDisposition::RejectedByAuthority
    );
    assert!(pending_publications(&published.authority, &published.control).is_empty());
    assert!(terminal_fence_releases(&published.authority, &published.control).is_empty());
}

#[test]
fn crash_recovered_publication_can_be_retained_for_forensics_explicitly() {
    let published = published_restore("phase-13-forensic-retention");
    let root = published.published.publication().current_root();
    let directory = published.publication_directory.path().to_path_buf();
    drop(published.published);
    let pending = pending_publications(&published.authority, &published.control)
        .into_iter()
        .next()
        .expect("pending publication survives process loss");
    let recovered = pending
        .recover(
            &directory,
            root,
            &published.authority,
            &ExactRecoveryFencePort,
        )
        .expect("reopen published root and fence");
    let crate::RecoveredPendingRecoveryPublication::BackupRestore(recovered) = recovered else {
        panic!("restore operation identity must survive replay");
    };
    let retained = recovered
        .retain_for_forensics(
            [0x71; 32],
            &published.control,
            OperationalTransitionId::new("retain-rejected-media").unwrap(),
            &ExactRecoveryFencePort,
        )
        .expect("forensic retention is an explicit durable disposition");
    assert_eq!(retained.retention_plan_identity(), [0x71; 32]);
    assert_eq!(
        retained.fence_release().disposition(),
        RecoveryWriteFenceDisposition::RetainedForForensics
    );
    assert!(pending_publications(&published.authority, &published.control).is_empty());
}

#[test]
fn terminal_disposition_survives_a_crash_before_fence_release() {
    let published = published_restore("phase-13-terminal-before-release");
    let basis = [0x72; 32];
    assert!(matches!(
        published.published.abandon(
            basis,
            &published.control,
            OperationalTransitionId::new("terminal-before-release").unwrap(),
            &UnavailableReleaseFencePort,
        ),
        Err(crate::RecoveryCutoverExecutionDenial::Fence(
            RecoveryWriteFenceDenial::ProviderUnavailable
        ))
    ));

    let selection = ExactControlSelection::current(&published.authority, &published.control);
    let fencing = ControlStoreFencingAuthority::for_current_store(&published.authority, &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) =
        published.control.inspect_generations(&fencing)
    else {
        panic!("terminal control state remains selectable");
    };
    assert!(selected.pending_recovery_publication_handles().is_empty());
    let [release] = selected.terminal_recovery_fence_release_handles() else {
        panic!("durable terminal disposition retains one fence reconciliation handle");
    };
    assert_eq!(release.disposition_basis(), basis);
    assert_eq!(
        release.disposition(),
        crate::TerminalRecoveryPublicationDisposition::Abandoned
    );
    assert_eq!(
        release
            .reconcile(&published.control, &ExactRecoveryFencePort)
            .expect("idempotent authority-owned fence reconciliation")
            .disposition(),
        RecoveryWriteFenceDisposition::Abandoned
    );
    assert!(terminal_fence_releases(&published.authority, &published.control).is_empty());
}

struct UnavailableReleaseFencePort;

impl RecoveryWriteFencePort for UnavailableReleaseFencePort {
    fn establish(
        &self,
        request: RecoveryWriteFenceRequest,
    ) -> Result<RecoveryWriteFenceProviderReceipt, RecoveryWriteFenceDenial> {
        ExactRecoveryFencePort.establish(request)
    }

    fn release(
        &self,
        _request: RecoveryWriteFenceReleaseRequest,
    ) -> Result<RecoveryWriteFenceReleaseProviderReceipt, RecoveryWriteFenceDenial> {
        Err(RecoveryWriteFenceDenial::ProviderUnavailable)
    }

    fn recover_active(
        &self,
        request: RecoveryWriteFenceRecoveryRequest,
    ) -> Result<RecoveryWriteFenceProviderReceipt, RecoveryWriteFenceDenial> {
        ExactRecoveryFencePort.recover_active(request)
    }
}

struct PublishedRestoreCase {
    published: crate::PublishedBackupRestorePendingReadmission,
    control: crate::OperationalControlStore,
    authority: StoreCurrentAuthorityWitness,
    publication_directory: tempfile::TempDir,
    _scenario: crate::phase_1_6_tests::support::BackupScenario,
    _restore_directory: tempfile::TempDir,
}

fn published_restore(case: &str) -> PublishedRestoreCase {
    let ExecutedRestoreCase {
        executed,
        scenario,
        authority,
        control,
        _restore_directory,
        ..
    } = execute_restore(restore_world(case));
    let verified = executed
        .post_verify(verification_budget())
        .expect("independent staged verification");
    let store = PhysicalStoreIdentity::from_aspect_identity(authority.identity().clone());
    let roots = publication_inputs_for_store(&store, 91);
    let publication_directory = tempfile::tempdir().unwrap();
    let frontier =
        crate::RecoveryAuthorityFrontier::observed(&authority, 10, 12, 15, 14, 13, [0x81; 32])
            .unwrap();
    let current = crate::CurrentRecoveryAuthoritySnapshot::observe(
        &authority,
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
    let published = verified
        .resolve_cutover(current, admission_policy)
        .unwrap()
        .lower_cutover(&authority)
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
            &control,
            OperationalTransitionId::new(format!("fence-{case}")).unwrap(),
            &authority,
            &ExactRecoveryFencePort,
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap()
        .publish(
            &control,
            OperationalTransitionId::new(format!("publish-{case}")).unwrap(),
        )
        .unwrap();
    PublishedRestoreCase {
        published,
        control,
        authority,
        publication_directory,
        _scenario: scenario,
        _restore_directory,
    }
}

fn pending_publications(
    authority: &StoreCurrentAuthorityWitness,
    control: &crate::OperationalControlStore,
) -> Vec<crate::PendingRecoveryPublicationHandle> {
    let selection = ExactControlSelection::current(authority, control);
    let fencing = ControlStoreFencingAuthority::for_current_store(authority, &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) = control.inspect_generations(&fencing)
    else {
        panic!("control state must remain selectable");
    };
    selected.pending_recovery_publication_handles().to_vec()
}

fn terminal_fence_releases(
    authority: &StoreCurrentAuthorityWitness,
    control: &crate::OperationalControlStore,
) -> Vec<crate::TerminalRecoveryFenceReleaseHandle> {
    let selection = ExactControlSelection::current(authority, control);
    let fencing = ControlStoreFencingAuthority::for_current_store(authority, &selection);
    let crate::ControlStoreTrustPosture::Selected(selected) = control.inspect_generations(&fencing)
    else {
        panic!("control state must remain selectable");
    };
    selected.terminal_recovery_fence_release_handles().to_vec()
}

fn verification_budget() -> BackupVerificationBudget {
    BackupVerificationBudget::from_inspection(
        OfflineInspectionBudget::bounded(4 * 1024, u64::MAX).unwrap(),
    )
}
