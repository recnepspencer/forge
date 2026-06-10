#![allow(dead_code)]

use std::sync::{atomic::AtomicUsize, Arc};

use forge_foundational::facade::DiagnosticRichnessProfile;
use forge_proof::TransitionOutcome;
use forge_query::facade::ForgeQueryRuntimeSupportProfile;
use forge_server::{
    ForgeServer, ForgeServerBinaryDownload, ForgeServerCompatibilityExport,
    ForgeServerCompatibilityMutation, ForgeServerCompatibilityRead, ForgeServerDirectMutation,
    ForgeServerDirectRead, ForgeServerMultipartUpload, ForgeServerQueryHandoffDenial,
    ForgeServerStreamCancellationKind, ForgeServerStreamCancellationReceipt,
    ForgeServerStreamingResponse,
};

use crate::{
    compat_http_phase_eight_runtime, compat_http_phase_four_assertions,
    compat_http_phase_four_runtime, compat_http_phase_nine_runtime, compat_http_phase_ten_runtime,
    compat_http_phase_three_runtime, compat_http_phase_two_runtime, forge_native_assertions,
    query_handoff_runtime,
};

pub(crate) fn phase_thirteen_server() -> ForgeServer {
    compat_http_phase_ten_runtime::build_phase_ten_server_with_workspace_provider(
        query_handoff_runtime::ProfiledTestWorkspaceProvider::new(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
        ),
    )
}

pub(crate) fn phase_thirteen_counting_server() -> (ForgeServer, Arc<AtomicUsize>) {
    let attempted_writes = Arc::new(AtomicUsize::new(0));
    let server = compat_http_phase_three_runtime::build_phase_three_server_with_workspace_provider(
        compat_http_phase_three_runtime::StatefulCountingMutationWorkspaceProvider::new(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
            attempted_writes.clone(),
        ),
    );
    (server, attempted_writes)
}

pub(crate) fn direct_and_compat_read(
    server: &ForgeServer,
    operation_name: &str,
) -> (
    forge_server::ForgeServerAdmittedDirectDeclaration,
    ForgeServerDirectRead,
    ForgeServerCompatibilityRead,
) {
    let (session, declaration) =
        compat_http_phase_two_runtime::forge_native_named_read(server, operation_name);
    let direct = match session.direct().read(&declaration) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected direct read success, got {other:?}"),
    };
    let compat =
        match server
            .compat_http()
            .read(compat_http_phase_two_runtime::compat_execution_input(
                server,
                operation_name,
            )) {
            TransitionOutcome::Success(value) => value,
            other => panic!("expected compat read success, got {other:?}"),
        };
    (declaration, direct, compat)
}

pub(crate) fn compat_read_with_diagnostics(
    server: &ForgeServer,
    operation_name: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> ForgeServerCompatibilityRead {
    match server
        .compat_http()
        .read(compat_http_phase_ten_runtime::compat_read_execution_input(
            server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            operation_name,
            diagnostics_profile,
        )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected diagnostics-scoped read success, got {other:?}"),
    }
}

pub(crate) fn direct_and_compat_mutation(
    server: &ForgeServer,
    identity: &str,
) -> (ForgeServerDirectMutation, ForgeServerCompatibilityMutation) {
    let compat =
        compat_http_phase_three_runtime::compat_mutation_success(server.compat_http().mutate(
            compat_http_phase_three_runtime::compat_mutation_execution_input(
                server,
                "tasks.insert",
                compat_http_phase_three_runtime::single_insert_body(identity),
            ),
        ));
    let direct = compat_http_phase_three_runtime::direct_mutation_success(
        forge_native_assertions::forge_native_session(server)
            .direct()
            .mutate(&forge_server::ForgeServerQueryOperation::single_mutation(
                "tasks.insert",
                compat_http_phase_three_runtime::insert_task(identity),
            )),
    );
    (direct, compat)
}

pub(crate) fn idempotent_mutation(
    server: &ForgeServer,
    identity: &str,
    idempotency_key: &str,
) -> ForgeServerCompatibilityMutation {
    compat_http_phase_three_runtime::compat_mutation_success(
        server.compat_http().mutate(
            forge_server::ForgeServerCompatibilityMutationExecutionInput::new(
                compat_http_phase_three_runtime::prepared_mutation_request(
                    server,
                    compat_http_phase_three_runtime::mutation_input("tasks.insert")
                        .with_header("idempotency-key", idempotency_key)
                        .build()
                        .expect("idempotent mutation input should validate"),
                ),
                "tasks.insert",
                compat_http_phase_three_runtime::single_insert_body(identity),
            ),
        ),
    )
}

pub(crate) fn idempotent_mutation_conflict(
    server: &ForgeServer,
    identity: &str,
    idempotency_key: &str,
) -> ForgeServerQueryHandoffDenial {
    compat_http_phase_three_runtime::compat_mutation_denied(
        server.compat_http().mutate(
            forge_server::ForgeServerCompatibilityMutationExecutionInput::new(
                compat_http_phase_three_runtime::prepared_mutation_request(
                    server,
                    compat_http_phase_three_runtime::mutation_input("tasks.insert")
                        .with_header("idempotency-key", idempotency_key)
                        .build()
                        .expect("idempotent conflict input should validate"),
                ),
                "tasks.insert",
                compat_http_phase_three_runtime::single_insert_body(identity),
            ),
        ),
    )
}

pub(crate) fn finished_incremental_export(
    server: &ForgeServer,
    operation_name: &str,
    chunk_bytes: usize,
) -> ForgeServerCompatibilityExport {
    let response =
        compat_http_phase_four_runtime::streaming_response_success(server.compat_http().stream(
            compat_http_phase_four_runtime::compat_stream_input(server, operation_name),
            forge_server::ForgeServerStreamSelection::incremental().with_chunk_bytes(chunk_bytes),
        ));
    let mut stream = match response {
        ForgeServerStreamingResponse::Stream(value) => value,
        other => panic!("expected incremental export, got {other:?}"),
    };
    let _ = compat_http_phase_four_assertions::collect_stream_bytes(&mut stream)
        .expect("stream bytes should serialize");
    stream
        .finish()
        .expect("incremental stream should finish after full consumption")
}

pub(crate) fn buffered_export(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCompatibilityExport {
    let response =
        compat_http_phase_four_runtime::streaming_response_success(server.compat_http().stream(
            compat_http_phase_four_runtime::compat_stream_input(server, operation_name),
            forge_server::ForgeServerStreamSelection::buffered(),
        ));
    match response {
        ForgeServerStreamingResponse::Buffered(value) => value,
        other => panic!("expected buffered export, got {other:?}"),
    }
}

pub(crate) fn cancellation_receipt(
    server: &ForgeServer,
    operation_name: &str,
    kind: ForgeServerStreamCancellationKind,
) -> ForgeServerStreamCancellationReceipt {
    let response =
        compat_http_phase_four_runtime::streaming_response_success(server.compat_http().stream(
            compat_http_phase_four_runtime::compat_stream_input(server, operation_name),
            compat_http_phase_four_runtime::default_stream_selection(),
        ));
    let mut stream = match response {
        ForgeServerStreamingResponse::Stream(value) => value,
        other => panic!("expected stream cancellation probe, got {other:?}"),
    };
    let _ = stream
        .next_chunk()
        .expect("cancellation probe chunk should serialize")
        .expect("cancellation probe should emit a chunk");
    match kind {
        ForgeServerStreamCancellationKind::ClientDisconnect => stream.abort_due_to_disconnect(),
        ForgeServerStreamCancellationKind::DownstreamBackpressure => {
            stream.abort_due_to_backpressure()
        }
        ForgeServerStreamCancellationKind::CallerCancelled => stream.cancel_by_caller(),
    }
}

pub(crate) fn canonical_upload_success(
    server: &ForgeServer,
    operation_name: &str,
    identity: &str,
) -> forge_server::ForgeServerCompatibilityUpload {
    compat_http_phase_ten_runtime::compat_upload_success(server.compat_http().upload(
        compat_http_phase_ten_runtime::compat_upload_execution_input(
            server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            operation_name,
            DiagnosticRichnessProfile::Standard,
            compat_http_phase_ten_runtime::canonical_upload(identity),
        ),
    ))
}

pub(crate) fn upload_with_payload(
    server: &ForgeServer,
    operation_name: &str,
    upload: ForgeServerMultipartUpload,
) -> forge_server::ForgeServerCompatibilityUploadOutcome<forge_server::ForgeServerCompatibilityUpload>
{
    server.compat_http().upload(
        compat_http_phase_ten_runtime::compat_upload_execution_input(
            server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            operation_name,
            DiagnosticRichnessProfile::Standard,
            upload,
        ),
    )
}

pub(crate) fn canonical_download_success(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerBinaryDownload {
    compat_http_phase_ten_runtime::compat_download_success(server.compat_http().download(
        compat_http_phase_ten_runtime::compat_download_execution_input(
            server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            operation_name,
            DiagnosticRichnessProfile::Standard,
            forge_server::ForgeServerBinaryDownloadRequest::new(b"hello-world".to_vec()),
        ),
    ))
}

pub(crate) fn resumed_download_success(server: &ForgeServer) -> ForgeServerBinaryDownload {
    let body = b"phase-thirteen-retry-body".to_vec();
    let first = compat_http_phase_eight_runtime::compat_download_success(
        server
            .compat_http()
            .download(forge_server::ForgeServerBinaryDownloadExecutionInput::new(
                compat_http_phase_eight_runtime::prepared_request(
                    server,
                    compat_http_phase_eight_runtime::download_input(
                        "tenant-a",
                        "workspace-42",
                        "branch-9",
                        "files.asset.download",
                    )
                    .with_header("range", "bytes=0-9")
                    .build()
                    .expect("initial range request should validate"),
                ),
                "files.asset.download",
                forge_server::ForgeServerBinaryDownloadRequest::new(body.clone()),
            )),
    );
    let resume = compat_http_phase_eight_runtime::compat_resume_success(
        server.compat_http().plan_binary_resume(first.session()),
    );
    compat_http_phase_eight_runtime::compat_download_success(
        server
            .compat_http()
            .download(forge_server::ForgeServerBinaryDownloadExecutionInput::new(
                compat_http_phase_eight_runtime::prepared_request(
                    server,
                    compat_http_phase_eight_runtime::download_input(
                        "tenant-a",
                        "workspace-42",
                        "branch-9",
                        "files.asset.download",
                    )
                    .with_header("range", "bytes=10-")
                    .build()
                    .expect("resumed range request should validate"),
                ),
                "files.asset.download",
                forge_server::ForgeServerBinaryDownloadRequest::new(body).with_resume_request(
                    forge_server::ForgeServerBinaryResumeRequest::resume_from(resume),
                ),
            )),
    )
}

pub(crate) fn malformed_upload_denial(server: &ForgeServer) -> ForgeServerQueryHandoffDenial {
    compat_http_phase_nine_runtime::compat_upload_denied(upload_with_payload(
        server,
        "files.asset",
        compat_http_phase_nine_runtime::malformed_upload(),
    ))
}
