use sha2::{Digest, Sha256};
use worth_store_operations::certification_scenario::{
    ScenarioDisasterRecoveryMedia, ScenarioOldPrimaryRejoinOwner,
};
use worth_store_operations::CurrentReplicaPromotion;
use worth_store_physical_certification::{
    DrivenOperationalControlStore, OperationalRecoveryProductionDriver,
};
use worth_store_replication::{
    durable_replica_target_identity, OldPrimaryDivergenceDisposition,
    ReplicaPromotionRejectionReceipt, ReplicationDisasterRecoveryOwner, ReplicationLineageIdentity,
    ReplicationPartitionWindow, ReplicationPeerId,
};

use super::{completed, transition};

pub(super) fn execute_old_primary_rejoin(
    identity: &str,
    driver: &OperationalRecoveryProductionDriver,
    driven: &DrivenOperationalControlStore<'_, '_>,
    current: &CurrentReplicaPromotion,
    media: &ScenarioDisasterRecoveryMedia,
) -> worth_store_replication::OldPrimaryRejoinReceipt {
    let peer = ReplicationPeerId::from_declared_peer("returning-old-primary").unwrap();
    let history = ReplicationDisasterRecoveryOwner::classify_replica_history(
        peer.clone(),
        ReplicationLineageIdentity::from_declared_lineage(format!(
            "lineage/{identity}/divergent-old-primary"
        ))
        .unwrap(),
        media.frontier(),
        true,
        true,
        [0x81; 32],
        media.lineage(),
    );
    let plan = completed(
        driver
            .plan_old_primary_rejoin(
                current,
                driven,
                transition(identity, "old-primary-rejoin-plan"),
                peer,
                history,
                OldPrimaryDivergenceDisposition::RebootstrapAfterForensicRetention,
                Some([0x82; 32]),
            )
            .unwrap(),
    );
    let resolved = completed(
        driver
            .execute_old_primary_rejoin(plan, &mut ScenarioOldPrimaryRejoinOwner)
            .unwrap(),
    );
    completed(
        driver
            .complete_old_primary_rejoin(
                resolved,
                driven,
                transition(identity, "old-primary-rejoin-completion"),
            )
            .unwrap(),
    )
    .receipt()
    .clone()
}

pub(super) fn reconcile_partition(
    identity: &str,
    media: &ScenarioDisasterRecoveryMedia,
    rejected: &ReplicaPromotionRejectionReceipt,
    promoted: &worth_store_replication::ReplicaPromotionReceipt,
    rejoin: &worth_store_replication::OldPrimaryRejoinReceipt,
) -> worth_store_replication::SplitBrainReconciliationReceipt {
    let survivor_a = ReplicationPeerId::from_declared_peer("replica-bootstrap-target").unwrap();
    let survivor_b = ReplicationPeerId::from_declared_peer("independent-survivor-b").unwrap();
    let window = ReplicationPartitionWindow::admit(
        Sha256::digest(format!("partition/{identity}")).into(),
        11,
        45,
        [ReplicationPeerId::from_declared_peer("old-primary").unwrap()],
        [survivor_a.clone(), survivor_b.clone()],
    )
    .unwrap();
    let history = |peer| {
        ReplicationDisasterRecoveryOwner::classify_replica_history(
            peer,
            media.lineage(),
            media.frontier(),
            true,
            true,
            durable_replica_target_identity(media.root()).unwrap(),
            media.lineage(),
        )
    };
    let a = history(survivor_a);
    let b = history(survivor_b);
    window
        .reconcile(
            [
                window.observe_survivor(&a, 30).unwrap(),
                window.observe_survivor(&b, 31).unwrap(),
            ],
            10,
            50,
            rejected,
            promoted,
            rejoin,
        )
        .unwrap()
}
