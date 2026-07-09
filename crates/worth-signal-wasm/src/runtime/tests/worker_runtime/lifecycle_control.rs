use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    double_counter_observation_attach_request, set_counter, worker_shell_with_counter_graph,
};
use crate::runtime::worker_host::{
    WorkerObservationDeliveryAttachRequest, WorkerObservationDeliveryDetachRequest,
};

#[test]
fn worker_lifecycle_control_attaches_observation_delivery_subscription() {
    let mut worker_shell = worker_shell_with_counter_graph();

    let packet = worker_shell
        .attach_observation_delivery(double_counter_observation_attach_request())
        .unwrap();
    let certification = worker_shell.certify_worker_lifecycle_control().unwrap();

    assert_eq!(packet.envelope_family, "lifecycleControl");
    assert_eq!(packet.lifecycle_event, "ObserverAttached");
    assert_eq!(packet.lifecycle_artifact, "observationDeliverySubscription");
    assert_eq!(packet.runtime_authority, "workerOwnedRuntime");
    assert_eq!(packet.observer_attached_count, 1);
    assert_eq!(packet.observer_detached_count, 0);
    assert_eq!(packet.detach_denial_count, 0);
    assert_eq!(packet.active_observer_count, 1);
    assert_eq!(packet.boundary_performance.bridge_envelope_count, 1);
    assert_eq!(packet.boundary_performance.runtime_admitted_item_count, 1);
    assert_eq!(certification.lifecycle_event, "ObserverAttached");
    assert_eq!(certification.packet_digest, packet.packet_digest);
    assert_digest_shape(&packet.lifecycle_digest);
    assert_digest_shape(&packet.packet_digest);
    assert_digest_shape(&certification.certification_digest);
}

#[test]
fn worker_lifecycle_control_detach_suppresses_observation_delivery() {
    let mut worker_shell = worker_shell_with_counter_graph();
    let attach = worker_shell
        .attach_observation_delivery(double_counter_observation_attach_request())
        .unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();
    worker_shell.deliver_latest_observation().unwrap();

    let detach = worker_shell
        .detach_observation_delivery(WorkerObservationDeliveryDetachRequest {
            lifecycle_subscription_id: attach.lifecycle_subscription_id,
        })
        .unwrap();
    let certification = worker_shell.certify_worker_lifecycle_control().unwrap();
    let stale_delivery_certification = worker_shell
        .certify_worker_observation_delivery()
        .unwrap_err();
    worker_shell
        .apply_committed_transaction(set_counter(8.0))
        .unwrap();

    let delivery_error = worker_shell.deliver_latest_observation().unwrap_err();

    assert_eq!(detach.lifecycle_event, "ObserverDetached");
    assert_eq!(detach.observer_detached_count, 1);
    assert_eq!(detach.active_observer_count, 0);
    assert_eq!(certification.lifecycle_event, "ObserverDetached");
    assert!(stale_delivery_certification
        .message
        .contains("active lifecycle subscription"));
    assert!(delivery_error
        .message
        .contains("active lifecycle subscription"));
}

#[test]
fn worker_lifecycle_control_churn_invalidates_observation_delivery_certification() {
    let mut worker_shell = worker_shell_with_counter_graph();
    worker_shell
        .attach_observation_delivery(double_counter_observation_attach_request())
        .unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();
    worker_shell.deliver_latest_observation().unwrap();

    worker_shell
        .attach_observation_delivery(double_counter_observation_attach_request())
        .unwrap();

    let stale_delivery_certification = worker_shell
        .certify_worker_observation_delivery()
        .unwrap_err();

    assert!(stale_delivery_certification
        .message
        .contains("current lifecycle evidence"));
}

#[test]
fn worker_lifecycle_control_reports_unknown_detach_without_truth_mutation() {
    let mut worker_shell = worker_shell_with_counter_graph();
    let before = worker_shell.branch_truth_envelope().unwrap();

    let packet = worker_shell
        .detach_observation_delivery(WorkerObservationDeliveryDetachRequest {
            lifecycle_subscription_id: 404,
        })
        .unwrap();
    let after = worker_shell.branch_truth_envelope().unwrap();

    assert_eq!(packet.lifecycle_event, "ObserverDetachDenied");
    assert_eq!(packet.detach_denial_count, 1);
    assert_eq!(packet.active_observer_count, 0);
    assert_eq!(packet.boundary_performance.runtime_admitted_item_count, 0);
    assert_eq!(before.committed_truth_digest, after.committed_truth_digest);
}

#[test]
fn worker_lifecycle_control_rejects_source_attachment_and_stale_certification() {
    let mut worker_shell = worker_shell_with_counter_graph();

    let source_error = worker_shell
        .attach_observation_delivery(WorkerObservationDeliveryAttachRequest {
            signal_id: "counter".to_owned(),
        })
        .unwrap_err();
    let attach = worker_shell
        .attach_observation_delivery(double_counter_observation_attach_request())
        .unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(9.0))
        .unwrap();

    let certification_error = worker_shell.certify_worker_lifecycle_control().unwrap_err();

    assert!(source_error.message.contains("not a published output"));
    assert_eq!(attach.lifecycle_event, "ObserverAttached");
    assert!(certification_error.message.contains("lifecycle evidence"));
}
