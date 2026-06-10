#[path = "support/compat_http/phase_seven/assertions.rs"]
mod compat_http_phase_seven_assertions;
#[path = "support/compat_http/phase_seven/runtime.rs"]
mod compat_http_phase_seven_runtime;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

use forge_query::facade::ForgeQueryRuntimeSupportProfile;
use forge_server::{
    ForgeServerBinaryDownloadRequest, ForgeServerCompatibilityRequestInput,
    ForgeServerQueryHandoffDenialCode,
};

use compat_http_phase_seven_assertions::assert_download_denial;
use compat_http_phase_seven_runtime::{
    build_phase_seven_server_with_workspace_provider, compat_download_denied, download_input,
    prepared_request,
};
use query_handoff_runtime::ProfiledTestWorkspaceProvider;

#[test]
fn compat_http_binary_download_identity_changes_when_equal_length_payload_bytes_change() {
    let left = ForgeServerBinaryDownloadRequest::new(b"abcde".to_vec())
        .with_content_type("application/octet-stream");
    let right = ForgeServerBinaryDownloadRequest::new(b"vwxyz".to_vec())
        .with_content_type("application/octet-stream");

    assert_ne!(left.canonical_digest(), right.canonical_digest());
}

#[test]
fn compat_http_download_rejects_json_accept_for_the_binary_route_family() {
    let server =
        build_phase_seven_server_with_workspace_provider(ProfiledTestWorkspaceProvider::new(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
        ));
    let denial = match server.compat_http().prepare_request(
        ForgeServerCompatibilityRequestInput::builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id("tenant-a")
            .with_workspace_id("workspace-42")
            .with_branch_id("branch-9")
            .with_route_family(forge_server::ForgeServerCompatHttpRouteFamily::Download)
            .with_method("GET")
            .with_path("/compat/downloads/files.asset.download")
            .with_header("accept", "application/json")
            .build()
            .expect("json-accept download request should validate structurally"),
    ) {
        forge_proof::TransitionOutcome::Denied(value) => value,
        other => panic!("expected binary representation denial, got {other:?}"),
    };

    assert_eq!(
        denial.code(),
        forge_server::ForgeServerCompatibilityDenialCode::UnsupportedRepresentation
    );
    assert!(denial
        .detail()
        .contains("unsupported accept header `application/json`"));
}

#[test]
fn compat_http_range_parser_denies_repeated_headers_and_unsupported_units() {
    let server =
        build_phase_seven_server_with_workspace_provider(ProfiledTestWorkspaceProvider::new(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile(),
        ));
    let repeated_range = compat_download_denied(
        server
            .compat_http()
            .download(forge_server::ForgeServerBinaryDownloadExecutionInput::new(
                prepared_request(
                    &server,
                    download_input(
                        "tenant-a",
                        "workspace-42",
                        "branch-9",
                        "files.asset.download",
                    )
                    .with_header("range", "bytes=0-4")
                    .with_header("range", "bytes=5-9")
                    .build()
                    .expect("repeated range request should validate structurally"),
                ),
                "files.asset.download",
                ForgeServerBinaryDownloadRequest::new(b"phase-seven-download-body".to_vec()),
            )),
    );
    let unsupported_unit = compat_download_denied(
        server
            .compat_http()
            .download(forge_server::ForgeServerBinaryDownloadExecutionInput::new(
                prepared_request(
                    &server,
                    download_input(
                        "tenant-a",
                        "workspace-42",
                        "branch-9",
                        "files.asset.download",
                    )
                    .with_header("range", "items=0-4")
                    .build()
                    .expect("unsupported-unit request should validate structurally"),
                ),
                "files.asset.download",
                ForgeServerBinaryDownloadRequest::new(b"phase-seven-download-body".to_vec()),
            )),
    );

    assert_download_denial(
        &repeated_range,
        ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
        "at most one Range header value",
    );
    assert_download_denial(
        &unsupported_unit,
        ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
        "only admits bytes=",
    );
}
