use forge_proof::TransitionOutcome;
use forge_server::{
    ForgeServer, ForgeServerDirectDeliveryClass, ForgeServerDirectDeliveryRequest,
    ForgeServerDirectFreshnessMode, ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffInput,
    ForgeServerQueryHandoffOperation, ForgeServerQueryRequestedResume, ForgeServerResponseInput,
    ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

use crate::{
    certification_bundle::{
        ForgeServerCertificationBundle, ForgeServerCertificationOutputDigest as Output,
    },
    forge_native_assertions::{admitted_named_read, operator_evidence_record},
    query_handoff_fixture::{admit_read, request_input, resolve_request_context, success},
};

use super::forge_native_common::{
    direct_bundle, direct_delivery_denied, direct_delivery_success, direct_lease_success,
    forge_native_session_for_branch, forge_native_session_for_target, request_context_digest,
    support_posture_digest,
};

pub fn runtime_backed_delivery_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&session, operation_name);
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = runtime_backed_request(lease.resume_basis_digest());
    let delivery = direct_delivery_success(session.direct().negotiate_delivery(&lease, &request));
    direct_bundle(
        server,
        session.resolved_request_context().request_context(),
        delivery.response_envelope().clone(),
    )
    .with_output_digest(
        Output::SurfaceContract,
        delivery.downstream_delivery_contract().contract_digest(),
    )
    .with_output_digest(Output::Declaration, lease.declaration_digest())
    .with_output_digest(Output::Handoff, delivery.handoff_digest())
    .with_output_digest(
        Output::SupportPosture,
        support_posture_digest(delivery.support_posture()),
    )
    .with_output_digest(Output::Branch, delivery.direct_context().branch_digest())
    .with_output_digest(
        Output::Workspace,
        delivery.direct_context().workspace_digest(),
    )
    .with_output_digest(Output::Basis, lease.resume_basis_digest())
    .with_output_digest(Output::DeliveryRequest, delivery.request().request_digest())
    .with_output_digest(Output::ResumeMode, resume_mode_label(delivery.request()))
    .with_output_digest(
        Output::FreshnessMode,
        delivery.request().freshness_mode().as_str(),
    )
    .with_output_digest(
        Output::DeliveryClass,
        delivery.request().delivery_class().as_str(),
    )
}

pub fn compatibility_runtime_backed_delivery_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&session, operation_name);
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let requested_resume =
        ForgeServerQueryRequestedResume::runtime_backed(Some(lease.resume_basis_digest()));
    let request = delivery_request(requested_resume.clone());
    let request_context = resolve_request_context(
        server,
        request_input(
            ForgeServerSurfaceFamily::CompatHttp,
            ForgeServerTransportClass::CompatHttp,
        ),
    );
    let request_context_digest_value = request_context_digest(request_context.request_context());
    let admission = admit_read(server, request_context);
    let handoff = success(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admission,
                ForgeServerQueryHandoffOperation::downstream_delivery(
                    operation_name,
                    request.freshness_mode(),
                    request.delivery_class(),
                    requested_resume,
                ),
            )),
    );
    let branch_digest = handoff
        .admission()
        .request_context()
        .branch_target()
        .branch_digest();
    let workspace_digest = handoff
        .admission()
        .request_context()
        .workspace_target()
        .workspace_digest();
    let handoff_digest = handoff.canonical_digest().to_string();
    let support_posture = support_posture_digest(handoff.support_posture()).to_string();
    let surface_contract = handoff
        .downstream_delivery_contract()
        .contract_digest()
        .to_string();
    let response = server
        .responses()
        .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest_value,
        response.clone(),
        operator_evidence_record(server, response),
    )
    .with_output_digest(Output::SurfaceContract, surface_contract)
    .with_output_digest(Output::Declaration, declaration.declaration_digest())
    .with_output_digest(Output::Handoff, handoff_digest)
    .with_output_digest(Output::SupportPosture, support_posture)
    .with_output_digest(Output::Branch, branch_digest)
    .with_output_digest(Output::Workspace, workspace_digest)
    .with_output_digest(Output::Basis, lease.resume_basis_digest())
    .with_output_digest(Output::DeliveryRequest, request.request_digest())
    .with_output_digest(Output::ResumeMode, resume_mode_label(&request))
    .with_output_digest(Output::FreshnessMode, request.freshness_mode().as_str())
    .with_output_digest(Output::DeliveryClass, request.delivery_class().as_str())
}

pub fn durable_delivery_denial_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&session, operation_name);
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = delivery_request(ForgeServerQueryRequestedResume::durable());
    let denial = direct_delivery_denied(session.direct().negotiate_delivery(&lease, &request));
    denial_bundle(
        server,
        session.resolved_request_context().request_context(),
        denial,
        &lease,
        &request,
    )
}

pub fn compatibility_durable_delivery_denial_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&session, operation_name);
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = delivery_request(ForgeServerQueryRequestedResume::durable());
    let request_context = resolve_request_context(
        server,
        request_input(
            ForgeServerSurfaceFamily::CompatHttp,
            ForgeServerTransportClass::CompatHttp,
        ),
    );
    let durable_request_context = request_context.request_context().clone();
    let admission = admit_read(server, request_context);
    let denial = compatibility_delivery_denied(
        server,
        ForgeServerQueryHandoffInput::new(
            admission,
            ForgeServerQueryHandoffOperation::downstream_delivery(
                operation_name,
                request.freshness_mode(),
                request.delivery_class(),
                ForgeServerQueryRequestedResume::durable(),
            ),
        ),
    );
    denial_bundle(server, &durable_request_context, denial, &lease, &request)
}

pub fn runtime_backed_missing_basis_denial_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&session, operation_name);
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = delivery_request(ForgeServerQueryRequestedResume::runtime_backed(
        None::<String>,
    ));
    let denial = direct_delivery_denied(session.direct().negotiate_delivery(&lease, &request));
    denial_bundle(
        server,
        session.resolved_request_context().request_context(),
        denial,
        &lease,
        &request,
    )
}

pub fn runtime_backed_stale_basis_denial_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&session, operation_name);
    let lease = direct_lease_success(session.direct().declare_lease(&declaration));
    let request = delivery_request(ForgeServerQueryRequestedResume::runtime_backed(Some(
        "basis:drifted",
    )));
    let denial = direct_delivery_denied(session.direct().negotiate_delivery(&lease, &request));
    denial_bundle(
        server,
        session.resolved_request_context().request_context(),
        denial,
        &lease,
        &request,
    )
}

pub fn cross_workspace_lease_reuse_denial_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let main_session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&main_session, operation_name);
    let lease = direct_lease_success(main_session.direct().declare_lease(&declaration));
    let hostile_session = forge_native_session_for_target(server, Some("workspace-84"), None);
    let request = delivery_request(ForgeServerQueryRequestedResume::none());
    let denial = direct_delivery_denied(
        hostile_session
            .direct()
            .negotiate_delivery(&lease, &request),
    );
    denial_bundle(
        server,
        hostile_session.resolved_request_context().request_context(),
        denial,
        &lease,
        &request,
    )
}

pub fn cross_branch_lease_reuse_denial_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let main_session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&main_session, operation_name);
    let lease = direct_lease_success(main_session.direct().declare_lease(&declaration));
    let hostile_session = forge_native_session_for_target(server, None, Some("branch-9"));
    let request = delivery_request(ForgeServerQueryRequestedResume::none());
    let denial = direct_delivery_denied(
        hostile_session
            .direct()
            .negotiate_delivery(&lease, &request),
    );
    denial_bundle(
        server,
        hostile_session.resolved_request_context().request_context(),
        denial,
        &lease,
        &request,
    )
}

fn denial_bundle(
    server: &ForgeServer,
    request_context: &forge_server::ForgeServerRequestContext,
    denial: ForgeServerQueryHandoffDenial,
    lease: &forge_server::ForgeServerDirectLeaseDeclaration,
    request: &ForgeServerDirectDeliveryRequest,
) -> ForgeServerCertificationBundle {
    let denial_code = format!("{:?}", denial.code());
    let denial_detail = denial.detail().to_string();
    let response = server
        .responses()
        .shape_with_defaults(ForgeServerResponseInput::query_handoff_denied(denial));
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest(request_context),
        response.clone(),
        operator_evidence_record(server, response),
    )
    .with_output_digest(Output::Declaration, lease.declaration_digest())
    .with_output_digest(Output::Basis, lease.resume_basis_digest())
    .with_output_digest(
        Output::Branch,
        request_context.branch_target().branch_digest(),
    )
    .with_output_digest(
        Output::Workspace,
        request_context.workspace_target().workspace_digest(),
    )
    .with_output_digest(Output::DeliveryRequest, request.request_digest())
    .with_output_digest(Output::ResumeMode, resume_mode_label(request))
    .with_output_digest(Output::FreshnessMode, request.freshness_mode().as_str())
    .with_output_digest(Output::DeliveryClass, request.delivery_class().as_str())
    .with_output_digest(Output::DenialCode, denial_code)
    .with_output_digest(Output::DenialDetail, denial_detail)
}

fn delivery_request(
    requested_resume: ForgeServerQueryRequestedResume,
) -> ForgeServerDirectDeliveryRequest {
    ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        requested_resume,
    )
}

fn runtime_backed_request(basis_digest: &str) -> ForgeServerDirectDeliveryRequest {
    delivery_request(ForgeServerQueryRequestedResume::runtime_backed(Some(
        basis_digest.to_string(),
    )))
}

fn resume_mode_label(request: &ForgeServerDirectDeliveryRequest) -> &'static str {
    match request.requested_resume().kind() {
        forge_server::ForgeServerQueryRequestedResumeKind::None => "None",
        forge_server::ForgeServerQueryRequestedResumeKind::RuntimeBacked => "RuntimeBacked",
        forge_server::ForgeServerQueryRequestedResumeKind::Durable => "Durable",
    }
}

fn compatibility_delivery_denied(
    server: &ForgeServer,
    input: ForgeServerQueryHandoffInput,
) -> ForgeServerQueryHandoffDenial {
    match server.query_handoff().prepare(input) {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied compatibility downstream delivery, got {other:?}"),
    }
}
