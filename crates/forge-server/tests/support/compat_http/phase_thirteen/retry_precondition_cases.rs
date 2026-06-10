use std::sync::atomic::Ordering;

use crate::{
    compat_http_phase_thirteen_assertions::assert_denial_contains,
    compat_http_phase_thirteen_runtime::{
        canonical_download_success, compat_read_with_diagnostics, idempotent_mutation,
        idempotent_mutation_conflict, phase_thirteen_counting_server, phase_thirteen_server,
        resumed_download_success,
    },
};

#[test]
fn compat_http_phase_thirteen_retry_and_precondition_matrix_keeps_authority_and_resume_honest() {
    let (server, attempted_writes) = phase_thirteen_counting_server();
    let first = idempotent_mutation(&server, "phase-thirteen-idem", "idem-phase-13");
    let replay = idempotent_mutation(&server, "phase-thirteen-idem", "idem-phase-13");
    let conflict =
        idempotent_mutation_conflict(&server, "phase-thirteen-conflict", "idem-phase-13");

    assert!(!first.envelope().replay_receipt().is_replayed());
    assert!(replay.envelope().replay_receipt().is_replayed());
    assert_eq!(
        first.mutation_result().result_digest(),
        replay.mutation_result().result_digest(),
    );
    assert_denial_contains(
        &conflict,
        forge_server::ForgeServerQueryHandoffDenialCode::CompatibilityIdempotencyConflict,
        "cannot be reused",
    );

    let read_server = phase_thirteen_server();
    let baseline = compat_read_with_diagnostics(
        &read_server,
        "users.profile",
        forge_foundational::facade::DiagnosticRichnessProfile::Standard,
    );
    let precondition_denial = match read_server.compat_http().read(
        forge_server::ForgeServerCompatibilityExecutionInput::new(
            crate::compat_http_phase_two_runtime::prepared_read_request(
                &read_server,
                crate::compat_http_phase_two_runtime::read_input("users.profile")
                    .with_header("if-match", "\"validator:wrong\"")
                    .build()
                    .expect("conditional request should validate"),
            ),
            "users.profile",
        ),
    ) {
        forge_proof::TransitionOutcome::Denied(value) => value,
        other => panic!("expected conditional read denial, got {other:?}"),
    };
    assert_eq!(
        baseline.validator().entity_tag(),
        format!("\"{}\"", baseline.validator().canonical_digest())
    );
    assert_denial_contains(
        &precondition_denial,
        forge_server::ForgeServerQueryHandoffDenialCode::CompatibilityConditionalReadPreconditionFailed,
        "does not match the canonical read validator",
    );

    let resume_server = phase_thirteen_server();
    let resumed = resumed_download_success(&resume_server);
    let ordinary = canonical_download_success(&resume_server, "files.asset.download");
    let retry_evidence = resumed
        .transfer_cleanup_evidence()
        .expect("resumed downloads should emit retry lifecycle evidence");
    assert_eq!(
        retry_evidence.reason(),
        forge_server::ForgeServerTransferCleanupReason::DownloadRetryAdmitted
    );
    assert!(
        ordinary.transfer_cleanup_evidence().is_none(),
        "ordinary downloads must not fabricate retry lifecycle evidence",
    );

    assert_eq!(
        attempted_writes.load(Ordering::Relaxed),
        1,
        "idempotent replay must not create duplicate authority effects",
    );
}
