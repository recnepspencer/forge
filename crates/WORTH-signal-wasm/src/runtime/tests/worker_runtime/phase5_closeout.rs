use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    double_counter_observation_attach_request, set_counter, worker_shell_with_observed_counter,
};
use crate::runtime::worker_host::{
    WorkerObservationDeliveryDetachRequest, WorkerOutputDeliveryRequest,
};

fn double_counter_output_request() -> WorkerOutputDeliveryRequest {
    WorkerOutputDeliveryRequest {
        output_ids: vec!["doubleCounter".to_owned()],
    }
}

fn complete_phase5_closeout_evidence() -> crate::runtime::worker_host::WorkerRuntimeShell {
    let mut worker_shell = worker_shell_with_observed_counter();
    worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();
    worker_shell.deliver_latest_observation().unwrap();
    worker_shell
        .deliver_outputs(double_counter_output_request())
        .unwrap();
    worker_shell.read_diagnostics_summary().unwrap();
    worker_shell
}

#[test]
fn worker_phase5_closeout_certifies_one_delivery_and_diagnostics_story() {
    let worker_shell = complete_phase5_closeout_evidence();

    let package = worker_shell.certify_worker_phase5_closeout().unwrap();

    assert_eq!(
        package.certification_family,
        "workerPhase5CloseoutCertification"
    );
    assert_eq!(
        package.phase_closeout_mode,
        "ObservationOutputDiagnosticsLifecycleBoundary"
    );
    assert_eq!(package.covered_suite_count, 2);
    assert_eq!(package.observation_delivery_packet_count, 1);
    assert_eq!(package.output_delivery_packet_count, 1);
    assert_eq!(package.diagnostics_summary_read_count, 1);
    assert_eq!(package.diagnostics_rich_read_count, 0);
    assert_eq!(package.diagnostics_cold_reconstruction_count, 0);
    assert_eq!(package.active_lifecycle_subscription_count, 1);
    assert_eq!(package.observation_delivery_breadth, 1);
    assert_eq!(package.output_delivery_breadth, 1);
    assert_eq!(package.rollback_suppressed_delivery_count, 0);
    assert!(package.output_payload_byte_count >= 2);
    assert_digest_shape(&package.observation_digest);
    assert_digest_shape(&package.output_digest);
    assert_digest_shape(&package.diagnostics_summary_digest);
    assert_digest_shape(&package.rich_read_availability_digest);
    assert_digest_shape(&package.observation_lifecycle_digest);
    assert_digest_shape(&package.delivery_breadth_digest);
    assert_digest_shape(&package.boundary_performance_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_phase5_closeout_rejects_missing_diagnostics_evidence() {
    let mut worker_shell = worker_shell_with_observed_counter();
    worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();
    worker_shell.deliver_latest_observation().unwrap();
    worker_shell
        .deliver_outputs(double_counter_output_request())
        .unwrap();

    let error = worker_shell.certify_worker_phase5_closeout().unwrap_err();

    assert!(error.message.contains("summary evidence"));
}

#[test]
fn worker_phase5_closeout_rejects_cleared_evidence_after_mutation() {
    let mut worker_shell = complete_phase5_closeout_evidence();
    worker_shell
        .apply_committed_transaction(set_counter(8.0))
        .unwrap();

    let error = worker_shell.certify_worker_phase5_closeout().unwrap_err();

    assert!(error.message.contains("delivery evidence"));
}

#[test]
fn worker_phase5_closeout_rejects_lifecycle_churn_after_delivery() {
    let mut worker_shell = complete_phase5_closeout_evidence();
    worker_shell
        .attach_observation_delivery(double_counter_observation_attach_request())
        .unwrap();

    let error = worker_shell.certify_worker_phase5_closeout().unwrap_err();

    assert!(error.message.contains("current lifecycle evidence"));
}

#[test]
fn worker_phase5_closeout_rejects_detached_observation_lifecycle() {
    let mut worker_shell = worker_shell_with_observed_counter();
    let attach = worker_shell
        .attach_observation_delivery(double_counter_observation_attach_request())
        .unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();
    worker_shell.deliver_latest_observation().unwrap();
    worker_shell
        .deliver_outputs(double_counter_output_request())
        .unwrap();
    worker_shell.read_diagnostics_summary().unwrap();
    worker_shell
        .detach_observation_delivery(WorkerObservationDeliveryDetachRequest {
            lifecycle_subscription_id: attach.lifecycle_subscription_id,
        })
        .unwrap();

    let error = worker_shell.certify_worker_phase5_closeout().unwrap_err();

    assert!(error.message.contains("current lifecycle evidence"));
}
