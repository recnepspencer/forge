use forge_proof::TransitionOutcome;
use forge_server::{
    request_context::DiagnosticRichnessProfile, ForgeServerDirectDeclaration,
    ForgeServerDirectDeclarationDenialCode,
};

use crate::forge_native_runtime::{
    build_server, build_server_with_failing_workspace_provider, forge_native_session_input_builder,
};

#[test]
fn direct_workspace_binding_failure_fails_before_support_snapshot_and_preserves_diagnostics() {
    let server =
        build_server_with_failing_workspace_provider("bind_direct", "workspace bind exploded");

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .with_diagnostics_profile(DiagnosticRichnessProfile::OperationalMinimal)
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let denial = session
        .declarations()
        .read(ForgeServerDirectDeclaration::named_read("users.profile"))
        .expect_err("workspace binding failure must deny before support posture exists");

    assert_eq!(
        denial.code(),
        ForgeServerDirectDeclarationDenialCode::WorkspaceBindingFailed
    );
    assert_eq!(
        denial.diagnostics_profile(),
        DiagnosticRichnessProfile::OperationalMinimal
    );
    assert!(denial
        .detail()
        .contains("bind_direct: workspace bind exploded"));
    assert!(denial.support_snapshot().is_none());
}

#[test]
fn direct_source_not_admitted_denial_preserves_requested_diagnostics_profile() {
    let server = build_server(true);

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .with_diagnostics_profile(DiagnosticRichnessProfile::OperationalMinimal)
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let prepared = session
        .declarations()
        .read(ForgeServerDirectDeclaration::saved_query(
            "users.profile.saved",
        ))
        .expect("saved-query declaration should still prepare a support snapshot");

    let denial = prepared
        .admit()
        .expect_err("saved-query declaration must fail closed after preparation");

    assert_eq!(
        denial.code(),
        ForgeServerDirectDeclarationDenialCode::SourceNotAdmitted
    );
    assert_eq!(
        denial.diagnostics_profile(),
        DiagnosticRichnessProfile::OperationalMinimal
    );
    assert_eq!(
        denial
            .support_snapshot()
            .expect("source-not-admitted denial should preserve support snapshot")
            .source_support_reason(),
        "saved-query declaration intake remains deferred until a later direct-consumption phase"
    );
}
