use forge_proof::TransitionOutcome;
use forge_query::facade::{
    CompletedProjectionFactConsumption, ProjectionConsumptionBindingContext,
    ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError,
};
use forge_server::{
    ForgeServerDirectProjectionOutcome, ForgeServerDirectProjectionRequest,
    ForgeServerQueryHandoffDenialCode, ForgeServerSuccessKind,
};

use crate::{
    direct_context_runtime::RemaskWorkspaceProvider,
    forge_native_assertions::{admitted_named_read, forge_native_session},
    forge_native_runtime::{build_server, build_server_with_workspace_provider},
};

#[test]
fn direct_projection_consumes_retained_live_read_facts_with_typed_receipt_boundary() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let direct_read = direct_read_success(session.direct().read(&declaration));
    let request = projection_request()
        .entity_identities()
        .view_local_identities()
        .display_field("profile.display_name");
    let query_projection_attempt = direct_read
        .read_result()
        .consume_projection_facts_with_binding(
            query_binding(&request, direct_read.read_result().receipt()),
            request.requested_facts().clone(),
        )
        .expect("query projection path should stay typed");
    let query_projection = query_projection_attempt
        .completed()
        .expect("query projection should admit");

    let projection = direct_projection_success(session.direct().project(&declaration, &request));

    assert_eq!(
        projection
            .response_envelope()
            .success()
            .expect("success envelope")
            .payload()
            .kind(),
        ForgeServerSuccessKind::DirectProjection
    );
    assert_eq!(projection.facts().entity_identities().len(), 1);
    assert_eq!(projection.facts().view_local_identities().len(), 1);
    assert_eq!(projection.facts().display_fields().len(), 1);
    assert!(projection.warning_kinds().is_empty());
    assert_eq!(
        projection.materialization_digest().as_str(),
        query_projection
            .materialized_fact_posture()
            .map(|posture| posture.posture_digest())
            .unwrap_or(query_projection.receipt().receipt_digest())
    );
    assert_eq!(
        projection.basis_digest(),
        Some(
            direct_read
                .read_result()
                .receipt()
                .snapshot_identity()
                .terminal_projection_for_reporting()
                .as_str()
        )
    );
    assert_eq!(projection.policy_digest(), "policy:test");
    assert_eq!(
        projection.result_shape_digest(),
        direct_read.read_result().receipt().view_shape_digest()
    );
    assert_eq!(
        projection
            .projection_consumption_envelope()
            .integrity_digest(),
        projection.fact_receipt().integrity_digest()
    );
    assert_eq!(
        projection.fact_receipt().counter_snapshot_digest(),
        query_projection.receipt().counter_snapshot_digest()
    );
}

#[test]
fn direct_projection_denies_hidden_field_requests() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let request = ForgeServerDirectProjectionRequest::new(
        "authorized-projection:test",
        "narrowed-result-shape:test",
        "policy:test",
        "tenant-schema:test",
        ["identity.id"],
    )
    .display_field("profile.display_name");

    let denial = direct_projection_denied(session.direct().project(&declaration, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionDenied
    );
}

#[test]
fn direct_projection_denies_fact_families_live_reads_do_not_prove() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let request = projection_request().target_identity();

    let denial = direct_projection_denied(session.direct().project(&declaration, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionSourceMismatch
    );
}

#[test]
fn direct_projection_preserves_query_projection_receipt_parity() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let direct_read = direct_read_success(session.direct().read(&declaration));
    let request = projection_request()
        .entity_identities()
        .display_field("profile.display_name");

    let direct = direct_projection_success(session.direct().project(&declaration, &request));
    let query = query_projection_success(
        direct_read
            .read_result()
            .consume_projection_facts_with_binding(
                query_binding(&request, direct_read.read_result().receipt()),
                request.requested_facts().clone(),
            ),
    );

    assert_eq!(
        direct.fact_receipt().fact_set_digest(),
        query.receipt().fact_set_digest()
    );
    assert_eq!(
        direct.fact_receipt().receipt_digest(),
        query.receipt().receipt_digest()
    );
    assert_eq!(direct.basis_digest(), query.contract().basis_digest());
    assert_eq!(direct.policy_digest(), query.contract().policy_digest());
    assert_eq!(
        direct.result_shape_digest(),
        query.contract().canonical_result_shape_digest()
    );
    assert_eq!(
        direct.materialization_digest().as_str(),
        query
            .materialized_fact_posture()
            .map(|posture| posture.posture_digest())
            .unwrap_or(query.receipt().receipt_digest())
    );
    assert_eq!(
        direct.fact_receipt().counter_snapshot_digest(),
        query.receipt().counter_snapshot_digest()
    );
    assert_eq!(query.authority_reopen_count(), 0);
}

#[test]
fn direct_projection_preserves_materialized_remask_posture_in_direct_context() {
    let server = build_server_with_workspace_provider(
        RemaskWorkspaceProvider::new(ProjectionRemaskTestSupport::projection()),
        true,
    );
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let request = projection_request()
        .entity_identities()
        .display_field("profile.display_name");

    let projection = direct_projection_success(session.direct().project(&declaration, &request));

    assert_eq!(
        projection.direct_context().remask_posture().disposition(),
        forge_server::ForgeServerDirectRemaskDisposition::Remasked
    );
    assert_eq!(
        projection
            .direct_context()
            .remask_posture()
            .materialized_artifact()
            .expect("materialized remask artifact")
            .basis_digest(),
        projection
            .direct_context()
            .basis_digest()
            .expect("projection basis digest")
    );
    assert_eq!(
        projection.direct_context().remask_posture().remask_digest(),
        projection
            .facts()
            .materialized_fact_posture()
            .map(|posture| posture.posture_digest())
    );
}

#[test]
fn direct_projection_denies_requests_with_no_fact_families() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let request = ForgeServerDirectProjectionRequest::new(
        "authorized-projection:test",
        "narrowed-result-shape:test",
        "policy:test",
        "tenant-schema:test",
        ["identity.id", "profile.display_name"],
    );

    let denial = direct_projection_denied(session.direct().project(&declaration, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::DirectProjectionBindingInvalid
    );
}

#[test]
fn direct_projection_denies_missing_retained_live_view() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile.missing");
    let request = projection_request().entity_identities();

    let denial = direct_projection_denied(session.direct().project(&declaration, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::RetainedQueryArtifactUnavailable
    );
    assert_eq!(
        denial.detail(),
        "live view `users.profile.missing` has no retained subscription installation"
    );
}

#[test]
fn direct_projection_preserves_query_projection_denial_parity_for_hidden_fields() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let direct_read = direct_read_success(session.direct().read(&declaration));
    let request = ForgeServerDirectProjectionRequest::new(
        "authorized-projection:test",
        "narrowed-result-shape:test",
        "policy:test",
        "tenant-schema:test",
        ["identity.id"],
    )
    .display_field("profile.display_name");

    let direct_denial = direct_projection_denied(session.direct().project(&declaration, &request));
    let query_attempt = direct_read
        .read_result()
        .consume_projection_facts_with_binding(
            query_binding(&request, direct_read.read_result().receipt()),
            request.requested_facts().clone(),
        )
        .expect("query projection denial should stay typed");

    assert_eq!(
        direct_denial.code(),
        ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionDenied
    );
    match query_attempt {
        ProjectionFactConsumptionAttempt::Denied(denied) => {
            assert_eq!(direct_denial.detail(), format!("{:?}", denied.reason()));
        }
        other => panic!("expected denied query projection path, got {other:?}"),
    }
}

fn projection_request() -> ForgeServerDirectProjectionRequest {
    ForgeServerDirectProjectionRequest::new(
        "authorized-projection:test",
        "narrowed-result-shape:test",
        "policy:test",
        "tenant-schema:test",
        ["identity.id", "profile.display_name"],
    )
}

fn query_binding(
    request: &ForgeServerDirectProjectionRequest,
    receipt: &forge_query::facade::ForgeQueryLiveReadReceipt,
) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::from_projection_metadata(
        receipt.view_shape_digest(),
        receipt.query_digest(),
        receipt.view_shape_digest(),
        request.authorized_projection_identity(),
        request.narrowed_result_shape_digest(),
        request.policy_digest(),
        request.tenant_schema_basis_digest(),
        request.visible_fields().to_vec(),
    )
}

fn direct_read_success(
    outcome: forge_server::ForgeServerDirectReadOutcome,
) -> forge_server::ForgeServerDirectRead {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct read, got {other:?}"),
    }
}

fn direct_projection_success(
    outcome: ForgeServerDirectProjectionOutcome,
) -> forge_server::ForgeServerDirectProjection {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct projection, got {other:?}"),
    }
}

fn direct_projection_denied(
    outcome: ForgeServerDirectProjectionOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied direct projection, got {other:?}"),
    }
}

fn query_projection_success(
    outcome: Result<ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError>,
) -> CompletedProjectionFactConsumption {
    match outcome.expect("query projection path should stay typed") {
        ProjectionFactConsumptionAttempt::Admitted(value)
        | ProjectionFactConsumptionAttempt::AdmittedWithWarnings(value, _) => value,
        other => panic!("expected admitted query projection path, got {other:?}"),
    }
}

struct ProjectionRemaskTestSupport;

impl ProjectionRemaskTestSupport {
    fn projection() -> forge_query::facade::ForgeQueryRuntimeRemaskProjection {
        forge_query::facade::ForgeQueryRuntimeRemaskProjection::remasked(
            forge_query::facade::ForgeQueryRuntimeRemaskReasonKind::PolicyDrift,
            "policy:test",
            "tenant-truth:test",
            "tenant-schema:test",
            "relationship-proof:test",
            "schema-context:test",
        )
    }
}
