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
use forge_query::facade::ForgeQueryRuntimeSupportProfile;
use forge_server::{
    ForgeServerQueryHandoffDenialCode, ForgeServerUploadChunk, ForgeServerUploadCleanupReason,
    ForgeServerUploadContentEncoding, ForgeServerUploadExpectation, ForgeServerUploadManifest,
    ForgeServerUploadPart, ForgeServerUploadTransferMode,
};
use serde_json::json;

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
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
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
        forge_proof::TransitionOutcome::Denied(value) => value,
        other => panic!("expected inactive-session denial, got {other:?}"),
    };

    assert_cleanup_receipt(&interrupted, ForgeServerUploadCleanupReason::Interrupted);
    assert_cleanup_receipt(&expired, ForgeServerUploadCleanupReason::Expired);
    assert_cleanup_receipt(&abandoned, ForgeServerUploadCleanupReason::Abandoned);
    assert_cleanup_receipt(&mismatch, ForgeServerUploadCleanupReason::OwnershipMismatch);
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
        ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
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
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
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
        ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "chunk pacing cap",
    );
    assert_upload_denial(
        &unknown_length_denial,
        ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "without a known content length",
    );
    assert_upload_denial(
        &compression_denial,
        ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
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
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
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
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
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
        ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
        "integrity digest mismatch",
    );
    assert_eq!(
        failed_writes.load(Ordering::Relaxed),
        0,
        "integrity mismatch must deny before metadata truth commits"
    );
}

fn begin_session(
    server: &forge_server::ForgeServer,
    tenant_id: &str,
    workspace_id: &str,
    branch_id: &str,
    operation_name: &str,
    boundary: &str,
) -> forge_server::ForgeServerBinaryIngressSession {
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
        forge_proof::TransitionOutcome::Success(value) => value,
        other => panic!("expected binary ingress session, got {other:?}"),
    }
}

fn cleanup_upload(identity: &str) -> forge_server::ForgeServerMultipartUpload {
    forge_server::ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(json!({
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
    .with_expectation(ForgeServerUploadExpectation::continue_optional())
    .with_part(
        ForgeServerUploadPart::file("avatar")
            .with_content_type("image/png")
            .with_declared_length(8)
            .with_body_bytes(b"cleanup!".to_vec()),
    )
}

fn drip_fed_upload() -> forge_server::ForgeServerMultipartUpload {
    let part = (0..33).fold(
        ForgeServerUploadPart::file("avatar")
            .with_content_type("image/png")
            .with_declared_length(33)
            .with_transfer_mode(ForgeServerUploadTransferMode::UnknownLength)
            .with_body_bytes(vec![b'a'; 33]),
        |part, _| part.with_wire_chunk(ForgeServerUploadChunk::new(vec![1])),
    );
    forge_server::ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(single_insert_body("drip")).with_file_part("avatar"),
    )
    .with_part(part)
}

fn oversized_unknown_length_upload() -> forge_server::ForgeServerMultipartUpload {
    let authoritative_bytes = vec![7u8; 1_024];
    let part = (0..9).fold(
        ForgeServerUploadPart::file("avatar")
            .with_content_type("application/octet-stream")
            .with_declared_length(authoritative_bytes.len() as u64)
            .with_transfer_mode(ForgeServerUploadTransferMode::UnknownLength)
            .with_content_encoding(ForgeServerUploadContentEncoding::Gzip)
            .with_body_bytes(authoritative_bytes),
        |part, _| part.with_wire_chunk(ForgeServerUploadChunk::new(vec![9u8; 2 * 1024 * 1024])),
    );
    forge_server::ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(single_insert_body("unknown")).with_file_part("avatar"),
    )
    .with_part(part)
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
        ForgeServerUploadManifest::new(single_insert_body("compressed")).with_file_part("avatar"),
    )
    .with_part(part)
}

fn integrity_honest_upload() -> forge_server::ForgeServerMultipartUpload {
    let manifest =
        ForgeServerUploadManifest::new(single_insert_body("integrity")).with_file_part("avatar");
    let manifest_digest = stable_digest(manifest.integrity_basis().as_bytes());
    let avatar_bytes = b"avatar-phase-six".to_vec();
    forge_server::ForgeServerMultipartUpload::new(manifest.with_integrity_digest(manifest_digest))
        .with_part(
            ForgeServerUploadPart::file("avatar")
                .with_content_type("image/png")
                .with_declared_length(avatar_bytes.len() as u64)
                .with_body_bytes(avatar_bytes.clone())
                .with_integrity_digest(stable_digest(&avatar_bytes)),
        )
}

fn integrity_mismatched_upload() -> forge_server::ForgeServerMultipartUpload {
    let avatar_bytes = b"avatar-phase-six".to_vec();
    forge_server::ForgeServerMultipartUpload::new(
        ForgeServerUploadManifest::new(single_insert_body("integrity-bad"))
            .with_file_part("avatar")
            .with_integrity_digest("wrong-manifest-digest"),
    )
    .with_part(
        ForgeServerUploadPart::file("avatar")
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
