use crate::runtime::tests::support::*;
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    define_portable_counter_graph, portable_counter_publication,
};
use crate::runtime::worker_host::WorkerRuntimeShell;

fn worker_shell_with_counter_graph() -> WorkerRuntimeShell {
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    worker_shell
        .publish_graph(portable_counter_publication())
        .unwrap();
    worker_shell
}

fn set_counter(value: f64) -> Vec<TransactionOp> {
    vec![TransactionOp::Set {
        id: "counter".to_owned(),
        value: SignalValue::Number(value),
        aspect: None,
        aspects: None,
    }]
}

#[test]
fn worker_diagnostics_summary_cost_honesty_keeps_cold_reconstruction_zero() {
    let mut worker_shell = worker_shell_with_counter_graph();
    worker_shell
        .apply_committed_transaction(set_counter(7.0))
        .unwrap();
    let rich_packet = worker_shell.read_diagnostics_history().unwrap();

    let summary_packet = worker_shell.read_diagnostics_summary().unwrap();
    let certification = worker_shell
        .certify_worker_diagnostics_summary_read()
        .unwrap();

    assert_eq!(rich_packet.envelope_family, "diagnosticsHistoryRead");
    assert_eq!(rich_packet.read_mode, "RichDiagnosticsHistoryRead");
    assert_eq!(rich_packet.diagnostics_rich_read_count, 1);
    assert!(rich_packet.diagnostics_cold_reconstruction_count > 0);
    assert_eq!(
        rich_packet
            .boundary_performance
            .diagnostics_cold_reconstruction_count,
        rich_packet.diagnostics_cold_reconstruction_count
    );
    assert_eq!(summary_packet.envelope_family, "diagnosticsHistoryRead");
    assert_eq!(summary_packet.read_mode, "SummaryDiagnosticsRead");
    assert_eq!(summary_packet.runtime_authority, "workerOwnedRuntime");
    assert_eq!(summary_packet.diagnostics_summary_read_count, 1);
    assert_eq!(summary_packet.diagnostics_rich_read_count, 0);
    assert_eq!(summary_packet.diagnostics_cold_reconstruction_count, 0);
    assert_eq!(
        summary_packet
            .boundary_performance
            .diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(summary_packet.boundary_performance.bridge_envelope_count, 1);
    assert_eq!(summary_packet.boundary_performance.submitted_item_count, 1);
    assert_eq!(certification.diagnostics_cold_reconstruction_count, 0);
    assert_eq!(certification.packet_digest, summary_packet.packet_digest);
    assert_digest_shape(&summary_packet.diagnostics_summary_digest);
    assert_digest_shape(&summary_packet.rich_read_availability_digest);
    assert_digest_shape(&summary_packet.packet_digest);
    assert_digest_shape(&certification.certification_digest);
}

#[test]
fn worker_diagnostics_summary_read_matches_compatibility_summary_truth() {
    let mut worker_shell = worker_shell_with_counter_graph();
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    define_portable_counter_graph(&mut compatibility_runtime);
    worker_shell
        .apply_committed_transaction(set_counter(9.0))
        .unwrap();
    compatibility_runtime
        .apply_transaction(set_counter(9.0))
        .unwrap();

    let summary_packet = worker_shell.read_diagnostics_summary().unwrap();
    let compatibility_summary = compatibility_runtime.diagnostics_summary_now().unwrap();

    assert_eq!(
        summary_packet.summary.active_node_count,
        compatibility_summary.active_node_count
    );
    assert_eq!(
        summary_packet.summary.clean_node_count,
        compatibility_summary.clean_node_count
    );
    assert_eq!(
        summary_packet.summary.dependency_edge_count,
        compatibility_summary.dependency_edge_count
    );
    assert_eq!(
        summary_packet.summary.nodes_with_execution_record,
        compatibility_summary.nodes_with_execution_record
    );
    assert_eq!(summary_packet.diagnostics_cold_reconstruction_count, 0);
    assert_eq!(
        summary_packet
            .boundary_performance
            .diagnostics_cold_reconstruction_count,
        0
    );
}

#[test]
fn worker_diagnostics_summary_certification_rejects_cleared_read_evidence() {
    let mut worker_shell = worker_shell_with_counter_graph();
    worker_shell.read_diagnostics_summary().unwrap();
    worker_shell
        .apply_committed_transaction(set_counter(11.0))
        .unwrap();

    let error = worker_shell
        .certify_worker_diagnostics_summary_read()
        .unwrap_err();

    assert!(error.message.contains("summary evidence"));
}
