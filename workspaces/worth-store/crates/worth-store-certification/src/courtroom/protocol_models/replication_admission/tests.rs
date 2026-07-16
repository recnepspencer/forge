use std::collections::BTreeSet;

use worth_store_formal_models::ReplicationAdmissionAction;
use worth_store_replication::ReplicationAdmissionStage;

use super::scenarios::{ordinary_replication_admission_actions, publication_pending_observation};

#[test]
fn every_replication_model_action_is_reached_by_an_ordinary_owner_outcome() {
    let executed = ordinary_replication_admission_actions();
    let declared = BTreeSet::from(ReplicationAdmissionAction::all());

    assert_eq!(executed, declared);
}

#[test]
fn crash_loss_of_pending_readiness_never_becomes_published_progress() {
    let lost_pending = publication_pending_observation();

    assert_eq!(
        lost_pending.stage(),
        ReplicationAdmissionStage::PublicationReady
    );
    assert_ne!(lost_pending.stage(), ReplicationAdmissionStage::Published);
    assert_ne!(
        lost_pending.stage(),
        ReplicationAdmissionStage::PeerProgress
    );
}
