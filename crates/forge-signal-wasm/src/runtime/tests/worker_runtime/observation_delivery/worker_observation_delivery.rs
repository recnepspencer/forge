use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    set_counter, worker_shell_with_observed_counter,
};

#[test]
fn worker_observation_delivery_packets_committed_observation_boundary() {
    let mut worker_shell = worker_shell_with_observed_counter();
    let transaction = worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();

    let packet = worker_shell.deliver_latest_observation().unwrap();
    let certification = worker_shell.certify_worker_observation_delivery().unwrap();

    assert_eq!(packet.envelope_family, "observationDelivery");
    assert_eq!(packet.delivery_mode, "CommittedObservationDelivery");
    assert_eq!(packet.runtime_authority, "workerOwnedRuntime");
    assert_eq!(
        packet.worker_first_truth_digest,
        transaction.committed_truth_digest
    );
    assert_eq!(packet.observation_delivery_packet_count, 1);
    assert_eq!(packet.observation_delivery_breadth, 1);
    assert_eq!(packet.delivered_observation_count, 1);
    assert_eq!(packet.rollback_suppressed_delivery_count, 0);
    assert_eq!(packet.active_lifecycle_subscription_count, 1);
    assert_eq!(packet.boundary_performance.bridge_envelope_count, 1);
    assert_eq!(packet.boundary_performance.submitted_item_count, 1);
    assert_eq!(certification.observation_delivery_breadth, 1);
    assert_eq!(certification.active_lifecycle_subscription_count, 1);
    assert_eq!(
        certification.observation_lifecycle_digest,
        packet.observation_lifecycle_digest
    );
    assert_eq!(certification.packet_digest, packet.packet_digest);
    assert_digest_shape(&packet.observation_digest);
    assert_digest_shape(&packet.observation_lifecycle_digest);
    assert_digest_shape(&packet.packet_digest);
    assert_digest_shape(&certification.certification_digest);
}

#[test]
fn worker_observation_delivery_rejects_missing_committed_observation() {
    let mut worker_shell = worker_shell_with_observed_counter();

    let error = worker_shell.deliver_latest_observation().unwrap_err();

    assert!(error.message.contains("committed observation evidence"));
}

#[test]
fn worker_observation_delivery_certification_rejects_cleared_delivery_evidence() {
    let mut worker_shell = worker_shell_with_observed_counter();
    worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();
    worker_shell.deliver_latest_observation().unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(8.0))
        .unwrap();

    let error = worker_shell
        .certify_worker_observation_delivery()
        .unwrap_err();

    assert!(error.message.contains("delivery evidence"));
}
