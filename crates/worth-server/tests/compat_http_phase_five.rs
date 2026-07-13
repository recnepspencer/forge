#[path = "support/compat_http/phase_five_runtime.rs"]
mod compat_http_phase_five_runtime;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

use std::sync::atomic::Ordering;

use serde_json::json;
use worth_query::facade::runtime::WorthQueryRuntimeSupportProfile;
use worth_server::{
    WorthServerCompatHttpRouteFamily, WorthServerCompatibilityRequestInput,
    WorthServerMultipartUpload, WorthServerQueryHandoffDenialCode, WorthServerUploadExpectation,
    WorthServerUploadManifest, WorthServerUploadPart,
};

use compat_http_phase_five_runtime::{
    build_phase_five_server, build_phase_five_server_with_mutation_disabled,
    build_phase_five_server_with_workspace_provider, compat_mutation_execution_input,
    compat_mutation_success, compat_upload_denied, compat_upload_execution_input,
    compat_upload_success, manifest_for, prepared_request, prepared_upload_denied,
    single_insert_body, upload_order_alpha, upload_order_beta,
};
use query_handoff_runtime::ProfiledCountingTestWorkspaceProvider;

#[test]
fn compat_http_upload_preserves_canonical_metadata_truth_across_part_order_and_boundary_variation()
{
    let alpha_server = build_phase_five_server();
    let beta_server = build_phase_five_server();

    let alpha_prepared = prepared_request(
        &alpha_server,
        compat_http_phase_five_runtime::upload_input("files.avatar.upload", "boundary-alpha")
            .build()
            .expect("alpha upload input should validate structurally"),
    );
    let beta_prepared = prepared_request(
        &beta_server,
        compat_http_phase_five_runtime::upload_input("files.avatar.upload", "boundary-beta")
            .build()
            .expect("beta upload input should validate structurally"),
    );
    let alpha = compat_upload_success(alpha_server.compat_http().upload(
        compat_upload_execution_input(
            &alpha_server,
            "files.avatar.upload",
            "boundary-alpha",
            upload_order_alpha("task-1"),
        ),
    ));
    let beta = compat_upload_success(beta_server.compat_http().upload(
        compat_upload_execution_input(
            &beta_server,
            "files.avatar.upload",
            "boundary-beta",
            upload_order_beta("task-1"),
        ),
    ));

    assert_ne!(
        alpha_prepared.request_contract().canonical_digest(),
        beta_prepared.request_contract().canonical_digest()
    );
    assert_eq!(
        alpha.upload().canonical_digest(),
        beta.upload().canonical_digest()
    );
    assert_eq!(
        alpha.mutation().mutation_request().canonical_digest(),
        beta.mutation().mutation_request().canonical_digest()
    );
    assert_eq!(
        alpha.mutation().mutation_result().result_digest(),
        beta.mutation().mutation_result().result_digest()
    );
    assert_eq!(
        alpha.mutation().mutation_result().inspection_digest(),
        beta.mutation().mutation_result().inspection_digest()
    );
    assert_eq!(
        alpha
            .mutation()
            .envelope()
            .response_envelope()
            .canonical_digest(),
        beta.mutation()
            .envelope()
            .response_envelope()
            .canonical_digest()
    );
}

#[test]
fn compat_http_upload_lowers_metadata_through_the_same_mutation_lane() {
    let upload_server = build_phase_five_server();
    let mutation_server = build_phase_five_server();

    let upload = compat_upload_success(upload_server.compat_http().upload(
        compat_upload_execution_input(
            &upload_server,
            "files.avatar.upload",
            "boundary-shared",
            upload_order_alpha("task-7"),
        ),
    ));
    let plain_mutation = compat_mutation_success(mutation_server.compat_http().mutate(
        compat_mutation_execution_input(
            &mutation_server,
            "files.avatar.upload",
            single_insert_body("task-7"),
        ),
    ));

    assert_eq!(
        upload.mutation().mutation_request().canonical_digest(),
        plain_mutation.mutation_request().canonical_digest()
    );
    assert_eq!(
        upload.mutation().mutation_result().result_digest(),
        plain_mutation.mutation_result().result_digest()
    );
    assert_eq!(
        upload.mutation().mutation_result().inspection_digest(),
        plain_mutation.mutation_result().inspection_digest()
    );
}

#[test]
fn compat_http_upload_rejects_malformed_part_graphs_and_unsupported_content_shapes() {
    let server = build_phase_five_server();
    let missing_part = compat_upload_denied(
        server.compat_http().upload(compat_upload_execution_input(
            &server,
            "files.avatar.upload",
            "boundary-a",
            WorthServerMultipartUpload::new(manifest_for("task-1")).with_part(
                WorthServerUploadPart::file("avatar")
                    .with_content_type("image/png")
                    .with_declared_length(128),
            ),
        )),
    );
    let missing_metadata = compat_upload_denied(
        server.compat_http().upload(compat_upload_execution_input(
            &server,
            "files.avatar.upload",
            "boundary-metadata",
            WorthServerMultipartUpload::new(
                WorthServerUploadManifest::new(json!({}))
                    .with_file_part("avatar")
                    .with_file_part("thumbnail"),
            )
            .with_part(
                WorthServerUploadPart::file("avatar")
                    .with_content_type("image/png")
                    .with_declared_length(128),
            )
            .with_part(
                WorthServerUploadPart::file("thumbnail")
                    .with_content_type("image/webp")
                    .with_declared_length(64),
            ),
        )),
    );
    let duplicate_part_identity = compat_upload_denied(
        server.compat_http().upload(compat_upload_execution_input(
            &server,
            "files.avatar.upload",
            "boundary-duplicate",
            WorthServerMultipartUpload::new(
                WorthServerUploadManifest::new(single_insert_body("task-1"))
                    .with_file_part("avatar")
                    .with_file_part("avatar"),
            )
            .with_part(
                WorthServerUploadPart::file("avatar")
                    .with_content_type("image/png")
                    .with_declared_length(128),
            ),
        )),
    );
    let oversized_part = compat_upload_denied(
        server.compat_http().upload(compat_upload_execution_input(
            &server,
            "files.avatar.upload",
            "boundary-b",
            WorthServerMultipartUpload::new(manifest_for("task-1"))
                .with_part(
                    WorthServerUploadPart::file("avatar")
                        .with_content_type("image/png")
                        .with_declared_length(128),
                )
                .with_part(
                    WorthServerUploadPart::file("thumbnail")
                        .with_content_type("image/webp")
                        .with_declared_length(9 * 1024 * 1024),
                ),
        )),
    );
    let wrong_content_type = compat_upload_denied(
        server.compat_http().upload(
            worth_server::WorthServerCompatibilityUploadExecutionInput::new(
                prepared_request(
                    &server,
                    upload_input_with_content_type("files.avatar.upload", "application/json")
                        .build()
                        .expect("wrong-content-type upload input should validate structurally"),
                ),
                "files.avatar.upload",
                upload_order_alpha("task-1"),
            ),
        ),
    );

    assert_eq!(
        missing_part.code(),
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid
    );
    assert!(missing_part
        .detail()
        .contains("part graph did not match the declared manifest file-part set"));
    assert_eq!(
        missing_metadata.code(),
        WorthServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid
    );
    assert!(missing_metadata
        .detail()
        .contains("must define `command` or `commands`"));
    assert_eq!(
        duplicate_part_identity.code(),
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid
    );
    assert!(duplicate_part_identity
        .detail()
        .contains("manifest file part names may not repeat"));
    assert_eq!(
        oversized_part.code(),
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid
    );
    assert!(oversized_part
        .detail()
        .contains("exceeds the phase-five early-admission cap"));
    assert_eq!(
        wrong_content_type.code(),
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid
    );
    assert!(wrong_content_type
        .detail()
        .contains("expected multipart/form-data"));
}

#[test]
fn compat_http_upload_keeps_blob_transport_out_of_structured_truth_artifacts() {
    let server = build_phase_five_server();
    let upload = compat_upload_success(server.compat_http().upload(compat_upload_execution_input(
        &server,
        "files.avatar.upload",
        "boundary-blob",
        upload_order_alpha("task-11"),
    )));

    assert!(upload.upload().canonical_digest().contains("avatar"));
    assert!(upload.upload().canonical_digest().contains("image/png"));
    assert!(!upload
        .mutation()
        .mutation_request()
        .canonical_digest()
        .contains("avatar"));
    assert!(!upload
        .mutation()
        .envelope()
        .response_envelope()
        .canonical_digest()
        .contains("image/png"));
    assert!(!upload.mutation().canonical_digest().contains("thumbnail"));
}

#[test]
fn compat_http_upload_prepare_rejects_expect_continue_before_any_write_executes() {
    let attempted_writes = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let server = build_phase_five_server_with_workspace_provider(
        ProfiledCountingTestWorkspaceProvider::new(
            WorthQueryRuntimeSupportProfile::scaffold_backend_profile(),
            attempted_writes.clone(),
        ),
    );
    let denial = prepared_upload_denied(
        server.compat_http().prepare_upload(
            worth_server::WorthServerCompatibilityUploadExecutionInput::new(
                prepared_request(
                    &server,
                    upload_input_with_content_type(
                        "files.avatar.upload",
                        "multipart/form-data; boundary=expect",
                    )
                    .with_header("expect", "100-continue")
                    .build()
                    .expect("expect-continue upload input should validate structurally"),
                ),
                "files.avatar.upload",
                WorthServerMultipartUpload::new(WorthServerUploadManifest::new(json!(null)))
                    .with_expectation(WorthServerUploadExpectation::continue_required())
                    .with_part(
                        WorthServerUploadPart::file("avatar")
                            .with_content_type("image/png")
                            .with_declared_length(128),
                    ),
            ),
        ),
    );

    assert_eq!(
        denial.code(),
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid
    );
    assert!(denial
        .detail()
        .contains("manifest metadata must be a JSON object"));
    assert_eq!(
        attempted_writes.load(Ordering::Relaxed),
        0,
        "prepare_upload denials must occur before any authoritative write executes"
    );
}

#[test]
fn compat_http_upload_expect_continue_denies_unauthorized_requests_before_any_body_progression() {
    let server = build_phase_five_server_with_mutation_disabled();
    let denial = match server.compat_http().prepare_request(
        upload_input_with_content_type(
            "files.avatar.upload",
            "multipart/form-data; boundary=authz",
        )
        .with_header("expect", "100-continue")
        .build()
        .expect("unauthorized expect-continue upload input should validate structurally"),
    ) {
        worth_proof::TransitionOutcome::Denied(value) => value,
        other => panic!("expected upload request denial, got {other:?}"),
    };

    assert_eq!(
        denial.code(),
        worth_server::WorthServerCompatibilityDenialCode::MiddlewareDenied
    );
    assert!(denial
        .detail()
        .contains("query mutation intent is disabled"));
}

#[test]
fn compat_http_upload_requires_the_upload_route_family() {
    let server = build_phase_five_server();
    let denial = compat_upload_denied(
        server.compat_http().upload(
            worth_server::WorthServerCompatibilityUploadExecutionInput::new(
                prepared_request(
                    &server,
                    WorthServerCompatibilityRequestInput::builder()
                        .with_authenticated_principal_id("principal-7")
                        .with_tenant_id("tenant-a")
                        .with_workspace_id("workspace-42")
                        .with_branch_id("branch-9")
                        .with_route_family(WorthServerCompatHttpRouteFamily::Mutation)
                        .with_method("POST")
                        .with_path("/compat/mutations/files.avatar.upload")
                        .with_header("accept", "application/json")
                        .with_body_content_type("application/json")
                        .with_body_present(true)
                        .build()
                        .expect("mutation-shaped request should validate structurally"),
                ),
                "files.avatar.upload",
                upload_order_alpha("task-1"),
            ),
        ),
    );

    assert_eq!(
        denial.code(),
        WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid
    );
    assert!(denial.detail().contains("requires the upload route family"));
}

fn upload_input_with_content_type(
    operation_name: &str,
    content_type: &str,
) -> worth_server::WorthServerCompatibilityRequestInputBuilder {
    WorthServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(WorthServerCompatHttpRouteFamily::Upload)
        .with_method("POST")
        .with_path(format!("/compat/uploads/{operation_name}"))
        .with_header("accept", "application/json")
        .with_body_content_type(content_type)
        .with_body_present(true)
}
