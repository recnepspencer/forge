use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiSourceEventIngress, WorthUiSourceProvider,
    WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial,
    WorthUiWatcherEvent,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_dsl::{
    WorthUiDslCompileReport, WorthUiDslCompileStopClass, WorthUiDslDiagnosticIdentity,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn filesystem_and_in_memory_transports_preserve_diagnostic_evidence() {
    let scenario = FilesystemApplicationLifecycleScenario::new("transport-diagnostic-parity");
    let workspace = FilesystemContractWorkspace::new("transport-diagnostic-parity");
    let malformed_source = "component workspace.component.broken {";
    workspace.write("app/main.wui", malformed_source);
    let capabilities = scenario.capability_application();

    let filesystem_snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("filesystem transport should acquire malformed source bytes");
    let filesystem_report = dsl_compilation_report(
        filesystem_snapshot.lower_to_candidate_submission(capabilities.capabilities()),
        "filesystem-authored malformed source",
    );

    let in_memory_provider = WorthUiSourceProvider::in_memory("malformed-in-memory")
        .with_file("app/main.wui", malformed_source);
    let mut in_memory_ingress = WorthUiSourceEventIngress::new(in_memory_provider).start();
    let in_memory_snapshot = in_memory_ingress
        .ingest([WorthUiWatcherEvent::provider_revision(
            "malformed-in-memory",
        )])
        .expect("in-memory production ingress should settle malformed bytes");
    let in_memory_report = dsl_compilation_report(
        in_memory_snapshot.lower_to_candidate_submission(capabilities.capabilities()),
        "in-memory malformed source",
    );

    assert_eq!(
        diagnostic_evidence(&filesystem_report),
        diagnostic_evidence(&in_memory_report),
        "transport identity must not change DSL diagnostic identity or stop class"
    );
    workspace.close();
}

fn dsl_compilation_report(
    result: Result<WorthUiWatchedCandidateSubmission, WorthUiWatchedCandidateSubmissionDenial>,
    gateway: &str,
) -> WorthUiDslCompileReport {
    match result {
        Err(WorthUiWatchedCandidateSubmissionDenial::DslCompilation(report)) => report,
        Err(other) => panic!("{gateway} stopped outside DSL compilation: {other:?}"),
        Ok(_) => panic!("{gateway} unexpectedly produced a runtime candidate"),
    }
}

fn diagnostic_evidence(
    report: &WorthUiDslCompileReport,
) -> Vec<(WorthUiDslDiagnosticIdentity, WorthUiDslCompileStopClass)> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| (diagnostic.identity().clone(), diagnostic.stop_class()))
        .collect()
}
