use flate2::{write::GzEncoder, Compression};
use forge_query::facade::ForgeQueryRuntimeSupportProfile;
use forge_server::{
    ForgeServerQueryHandoffDenialCode, ForgeServerStreamCancellationKind, ForgeServerUploadChunk,
    ForgeServerUploadContentEncoding, ForgeServerUploadExpectation, ForgeServerUploadManifest,
    ForgeServerUploadPart,
};
use serde_json::json;
use std::io::Write;

use crate::{
    compat_http_phase_eight_runtime, compat_http_phase_six_runtime,
    compat_http_phase_thirteen_assertions::{
        assert_binary_counter, assert_denial_contains, assert_external_counter,
    },
    compat_http_phase_thirteen_runtime::{cancellation_receipt, phase_thirteen_server},
    compat_http_phase_twelve_runtime, query_handoff_runtime,
};

#[test]
fn compat_http_phase_thirteen_transfer_hostility_matrix_preserves_exact_counters_and_zero_truth_drift(
) {
    let slowloris_server = phase_thirteen_server();
    let cleanup_server = phase_thirteen_server();
    let slowloris =
        match slowloris_server
            .compat_http()
            .upload(compat_http_phase_twelve_runtime::upload_input(
                &slowloris_server,
                compat_http_phase_twelve_runtime::drip_fed_upload(),
            )) {
            forge_proof::TransitionOutcome::Denied(value) => value,
            other => panic!("expected slowloris denial, got {other:?}"),
        };
    let slowloris_receipt = slowloris
        .abuse_budget_receipt()
        .expect("slowloris denial should retain abuse budget receipt");
    assert_binary_counter(
        slowloris_receipt
            .binary_counters()
            .expect("slowloris denial should expose binary counters"),
        "compat_http.transfer.slowloris_cutoffs",
        1,
    );
    assert_binary_counter(
        slowloris_receipt
            .binary_counters()
            .expect("slowloris denial should expose binary counters"),
        "compat_http.transfer.semantic_truth_drift",
        0,
    );

    let disconnect = cancellation_receipt(
        &slowloris_server,
        "users.profile",
        ForgeServerStreamCancellationKind::ClientDisconnect,
    );
    assert_external_counter(
        disconnect.transfer_cleanup_evidence().external_counters(),
        "compat_http.transfer.disconnect_events",
        1,
    );
    assert_binary_counter(
        disconnect.transfer_cleanup_evidence().binary_counters(),
        "compat_http.transfer.semantic_truth_drift",
        0,
    );

    let prepared_upload = match cleanup_server.compat_http().prepare_upload(
        compat_http_phase_twelve_runtime::upload_input(&cleanup_server, staged_cleanup_upload()),
    ) {
        forge_proof::TransitionOutcome::Success(value) => value,
        other => panic!("expected upload preparation success, got {other:?}"),
    };
    let ingress_session = match cleanup_server
        .compat_http()
        .begin_binary_ingress(prepared_upload)
    {
        forge_proof::TransitionOutcome::Success(value) => value,
        other => panic!("expected staged upload session, got {other:?}"),
    };
    let cleanup = cleanup_server
        .compat_http()
        .interrupt_binary_ingress(&ingress_session)
        .expect("staged upload cleanup should succeed");
    assert_external_counter(
        cleanup.transfer_cleanup_evidence().external_counters(),
        "compat_http.transfer.staged_cleanup_events",
        1,
    );
    assert_binary_counter(
        cleanup.transfer_cleanup_evidence().binary_counters(),
        "compat_http.transfer.semantic_truth_drift",
        0,
    );

    let range_server =
        compat_http_phase_eight_runtime::build_phase_eight_server_with_workspace_provider(
            query_handoff_runtime::ProfiledTestWorkspaceProvider::new(
                ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
            ),
        );
    let multi_range_denial = compat_http_phase_eight_runtime::compat_download_denied(
        range_server.compat_http().download(
            forge_server::ForgeServerBinaryDownloadExecutionInput::new(
                compat_http_phase_eight_runtime::prepared_request(
                    &range_server,
                    compat_http_phase_eight_runtime::download_input(
                        "tenant-a",
                        "workspace-42",
                        "branch-9",
                        "files.asset.download",
                    )
                    .with_header("range", "bytes=0-4,6-9")
                    .build()
                    .expect("multi-range request should validate"),
                ),
                "files.asset.download",
                forge_server::ForgeServerBinaryDownloadRequest::new(
                    b"phase-thirteen-download-body".to_vec(),
                ),
            ),
        ),
    );
    assert_denial_contains(
        &multi_range_denial,
        ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
        "multi-range requests",
    );

    let resume_server =
        compat_http_phase_eight_runtime::build_phase_eight_server_with_workspace_provider(
            query_handoff_runtime::ProfiledTestWorkspaceProvider::new(
                ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
            ),
        );
    let baseline_body = b"phase-thirteen-download-body".to_vec();
    let first = compat_http_phase_eight_runtime::compat_download_success(
        resume_server.compat_http().download(
            forge_server::ForgeServerBinaryDownloadExecutionInput::new(
                compat_http_phase_eight_runtime::prepared_request(
                    &resume_server,
                    compat_http_phase_eight_runtime::download_input(
                        "tenant-a",
                        "workspace-42",
                        "branch-9",
                        "files.asset.download",
                    )
                    .with_header("range", "bytes=0-10")
                    .build()
                    .expect("initial resume request should validate"),
                ),
                "files.asset.download",
                forge_server::ForgeServerBinaryDownloadRequest::new(baseline_body.clone()),
            ),
        ),
    );
    let resume = compat_http_phase_eight_runtime::compat_resume_success(
        resume_server
            .compat_http()
            .plan_binary_resume(first.session()),
    );
    let integrity_mismatch = compat_http_phase_eight_runtime::compat_download_denied(
        resume_server.compat_http().download(
            forge_server::ForgeServerBinaryDownloadExecutionInput::new(
                compat_http_phase_eight_runtime::prepared_request(
                    &resume_server,
                    compat_http_phase_eight_runtime::download_input(
                        "tenant-a",
                        "workspace-42",
                        "branch-9",
                        "files.asset.download",
                    )
                    .with_header("range", "bytes=11-")
                    .build()
                    .expect("integrity mismatch request should validate"),
                ),
                "files.asset.download",
                forge_server::ForgeServerBinaryDownloadRequest::new(
                    b"phase-thirteen-download-bodz".to_vec(),
                )
                .with_resume_request(
                    forge_server::ForgeServerBinaryResumeRequest::resume_from(resume),
                ),
            ),
        ),
    );
    assert_denial_contains(
        &integrity_mismatch,
        ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
        "resume integrity digest mismatch",
    );

    let compression_server =
        compat_http_phase_six_runtime::build_phase_six_server_with_workspace_provider(
            query_handoff_runtime::ProfiledTestWorkspaceProvider::new(
                ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
            ),
        );
    let compression_denial = compat_http_phase_six_runtime::compat_upload_denied(
        compression_server.compat_http().upload(
            compat_http_phase_six_runtime::compat_upload_execution_input(
                &compression_server,
                "tenant-a",
                "workspace-42",
                "branch-9",
                "files.compressed.upload",
                "boundary-compressed",
                compression_ratio_abuse_upload(),
            ),
        ),
    );
    assert_denial_contains(
        &compression_denial,
        ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "decompression ratio cap",
    );
}

fn compression_ratio_abuse_upload() -> forge_server::ForgeServerMultipartUpload {
    let authoritative_bytes = vec![3u8; 4_096];
    let compressed_bytes = gzip_bytes(&authoritative_bytes);
    let part = ForgeServerUploadPart::file("avatar")
        .with_content_type("application/gzip")
        .with_declared_length(authoritative_bytes.len() as u64)
        .with_content_encoding(ForgeServerUploadContentEncoding::Gzip)
        .with_body_bytes(authoritative_bytes)
        .with_wire_chunk(ForgeServerUploadChunk::new(compressed_bytes));
    forge_server::ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": {
                    "identity.id": "compressed-phase-thirteen",
                    "title.value": "Compressed Phase Thirteen"
                }
            }
        }))
        .with_file_part("avatar"),
    )
    .with_expectation(ForgeServerUploadExpectation::continue_optional())
    .with_part(part)
}

fn staged_cleanup_upload() -> forge_server::ForgeServerMultipartUpload {
    forge_server::ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": {
                    "identity.id": "phase-thirteen-cleanup",
                    "title.value": "Phase Thirteen Cleanup"
                }
            }
        }))
        .with_file_part("blob"),
    )
    .with_part(
        ForgeServerUploadPart::file("blob")
            .with_content_type("application/octet-stream")
            .with_declared_length(11)
            .with_body_bytes(b"hello-world".to_vec()),
    )
}

fn gzip_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(bytes)
        .expect("gzip encoder should accept payload bytes");
    encoder
        .finish()
        .expect("gzip encoder should finish payload")
}
