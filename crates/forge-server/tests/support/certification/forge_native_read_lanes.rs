use forge_server::{
    ForgeServer, ForgeServerDirectDeclaration, ForgeServerDirectViewShape,
    ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation, ForgeServerResponseInput,
    ForgeServerSurfaceFamily, ForgeServerTransportClass,
};

use crate::{
    certification_bundle::ForgeServerCertificationOutputDigest as Output,
    forge_native_assertions::{admitted_named_read, family_contract_digest},
    query_handoff_fixture::{admit_read_posture, request_input, resolve_request_context, success},
};

use crate::certification_bundle::ForgeServerCertificationBundle;

use super::forge_native_common::{
    direct_bundle, direct_read_denied, direct_read_success, forge_native_session_for_branch,
    request_context_digest, support_posture_digest,
};

pub fn product_read_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    product_read_bundle_with_branch_and_shape(
        server,
        operation_name,
        None,
        ForgeServerDirectViewShape::Detail,
    )
}

pub fn branch_product_read_bundle(
    server: &ForgeServer,
    operation_name: &str,
    branch_id: &str,
) -> ForgeServerCertificationBundle {
    product_read_bundle_with_branch_and_shape(
        server,
        operation_name,
        Some(branch_id),
        ForgeServerDirectViewShape::Detail,
    )
}

pub fn view_shape_product_read_bundle(
    server: &ForgeServer,
    operation_name: &str,
    view_shape: ForgeServerDirectViewShape,
) -> ForgeServerCertificationBundle {
    product_read_bundle_with_branch_and_shape(server, operation_name, None, view_shape)
}

pub fn lower_direct_read_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&session, operation_name);
    let read = direct_read_success(session.direct().read(&declaration));
    direct_bundle(
        server,
        session.resolved_request_context().request_context(),
        read.response_envelope().clone(),
    )
    .with_output_digest(
        Output::SurfaceContract,
        declaration.query_family_contract().contract_digest(),
    )
    .with_output_digest(Output::Declaration, declaration.declaration_digest())
    .with_output_digest(
        Output::DeclarationSupport,
        declaration.support_snapshot().support_posture_digest(),
    )
    .with_output_digest(Output::Handoff, read.handoff_digest())
    .with_output_digest(
        Output::SupportPosture,
        support_posture_digest(read.support_posture()),
    )
    .with_output_digest(Output::Branch, read.direct_context().branch_digest())
    .with_output_digest(Output::Workspace, read.direct_context().workspace_digest())
    .with_output_digest(
        Output::SupportMatrix,
        declaration.support_snapshot().support_matrix_digest(),
    )
    .with_output_digest(
        Output::ViewShape,
        format!("{:?}", declaration.support_snapshot().view_shape()),
    )
}

pub fn compatibility_overlap_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let direct_session = forge_native_session_for_branch(server, None);
    let declaration = admitted_named_read(&direct_session, operation_name);
    let request_context = resolve_request_context(
        server,
        request_input(
            ForgeServerSurfaceFamily::CompatHttp,
            ForgeServerTransportClass::CompatHttp,
        ),
    );
    let handoff = success(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admit_read_posture(server, request_context),
                ForgeServerQueryHandoffOperation::query_read(operation_name),
            )),
    );
    let request_context_digest = request_context_digest(handoff.admission().request_context());
    let surface_contract_digest = family_contract_digest(handoff.support_posture()).to_string();
    let support_posture_digest = support_posture_digest(handoff.support_posture());
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
    let response = server
        .responses()
        .shape_with_defaults(ForgeServerResponseInput::query_handoff_success(handoff));
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest,
        response.clone(),
        crate::forge_native_assertions::operator_evidence_record(server, response),
    )
    .with_output_digest(Output::SurfaceContract, surface_contract_digest)
    .with_output_digest(Output::Declaration, declaration.declaration_digest())
    .with_output_digest(
        Output::DeclarationSupport,
        declaration.support_snapshot().support_posture_digest(),
    )
    .with_output_digest(Output::SupportPosture, support_posture_digest)
    .with_output_digest(Output::Branch, branch_digest)
    .with_output_digest(Output::Workspace, workspace_digest)
}

pub fn retained_artifact_denial_bundle(
    server: &ForgeServer,
    operation_name: &str,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, None);
    let product = session
        .direct()
        .product()
        .named_read(operation_name)
        .expect("missing operation should still admit declaration shape");
    let denial = direct_read_denied(product.read());
    let response = server
        .responses()
        .shape_with_defaults(ForgeServerResponseInput::query_handoff_denied(denial));
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest(session.resolved_request_context().request_context()),
        response.clone(),
        crate::forge_native_assertions::operator_evidence_record(server, response),
    )
}

pub fn saved_query_intake_denial() -> forge_server::ForgeServerDirectDeclarationDenial {
    let server = super::forge_native_common::standard_server();
    let session = forge_native_session_for_branch(&server, None);
    session
        .direct()
        .product()
        .read(ForgeServerDirectDeclaration::saved_query(
            "users.profile.saved",
        ))
        .expect_err("saved query should remain denied at declaration intake")
}

fn product_read_bundle_with_branch_and_shape(
    server: &ForgeServer,
    operation_name: &str,
    branch_id: Option<&str>,
    view_shape: ForgeServerDirectViewShape,
) -> ForgeServerCertificationBundle {
    let session = forge_native_session_for_branch(server, branch_id);
    let product = session
        .direct()
        .product()
        .read(ForgeServerDirectDeclaration::named_read(operation_name).with_view_shape(view_shape))
        .expect("product read should admit");
    let read = direct_read_success(product.read());
    let snapshot = product.declaration_snapshot();
    direct_bundle(
        server,
        session.resolved_request_context().request_context(),
        read.response_envelope().clone(),
    )
    .with_output_digest(Output::SurfaceContract, snapshot.family_contract_digest())
    .with_output_digest(Output::Declaration, snapshot.declaration_digest())
    .with_output_digest(
        Output::DeclarationSupport,
        snapshot.support_posture_digest(),
    )
    .with_output_digest(Output::Handoff, read.handoff_digest())
    .with_output_digest(
        Output::SupportPosture,
        support_posture_digest(read.support_posture()),
    )
    .with_output_digest(Output::Branch, read.direct_context().branch_digest())
    .with_output_digest(Output::Workspace, read.direct_context().workspace_digest())
    .with_output_digest(Output::SupportMatrix, snapshot.support_matrix_digest())
    .with_output_digest(Output::ViewShape, format!("{:?}", snapshot.view_shape()))
}
