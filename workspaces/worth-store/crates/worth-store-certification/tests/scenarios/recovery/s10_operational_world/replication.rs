use std::path::Path;

use worth_store_authority::{ControlStoreFencingAuthority, PrimaryServingAuthority};
use worth_store_offline_verifier::ReplicaTargetVerificationBudget;
use worth_store_operations::certification_scenario::{
    certification_operator_assertion, ExactScenarioAuthorizationPort, OwnerBackedBackupScenario,
    ScenarioBootstrapOwner, ScenarioDisasterRecoveryMedia, ScenarioFencingProvider,
    ScenarioPromotionPublication,
};
use worth_store_operations::{
    AuthorizationReplayPolicy, AuthorizationRevocationObservation, CurrentReplicaPromotion,
    OperationalControlStore, OperationalCounterReceipt, OperationalOperationId,
    OperationalTransitionId, ReplicaBootstrapIntent, ReplicaPromotionIntent,
};
use worth_store_physical_certification::{
    DrivenOperationalControlStore, DrivenOperationalTransition, OperationalRecoveryProductionDriver,
};
use worth_store_physical_isolation::RecoverySourceLeaseRegistry;
use worth_store_replication::{
    durable_replica_target_identity, ReplicaPromotionOwner, ReplicaPromotionRejectionReceipt,
    ReplicaRecoveryFrontier, ReplicationDisasterRecoveryOwner, ReplicationLineageIdentity,
    ReplicationPeerId,
};

mod reconciliation;

pub struct ScenarioReplicaLifecycle {
    pub current: CurrentReplicaPromotion,
    pub counters: [OperationalCounterReceipt; 2],
    pub rejected_highest_observed: ReplicaPromotionRejectionReceipt,
    pub split_brain_reconciliation:
        Option<worth_store_replication::SplitBrainReconciliationReceipt>,
    pub revoked_authorization_recovery: worth_store_certification::courtroom::operational_recovery::RevokedAuthorizationRecoveryReceipt,
}

pub fn execute_replica_lifecycle(
    identity: &str,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driver: &OperationalRecoveryProductionDriver,
    driven: &DrivenOperationalControlStore<'_, '_>,
    rejoin_old_primary: bool,
) -> ScenarioReplicaLifecycle {
    let media = ScenarioDisasterRecoveryMedia::materialize(
        scenario.workspace_root(),
        scenario.security_scope_identity(),
        identity,
    );
    let target = scenario.workspace_root().join("replica-target");
    let (rejected_highest_observed, _) = reject_divergent_highest_observed(identity, &media);
    let bootstrap = execute_bootstrap(identity, scenario, control, driver, driven, &media, &target);
    let (current, promotion, revoked_authorization_recovery) =
        execute_promotion(identity, scenario, control, driver, driven, &media, &target);
    let rejoin = rejoin_old_primary.then(|| {
        reconciliation::execute_old_primary_rejoin(identity, driver, driven, &current, &media)
    });
    let split_brain_reconciliation = rejoin.as_ref().map(|rejoin| {
        reconciliation::reconcile_partition(
            identity,
            &media,
            &rejected_highest_observed,
            current.promotion_receipt(),
            rejoin,
        )
    });
    ScenarioReplicaLifecycle {
        current,
        counters: [bootstrap, promotion],
        rejected_highest_observed,
        split_brain_reconciliation,
        revoked_authorization_recovery,
    }
}

fn reject_divergent_highest_observed(
    identity: &str,
    media: &ScenarioDisasterRecoveryMedia,
) -> (
    ReplicaPromotionRejectionReceipt,
    worth_store_replication::DivergentReplicaHistoryReport,
) {
    let peer = ReplicationPeerId::from_declared_peer("highest-observed-divergent").unwrap();
    let frontier = ReplicaRecoveryFrontier::admit(120, 119, 80, 80, 9).unwrap();
    let divergent = ReplicationDisasterRecoveryOwner::classify_replica_history(
        peer.clone(),
        ReplicationLineageIdentity::from_declared_lineage(format!(
            "lineage/{identity}/highest-observed-divergent"
        ))
        .unwrap(),
        frontier,
        true,
        true,
        [0x91; 32],
        media.lineage(),
    );
    let rejection = ReplicaPromotionOwner::resolve_candidate_with_rejection_receipt(
        ReplicaPromotionOwner::intent(peer, media.frontier()),
        divergent.clone(),
    )
    .expect_err("a higher observed LSN cannot outrank divergent lineage");
    (rejection, divergent)
}

fn execute_bootstrap(
    identity: &str,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driver: &OperationalRecoveryProductionDriver,
    driven: &DrivenOperationalControlStore<'_, '_>,
    media: &ScenarioDisasterRecoveryMedia,
    target: &Path,
) -> OperationalCounterReceipt {
    let operation = operation(identity, "replica-bootstrap");
    let verified = media.verify();
    let cut = verified
        .resolve_bootstrap_source_cut(operation.stable_fingerprint(), 2, 32 * 1024)
        .expect("verified bootstrap source cut");
    let lease =
        RecoverySourceLeaseRegistry::open(scenario.workspace_root().join("bootstrap-leases"))
            .expect("bootstrap lease registry")
            .admit_bootstrap_source_cut(cut)
            .expect("durable bootstrap source lease")
            .lease();
    let target_identity = durable_replica_target_identity(media.root())
        .expect("closed DR media has a target identity");
    let ready = ReplicaBootstrapIntent::new(
        operation,
        ReplicationPeerId::from_declared_peer("replica-bootstrap-target").unwrap(),
        target_identity,
        scenario.authority().authority_identity(),
        scenario.operational_security_scope(),
    )
    .unwrap()
    .resolve(verified, lease)
    .unwrap()
    .lower()
    .unwrap()
    .authorize(
        &ExactScenarioAuthorizationPort,
        &certification_operator_assertion(),
        20,
        80,
        AuthorizationReplayPolicy::SingleUse,
        AuthorizationRevocationObservation::NotRevoked { observed_at: 20 },
    )
    .unwrap()
    .ready_with_certification_control_store(
        control,
        driven,
        transition(identity, "bootstrap-authorization"),
        scenario.authority(),
        21,
        AuthorizationRevocationObservation::NotRevoked { observed_at: 21 },
    )
    .unwrap();
    let mut owner = ScenarioBootstrapOwner::new(media.root(), target, media.frontier());
    let transferred = completed(driver.bootstrap_transfer(ready, &mut owner).unwrap());
    let executed = completed(
        driver
            .persist_bootstrap_transfer(&transferred, transition(identity, "bootstrap-transfer"))
            .unwrap(),
    );
    let counters = OperationalCounterReceipt::from_replica_bootstrap(&executed);
    let verified = completed(
        driver
            .post_verify_bootstrap(
                executed,
                target,
                ReplicaTargetVerificationBudget::bounded(64 * 1024).unwrap(),
            )
            .unwrap(),
    );
    completed(
        driver
            .complete_bootstrap(
                verified,
                driven,
                transition(identity, "bootstrap-completion"),
            )
            .unwrap(),
    );
    counters
}

fn execute_promotion(
    identity: &str,
    scenario: &OwnerBackedBackupScenario,
    control: &OperationalControlStore,
    driver: &OperationalRecoveryProductionDriver,
    driven: &DrivenOperationalControlStore<'_, '_>,
    media: &ScenarioDisasterRecoveryMedia,
    target: &Path,
) -> (
    CurrentReplicaPromotion,
    OperationalCounterReceipt,
    worth_store_certification::courtroom::operational_recovery::RevokedAuthorizationRecoveryReceipt,
) {
    let provider = ScenarioFencingProvider::for_current_prefix(control);
    let selected = ControlStoreFencingAuthority::for_current_store(scenario.authority(), &provider)
        .select_generation()
        .unwrap();
    let serving = PrimaryServingAuthority::for_selected_control_generation(
        scenario.authority(),
        selected,
        &provider,
    )
    .unwrap();
    let old_primary_lease = serving.acquire(9, 10, 100).unwrap();
    let operation = operation(identity, "replica-promotion");
    let peer = ReplicationPeerId::from_declared_peer("replica-bootstrap-target").unwrap();
    let target_identity = durable_replica_target_identity(target).unwrap();
    let history = ReplicationDisasterRecoveryOwner::classify_replica_history(
        peer.clone(),
        media.lineage(),
        media.frontier(),
        true,
        true,
        target_identity,
        media.lineage(),
    );
    let lower = || {
        ReplicaPromotionIntent::new(
            operation.clone(),
            peer.clone(),
            target_identity,
            media.frontier(),
            scenario.authority().authority_identity(),
            scenario.operational_security_scope(),
        )
        .unwrap()
        .resolve(media.verify(), history.clone(), old_primary_lease)
        .unwrap()
        .lower()
        .unwrap()
    };
    let revoked = lower()
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            29,
            89,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::Revoked {
                observed_at: 29,
                reason_fingerprint: [0x96; 32],
            },
        )
        .expect_err("revoked promotion authorization cannot become execution-ready");
    let ready = lower()
        .authorize(
            &ExactScenarioAuthorizationPort,
            &certification_operator_assertion(),
            30,
            90,
            AuthorizationReplayPolicy::SingleUse,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 30 },
        )
        .unwrap()
        .ready_with_certification_control_store(
            control,
            driven,
            transition(identity, "promotion-authorization"),
            scenario.authority(),
            31,
            AuthorizationRevocationObservation::NotRevoked { observed_at: 31 },
        )
        .unwrap();
    let fenced = completed(driver.promotion_fence(ready, &serving).unwrap());
    let durable = completed(
        driver
            .persist_promotion_fence(&fenced, transition(identity, "promotion-fence"))
            .unwrap(),
    );
    let executed = completed(
        driver
            .record_promotion(&durable, transition(identity, "promotion-record"))
            .unwrap(),
    );
    let counters = OperationalCounterReceipt::from_replica_promotion(&executed);
    let verified = completed(
        driver
            .post_verify_promotion(
                executed,
                target,
                ReplicaTargetVerificationBudget::bounded(64 * 1024).unwrap(),
            )
            .unwrap(),
    );
    let mut publication = ScenarioPromotionPublication;
    let published = completed(
        driver
            .publish_promotion(
                verified,
                driven,
                transition(identity, "promotion-publication"),
                &mut publication,
            )
            .unwrap(),
    );
    let current = completed(
        driver
            .readmit_promotion(
                published,
                driven,
                transition(identity, "promotion-readmission"),
                &serving,
                40,
                100,
            )
            .unwrap(),
    );
    let recovery = worth_store_certification::courtroom::operational_recovery::RevokedAuthorizationRecoveryReceipt::from_revoked_attempt_and_fresh_promotion(
        &revoked,
        &current,
    )
    .expect("fresh canonical promotion follows revoked authorization");
    (current, counters, recovery)
}

fn operation(identity: &str, label: &str) -> OperationalOperationId {
    OperationalOperationId::new(format!("{identity}/{label}")).unwrap()
}

fn transition(identity: &str, label: &str) -> OperationalTransitionId {
    OperationalTransitionId::new(format!("{identity}/{label}")).unwrap()
}

fn completed<T: std::fmt::Debug>(transition: DrivenOperationalTransition<T>) -> T {
    match transition {
        DrivenOperationalTransition::Completed(value) => value,
        other => panic!("uninterrupted driver returned {other:?}"),
    }
}
