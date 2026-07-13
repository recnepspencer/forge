#[path = "support/compat_http/phase_six_assertions.rs"]
mod compat_http_phase_six_assertions;
#[path = "support/compat_http/phase_six_runtime.rs"]
mod compat_http_phase_six_runtime;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use flate2::{write::GzEncoder, Compression};
use serde_json::json;
use worth_query::facade::WorthQueryRuntimeSupportProfile;
use worth_server::{
    WorthServerQueryHandoffDenialCode, WorthServerUploadChunk, WorthServerUploadCleanupReason,
    WorthServerUploadContentEncoding, WorthServerUploadExpectation, WorthServerUploadManifest,
    WorthServerUploadPart, WorthServerUploadTransferMode,
};

use compat_http_phase_six_assertions::{
    assert_cleanup_receipt, assert_counter, assert_ingress_counters, assert_upload_denial,
    stable_digest,
};
use compat_http_phase_six_runtime::{
    build_phase_six_server_with_workspace_provider, compat_upload_denied,
    compat_upload_execution_input, compat_upload_success, prepared_request, prepared_upload,
    upload_input,
};
use query_handoff_runtime::ProfiledCountingTestWorkspaceProvider;

#[test]
fn compat_http_staged_upload_cleanup_receipts_prevent_authoritative_truth_drift() {
    let attempted_writes = Arc::new(AtomicUsize::new(0));
    let server =
        build_phase_six_server_with_workspace_provider(ProfiledCountingTestWorkspaceProvider::new(
            WorthQueryRuntimeSupportProfile::scaffold_backend_profile(),
            attempted_writes.clone(),
        ));

    let interrupted_session = begin_session(
        &server,
        "tenant-a",
        "workspace-42",
        "branch-9",
        "cleanup.interrupted",
        "boundary-interrupted",
    );
    let interrupted = server
        .compat_http()
        .interrupt_binary_ingress(&interrupted_session)
        .expect("interrupted session cleanup should succeed");
    let expired = server
        .compat_http()
        .expire_binary_ingress(&begin_session(
            &server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            "cleanup.expired",
            "boundary-expired",
        ))
        .expect("expired session cleanup should succeed");
    let abandoned = server
        .compat_http()
        .abandon_binary_ingress(&begin_session(
            &server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            "cleanup.abandoned",
            "boundary-abandoned",
        ))
        .expect("abandoned session cleanup should succeed");
    let mismatched_session = begin_session(
        &server,
        "tenant-a",
        "workspace-42",
        "branch-9",
        "cleanup.mismatch",
        "boundary-mismatch",
    );
    let mismatched_request = prepared_request(
        &server,
        upload_input(
            "tenant-z",
            "workspace-42",
            "branch-9",
            "cleanup.mismatch",
            "boundary-mismatch-foreign",
        ),
    );
    let mismatch = server
        .compat_http()
        .cleanup_mismatched_binary_ingress(&mismatched_request, &mismatched_session)
        .expect("mismatched session cleanup should succeed");
    let interrupted_after_cleanup_denial = match server
        .compat_http()
        .verify_binary_ingress(interrupted_session)
    {
        worth_proof::TransitionOutcome::Denied(value) => value,
        other => panic!("expected inactive-session denial, got {other:?}"),
    };

    assert_cleanup_receipt(&interrupted, WorthServerUploadCleanupReason::Interrupted);
    assert_cleanup_receipt(&expired, WorthServerUploadCleanupReason::Expired);
    assert_cleanup_receipt(&abandoned, WorthServerUploadCleanupReason::Abandoned);
    assert_cleanup_receipt(&mismatch, WorthServerUploadCleanupReason::OwnershipMismatch);
    assert_counter(
        interrupted.performance_receipt(),
        "compat_http.upload.cleanup_operations",
        1,
    );
    assert_counter(
        interrupted.performance_receipt(),
        "compat_http.upload.cleanup_staged_bytes",
        8,
    );
    assert_upload_denial(
        &interrupted_after_cleanup_denial,
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "no longer active",
    );
    assert_eq!(
        attempted_writes.load(Ordering::Relaxed),
        0,
        "cleanup paths must not advance authoritative metadata truth"
    );
}

#[test]
fn compat_http_chunked_and_compressed_ingress_enforces_exact_bounds() {
    let attempted_writes = Arc::new(AtomicUsize::new(0));
    let server =
        build_phase_six_server_with_workspace_provider(ProfiledCountingTestWorkspaceProvider::new(
            WorthQueryRuntimeSupportProfile::scaffold_backend_profile(),
            attempted_writes.clone(),
        ));

    let drip_denial =
        compat_upload_denied(server.compat_http().upload(compat_upload_execution_input(
            &server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            "files.drip.upload",
            "boundary-drip",
            drip_fed_upload(),
        )));
    let unknown_length_denial =
        compat_upload_denied(server.compat_http().upload(compat_upload_execution_input(
            &server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            "files.unknown.upload",
            "boundary-unknown",
            oversized_unknown_length_upload(),
        )));
    let compression_denial =
        compat_upload_denied(server.compat_http().upload(compat_upload_execution_input(
            &server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            "files.compressed.upload",
            "boundary-compressed",
            compression_ratio_abuse_upload(),
        )));

    assert_upload_denial(
        &drip_denial,
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "chunk pacing cap",
    );
    assert_upload_denial(
        &unknown_length_denial,
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "without a known content length",
    );
    assert_upload_denial(
        &compression_denial,
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "decompression ratio cap",
    );
    assert_eq!(
        attempted_writes.load(Ordering::Relaxed),
        0,
        "ingress hostility denials must occur before authoritative writes"
    );
}

#[test]
fn compat_http_upload_integrity_preserves_exact_digests_and_blocks_mismatch_before_commit() {
    let success_writes = Arc::new(AtomicUsize::new(0));
    let success_server =
        build_phase_six_server_with_workspace_provider(ProfiledCountingTestWorkspaceProvider::new(
            WorthQueryRuntimeSupportProfile::scaffold_backend_profile(),
            success_writes.clone(),
        ));
    let success_upload = integrity_honest_upload();
    let expected_manifest_digest =
        stable_digest(success_upload.manifest().integrity_basis().as_bytes());
    let expected_avatar_digest = stable_digest(b"avatar-phase-six");
    let success = compat_upload_success(success_server.compat_http().upload(
        compat_upload_execution_input(
            &success_server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            "files.integrity.upload",
            "boundary-integrity",
            success_upload,
        ),
    ));

    assert_eq!(
        success.ingress_integrity().manifest_digest(),
        expected_manifest_digest
    );
    assert_eq!(
        success.ingress_integrity().part_digest("avatar"),
        Some(expected_avatar_digest.as_str())
    );
    assert_ingress_counters(success.ingress_performance(), 16, 16, 0, 0, 1);
    assert_eq!(success_writes.load(Ordering::Relaxed), 1);

    let failed_writes = Arc::new(AtomicUsize::new(0));
    let failed_server =
        build_phase_six_server_with_workspace_provider(ProfiledCountingTestWorkspaceProvider::new(
            WorthQueryRuntimeSupportProfile::scaffold_backend_profile(),
            failed_writes.clone(),
        ));
    let denial = compat_upload_denied(failed_server.compat_http().upload(
        compat_upload_execution_input(
            &failed_server,
            "tenant-a",
            "workspace-42",
            "branch-9",
            "files.integrity.upload",
            "boundary-integrity-denied",
            integrity_mismatched_upload(),
        ),
    ));

    assert_upload_denial(
        &denial,
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "integrity digest mismatch",
    );
    assert_eq!(
        failed_writes.load(Ordering::Relaxed),
        0,
        "integrity mismatch must deny before metadata truth commits"
    );
}

fn begin_session(
    server: &worth_server::WorthServer,
    tenant_id: &str,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    boundary: &str,
) -> worth_server::WorthServerBinaryIngressSession {
    let prepared = prepared_upload(
        server,
        tenant_id,
        workspace_id,
        branch_id,
        operation_name,
        boundary,
        cleanup_upload(operation_name),
    );
    match server.compat_http().begin_binary_ingress(prepared) {
        worth_proof::TransitionOutcome::Success(value) => value,
        other => panic!("expected binary ingress session, got {other:?}"),
    }
}

fn cleanup_upload(identity: &str) -> worth_server::WorthServerMultipartUpload {
    worth_server::WorthServerMultipartUpload::new(
        WorthServerUploadManifest::new(json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": {
                    "identity.id": identity,
                    "title.value": format!("Cleanup {identity}")
                }
            }
        }))
        .with_file_part("avatar"),
    )
    .with_expectation(WorthServerUploadExpectation::continue_optional())
    .with_part(
        WorthServerUploadPart::file("avatar")
            .with_content_type("image/png")
            .with_declared_length(8)
            .with_body_bytes(b"cleanup!".to_vec()),
    )
}

fn drip_fed_upload() -> worth_server::WorthServerMultipartUpload {
    let part = (0..33).fold(
        WorthServerUploadPart::file("avatar")
            .with_content_type("image/png")
            .with_declared_length(33)
            .with_transfer_mode(WorthServerUploadTransferMode::UnknownLength)
            .with_body_bytes(vec![b'a'; 33]),
        |part, _| part.with_wire_chunk(WorthServerUploadChunk::new(vec![1])),
    );
    worth_server::WorthServerMultipartUpload::new(
        WorthServerUploadManifest::new(single_insert_body("drip")).with_file_part("avatar"),
    )
    .with_part(part)
}

fn oversized_unknown_length_upload() -> worth_server::WorthServerMultipartUpload {
    let authoritative_bytes = vec![7u8; 1_024];
    let part = (0..9).fold(
        WorthServerUploadPart::file("avatar")
            .with_content_type("application/octet-stream")
            .with_declared_length(authoritative_bytes.len() as u64)
            .with_transfer_mode(WorthServerUploadTransferMode::UnknownLength)
            .with_content_encoding(WorthServerUploadContentEncoding::Gzip)
            .with_body_bytes(authoritative_bytes),
        |part, _| part.with_wire_chunk(WorthServerUploadChunk::new(vec![9u8; 2 * 1024 * 1024])),
    );
    worth_server::WorthServerMultipartUpload::new(
        WorthServerUploadManifest::new(single_insert_body("unknown")).with_file_part("avatar"),
    )
    .with_part(part)
}

fn compression_ratio_abuse_upload() -> worth_server::WorthServerMultipartUpload {
    let authoritative_bytes = vec![3u8; 4_096];
    let compressed_bytes = gzip_bytes(&authoritative_bytes);
    let part = WorthServerUploadPart::file("avatar")
        .with_content_type("application/gzip")
        .with_declared_length(authoritative_bytes.len() as u64)
        .with_content_encoding(WorthServerUploadContentEncoding::Gzip)
        .with_body_bytes(authoritative_bytes)
        .with_wire_chunk(WorthServerUploadChunk::new(compressed_bytes));
    worth_server::WorthServerMultipartUpload::new(
        WorthServerUploadManifest::new(single_insert_body("compressed")).with_file_part("avatar"),
    )
    .with_part(part)
}

fn integrity_honest_upload() -> worth_server::WorthServerMultipartUpload {
    let manifest =
        WorthServerUploadManifest::new(single_insert_body("integrity")).with_file_part("avatar");
    let manifest_digest = stable_digest(manifest.integrity_basis().as_bytes());
    let avatar_bytes = b"avatar-phase-six".to_vec();
    worth_server::WorthServerMultipartUpload::new(manifest.with_integrity_digest(manifest_digest))
        .with_part(
            WorthServerUploadPart::file("avatar")
                .with_content_type("image/png")
                .with_declared_length(avatar_bytes.len() as u64)
                .with_body_bytes(avatar_bytes.clone())
                .with_integrity_digest(stable_digest(&avatar_bytes)),
        )
}

fn integrity_mismatched_upload() -> worth_server::WorthServerMultipartUpload {
    let avatar_bytes = b"avatar-phase-six".to_vec();
    worth_server::WorthServerMultipartUpload::new(
        WorthServerUploadManifest::new(single_insert_body("integrity-bad"))
            .with_file_part("avatar")
            .with_integrity_digest("wrong-manifest-digest"),
    )
    .with_part(
        WorthServerUploadPart::file("avatar")
            .with_content_type("image/png")
            .with_declared_length(avatar_bytes.len() as u64)
            .with_body_bytes(avatar_bytes)
            .with_integrity_digest("wrong-part-digest"),
    )
}

fn single_insert_body(identity: &str) -> serde_json::Value {
    json!({
        "command": {
            "family": "insert",
            "collection": "Task",
            "aspects": {
                "identity.id": identity,
                "title.value": format!("Title for {identity}")
            }
        }
    })
}

fn gzip_bytes(bytes: &[u8]) -> Vec<u8> {
    use std::io::Write;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(bytes)
        .expect("gzip encoder should accept payload bytes");
    encoder
        .finish()
        .expect("gzip encoder should finish payload")
}
