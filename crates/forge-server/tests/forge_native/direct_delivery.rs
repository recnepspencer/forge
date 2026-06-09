use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeSupportProfile,
};
use forge_server::{
    ForgeServerDirectDeliveryClass, ForgeServerDirectDeliveryOutcome,
    ForgeServerDirectDeliveryRequest, ForgeServerDirectFreshnessMode,
    ForgeServerDirectLeaseDeclarationOutcome, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerQueryRequestedResume, ForgeServerSuccessKind, ForgeServerSurfaceFamily,
    ForgeServerTransportClass,
};

use crate::{
    direct_delivery_runtime::DeclarationAdmitsButLiveDeliveryDeniesProvider,
    forge_native_assertions::{
        admitted_named_read, family_contract_digest, forge_native_session, operator_evidence_record,
    },
    forge_native_runtime::{build_server, build_server_with_workspace_provider},
    query_handoff_fixture::{admit_read, request_input, resolve_request_context, success},
    query_handoff_runtime::TestWorkspaceProvider,
};

#[test]
fn direct_lease_declaration_preserves_admitted_declaration_identity() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");

    let lease = direct_lease_success(session.direct().declare_lease(&declaration));

    assert_eq!(lease.workspace_name(), declaration.workspace_name());
    assert_eq!(lease.principal_id(), "principal-7");
    assert_eq!(lease.tenant_id(), "tenant-a");
    assert_eq!(lease.workspace_id(), "workspace-42");
    assert_eq!(lease.branch_label(), "main");
    assert!(!lease.resume_basis_digest().is_empty());
    assert_eq!(lease.declaration_digest(), declaration.declaration_digest());
    assert_eq!(lease.declaration_binding_label(), "users.profile");
    assert_eq!(
        lease.declaration_canonical_label(),
        "named-read:users.profile"
    );
    assert_eq!(
        lease.support_digest(),
        declaration.support_snapshot().support_posture_digest()
    );
    assert_eq!(lease.lease_digest(), lease.canonical_digest());
}

#[test]
fn direct_lease_declarations_compare_equal_for_equivalent_inputs_and_unequal_for_distinct_views() {
    let server = build_server(true);
    let session_a = forge_native_session(&server);
    let session_b = forge_native_session(&server);
    let declaration_a = admitted_named_read(&session_a, "users.profile");
    let declaration_b = admitted_named_read(&session_b, "users.profile");
    let declaration_c = admitted_named_read(&session_b, "users.profile.saved");

    let lease_a = direct_lease_success(session_a.direct().declare_lease(&declaration_a));
    let lease_b = direct_lease_success(session_b.direct().declare_lease(&declaration_b));
    let lease_c = direct_lease_success(session_b.direct().declare_lease(&declaration_c));

    assert_eq!(lease_a.lease_digest(), lease_b.lease_digest());
    assert_eq!(lease_a.support_digest(), lease_b.support_digest());
    assert_ne!(lease_a.lease_digest(), lease_c.lease_digest());
}

#[test]
fn direct_lease_declaration_denies_missing_retained_artifact() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile.missing");

    let denial = direct_lease_denied(session.direct().declare_lease(&declaration));

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
fn direct_delivery_contract_preserves_query_handoff_posture_parity() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = direct_request(ForgeServerQueryRequestedResume::none());

    let direct = direct_delivery_success(session.direct().negotiate_delivery(&lease, &request));
    let compat = success(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admit_read(
                    &server,
                    resolve_request_context(
                        &server,
                        request_input(
                            ForgeServerSurfaceFamily::CompatHttp,
                            ForgeServerTransportClass::CompatHttp,
                        ),
                    ),
                ),
                ForgeServerQueryHandoffOperation::downstream_delivery(
                    "users.profile",
                    ForgeServerDirectFreshnessMode::LiveStrict,
                    ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
                    ForgeServerQueryRequestedResume::none(),
                ),
            )),
    );

    assert_eq!(
        family_contract_digest(direct.support_posture()),
        family_contract_digest(compat.support_posture())
    );
    assert_eq!(direct.workspace_name(), compat.workspace().name());
    assert_eq!(direct.handoff_digest(), compat.canonical_digest());
    assert_eq!(
        direct.downstream_delivery_contract().contract_digest(),
        compat.downstream_delivery_contract().contract_digest()
    );
    assert_eq!(
        direct.runtime_resume_support_posture(),
        compat
            .downstream_delivery_contract()
            .runtime_resume_support_posture()
    );
    assert_eq!(
        direct.durable_resume_support_posture(),
        compat
            .downstream_delivery_contract()
            .durable_resume_support_posture()
    );
    assert_eq!(
        direct
            .response_envelope()
            .success()
            .expect("success envelope")
            .payload()
            .kind(),
        ForgeServerSuccessKind::DownstreamDelivery
    );
}

#[test]
fn direct_delivery_runtime_backed_resume_stays_explicit_and_stable() {
    let server = build_server(true);
    let session_a = forge_native_session(&server);
    let session_b = forge_native_session(&server);
    let declaration_a = admitted_named_read(&session_a, "users.profile");
    let declaration_b = admitted_named_read(&session_b, "users.profile");
    let lease_a = direct_lease_success(session_a.direct().declare_lease(&declaration_a));
    let lease_b = direct_lease_success(session_b.direct().declare_lease(&declaration_b));
    let request = ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        ForgeServerQueryRequestedResume::runtime_backed(Some(lease_a.resume_basis_digest())),
    );

    let direct_a =
        direct_delivery_success(session_a.direct().negotiate_delivery(&lease_a, &request));
    let direct_b =
        direct_delivery_success(session_b.direct().negotiate_delivery(&lease_b, &request));

    assert!(direct_a.runtime_backed_resume_supported());
    assert!(!direct_a.durable_resume_supported());
    assert_eq!(direct_a.handoff_digest(), direct_b.handoff_digest());
    assert_eq!(direct_a.canonical_digest(), direct_b.canonical_digest());
    assert_eq!(
        direct_a.request().canonical_digest(),
        direct_b.request().canonical_digest()
    );
    assert_eq!(
        direct_a.runtime_resume_support_posture(),
        direct_b.runtime_resume_support_posture()
    );
}

#[test]
fn direct_delivery_request_identity_tracks_freshness_class_and_resume_axes() {
    let strict_authoritative = ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        ForgeServerQueryRequestedResume::none(),
    );
    let coalesced_authoritative = ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveCoalesced,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        ForgeServerQueryRequestedResume::none(),
    );
    let strict_replaceable = ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::ReplaceableLatestState,
        ForgeServerQueryRequestedResume::none(),
    );
    let strict_runtime_backed = ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        ForgeServerQueryRequestedResume::runtime_backed(Some("basis:42")),
    );

    assert_eq!(
        strict_authoritative.request_digest(),
        strict_authoritative.canonical_digest()
    );
    assert_eq!(
        ForgeServerDirectFreshnessMode::LiveStrict.as_str(),
        "live_strict"
    );
    assert_eq!(
        ForgeServerDirectFreshnessMode::PresenceOnly.as_str(),
        "presence_only"
    );
    assert_ne!(
        strict_authoritative.request_digest(),
        coalesced_authoritative.request_digest()
    );
    assert_ne!(
        strict_authoritative.request_digest(),
        strict_replaceable.request_digest()
    );
    assert_ne!(
        strict_authoritative.request_digest(),
        strict_runtime_backed.request_digest()
    );
}

#[test]
fn direct_delivery_denies_durable_resume_debt() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = direct_request(ForgeServerQueryRequestedResume::durable());

    let denial = direct_delivery_denied(session.direct().negotiate_delivery(&lease, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::DurableResumeDeferred
    );
    assert!(denial.detail().contains("durable resume remains deferred"));
}

#[test]
fn direct_delivery_denies_runtime_backed_resume_without_basis() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = direct_request(ForgeServerQueryRequestedResume::runtime_backed(
        None::<String>,
    ));

    let denial = direct_delivery_denied(session.direct().negotiate_delivery(&lease, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeMissingBasis
    );
    assert!(denial.detail().contains(lease.resume_basis_digest()));
}

#[test]
fn direct_delivery_denies_runtime_backed_resume_with_stale_basis() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = direct_request(ForgeServerQueryRequestedResume::runtime_backed(Some(
        "basis:drifted",
    )));

    let denial = direct_delivery_denied(session.direct().negotiate_delivery(&lease, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeStaleBasis
    );
    assert!(denial.detail().contains("basis:drifted"));
    assert!(denial.detail().contains(lease.resume_basis_digest()));
}

#[test]
fn direct_delivery_fails_closed_when_live_family_is_not_admitted() {
    let server = build_server_with_workspace_provider(
        DeclarationAdmitsButLiveDeliveryDeniesProvider::new(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
                ForgeQueryRuntimeFamilySupport::unsupported(
                    ForgeQueryRuntimeFacadeFamily::Live,
                    "live delivery is intentionally denied in this hostile test profile",
                ),
            ),
        ),
        true,
    );
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = direct_request(ForgeServerQueryRequestedResume::none());

    let denial = direct_delivery_denied(session.direct().negotiate_delivery(&lease, &request));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
    );
    assert!(denial
        .detail()
        .contains("query workspace does not admit `live` facade family"));
}

#[test]
fn direct_delivery_success_records_downstream_operator_evidence() {
    let server = build_server(true);
    let session = forge_native_session(&server);
    let declaration = admitted_named_read(&session, "users.profile");
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = direct_request(ForgeServerQueryRequestedResume::none());

    let delivery = direct_delivery_success(session.direct().negotiate_delivery(&lease, &request));
    let evidence = operator_evidence_record(&server, delivery.response_envelope().clone());

    assert_eq!(
        evidence.classification(),
        &forge_server::ForgeServerOperatorEvidenceClass::DownstreamDeliverySucceeded
    );
}

#[test]
fn direct_delivery_denies_cross_workspace_lease_reuse() {
    let server_a = build_server(true);
    let server_b = build_server_with_workspace_provider(TestWorkspaceProvider, true);
    let session_a = forge_native_session(&server_a);
    let declaration_a = admitted_named_read(&session_a, "users.profile");
    let lease = direct_lease_success(session_a.direct().declare_lease(&declaration_a));
    let session_b = match server_b.forge_native().session(
        crate::forge_native_runtime::forge_native_session_input_builder()
            .with_workspace_id("workspace-84")
            .build()
            .expect("session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected forge-native session, got {other:?}"),
    };

    let denial = direct_delivery_denied(session_b.direct().negotiate_delivery(
        &lease,
        &direct_request(ForgeServerQueryRequestedResume::none()),
    ));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::LeaseDeclarationContextMismatch
    );
    assert!(denial.detail().contains("workspace=`workspace-42`"));
}

#[test]
fn direct_delivery_denies_cross_branch_lease_reuse() {
    let server = build_server(true);
    let main_session = forge_native_session(&server);
    let declaration = admitted_named_read(&main_session, "users.profile");
    let lease = direct_lease_success(main_session.direct().declare_lease(&declaration));
    let branch_session = match server.forge_native().session(
        crate::forge_native_runtime::forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("branch session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected branch forge-native session, got {other:?}"),
    };

    let denial = direct_delivery_denied(branch_session.direct().negotiate_delivery(
        &lease,
        &direct_request(ForgeServerQueryRequestedResume::none()),
    ));

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::LeaseDeclarationContextMismatch
    );
    assert!(denial.detail().contains("branch=`main`"));
    assert!(denial.detail().contains("branch=`branch:branch-9`"));
}

fn direct_request(
    requested_resume: ForgeServerQueryRequestedResume,
) -> ForgeServerDirectDeliveryRequest {
    ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        requested_resume,
    )
}

fn direct_delivery_success(
    outcome: ForgeServerDirectDeliveryOutcome,
) -> forge_server::ForgeServerDirectDeliveryContract {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct delivery contract, got {other:?}"),
    }
}

fn direct_lease_success(
    outcome: ForgeServerDirectLeaseDeclarationOutcome,
) -> forge_server::ForgeServerDirectLeaseDeclaration {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct lease declaration, got {other:?}"),
    }
}

fn direct_lease_denied(
    outcome: ForgeServerDirectLeaseDeclarationOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied direct lease declaration, got {other:?}"),
    }
}

fn direct_delivery_denied(
    outcome: ForgeServerDirectDeliveryOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied direct delivery contract, got {other:?}"),
    }
}
