use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeRemaskProjection, ForgeQueryRuntimeRemaskReasonKind,
    ForgeQueryRuntimeSupportProfile,
};
use forge_server::{
    request_context::DiagnosticRichnessProfile, ForgeServer, ForgeServerDirectProjectionRequest,
    ForgeServerForgeNativeSession, ForgeServerQuerySupportPosture, ForgeServerRequestContext,
    ForgeServerResponseEnvelope,
};

use crate::{
    direct_context_runtime::RemaskWorkspaceProvider,
    forge_native_assertions::operator_evidence_record,
    forge_native_runtime::{
        build_server, build_server_with_profiled_workspace, build_server_with_workspace_provider,
        forge_native_session_input_builder, server_with_request_context_default,
    },
};

use crate::certification_bundle::ForgeServerCertificationBundle;

pub fn standard_server() -> ForgeServer {
    build_server(true)
}

pub fn forensic_server() -> ForgeServer {
    server_with_request_context_default(DiagnosticRichnessProfile::Forensic)
}

pub fn remask_server() -> ForgeServer {
    build_server_with_workspace_provider(RemaskWorkspaceProvider::new(projection_remask()), true)
}

pub fn runtime_backed_server() -> ForgeServer {
    build_server_with_profiled_workspace(ForgeQueryRuntimeSupportProfile::scaffold_backend_profile())
}

pub fn durable_later_server() -> ForgeServer {
    build_server_with_profiled_workspace(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
                "durable restart and artifact reload stay deferred for direct certification lanes",
            ),
        ),
    )
}

pub fn forge_native_session_for_branch(
    server: &ForgeServer,
    branch_id: Option<&str>,
) -> ForgeServerForgeNativeSession {
    forge_native_session_for_target(server, None, branch_id)
}

pub fn forge_native_session_for_target(
    server: &ForgeServer,
    workspace_id: Option<&str>,
    branch_id: Option<&str>,
) -> ForgeServerForgeNativeSession {
    let mut builder = forge_native_session_input_builder();
    if let Some(workspace_id) = workspace_id {
        builder = builder.with_workspace_id(workspace_id);
    }
    if let Some(branch_id) = branch_id {
        builder = builder.with_branch_id(branch_id);
    }

    match server.forge_native().session(
        builder
            .build()
            .expect("forge-native certification session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected forge-native certification session, got {other:?}"),
    }
}

pub fn direct_bundle(
    server: &ForgeServer,
    request_context: &ForgeServerRequestContext,
    response: ForgeServerResponseEnvelope,
) -> ForgeServerCertificationBundle {
    ForgeServerCertificationBundle::from_response_and_evidence(
        request_context_digest(request_context),
        response.clone(),
        operator_evidence_record(server, response),
    )
}

pub fn request_context_digest(request_context: &ForgeServerRequestContext) -> String {
    format!(
        "principal={};tenant={};workspace={};branch={};diagnostics={:?}",
        request_context.authenticated_principal().principal_id(),
        request_context.workspace_target().tenant_id(),
        request_context.workspace_target().workspace_id(),
        request_context.branch_target().canonical_label(),
        request_context.diagnostics_profile(),
    )
}

pub fn support_posture_digest(posture: &ForgeServerQuerySupportPosture) -> String {
    match posture {
        ForgeServerQuerySupportPosture::ProductIndependent { label } => {
            format!("product-independent:{label}")
        }
        ForgeServerQuerySupportPosture::QueryReadSupported { family_contract } => {
            format!("query-read-supported:{}", family_contract.contract_digest())
        }
        ForgeServerQuerySupportPosture::DirectReadSupported { family_contract } => {
            format!(
                "direct-read-supported:{}",
                family_contract.contract_digest()
            )
        }
        ForgeServerQuerySupportPosture::DirectStateSupported { family_contract } => {
            format!(
                "direct-state-supported:{}",
                family_contract.contract_digest()
            )
        }
        ForgeServerQuerySupportPosture::DirectInspectionSupported { family_contract } => {
            format!(
                "direct-inspection-supported:{}",
                family_contract.contract_digest()
            )
        }
        ForgeServerQuerySupportPosture::DirectProjectionSupported { family_contract } => {
            format!(
                "direct-projection-supported:{}",
                family_contract.contract_digest()
            )
        }
        ForgeServerQuerySupportPosture::DirectMutationSupported { family_contract } => {
            format!(
                "direct-mutation-supported:{}",
                family_contract.contract_digest()
            )
        }
        ForgeServerQuerySupportPosture::QueryMutationSupported { family_contract } => {
            format!(
                "query-mutation-supported:{}",
                family_contract.contract_digest()
            )
        }
        ForgeServerQuerySupportPosture::DownstreamDeliverySupported {
            family_contract,
            runtime_resume_support_posture,
            durable_resume_support_posture,
            contract_digest,
        } => format!(
            "downstream-delivery-supported:{}:{}:{}:{contract_digest}",
            family_contract.contract_digest(),
            runtime_resume_support_posture.as_str(),
            durable_resume_support_posture.as_str(),
        ),
        ForgeServerQuerySupportPosture::RuntimeBackedResumeSupported {
            family_contract,
            runtime_resume_support_posture,
            support_digest,
            contract_digest,
        } => format!(
            "runtime-backed-resume-supported:{}:{}:{support_digest}:{contract_digest}",
            family_contract.contract_digest(),
            runtime_resume_support_posture.as_str(),
        ),
        ForgeServerQuerySupportPosture::DurableResumeSupported {
            family_contract,
            durable_resume_support_posture,
            support_digest,
            contract_digest,
        } => format!(
            "durable-resume-supported:{}:{}:{support_digest}:{contract_digest}",
            family_contract.contract_digest(),
            durable_resume_support_posture.as_str(),
        ),
    }
}

pub fn projection_request() -> ForgeServerDirectProjectionRequest {
    ForgeServerDirectProjectionRequest::new(
        "authorized-projection:test",
        "narrowed-result-shape:test",
        "policy:test",
        "tenant-schema:test",
        ["identity.id", "profile.display_name"],
    )
    .entity_identities()
    .view_local_identities()
    .display_field("profile.display_name")
}

fn projection_remask() -> ForgeQueryRuntimeRemaskProjection {
    ForgeQueryRuntimeRemaskProjection::remasked(
        ForgeQueryRuntimeRemaskReasonKind::PolicyDrift,
        "policy:test",
        "tenant-truth:test",
        "tenant-schema:test",
        "relationship-proof:test",
        "schema-context:test",
    )
}

pub fn direct_read_success(
    outcome: forge_server::ForgeServerDirectReadOutcome,
) -> forge_server::ForgeServerDirectRead {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct read, got {other:?}"),
    }
}

pub fn direct_read_denied(
    outcome: forge_server::ForgeServerDirectReadOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected denied direct read, got {other:?}"),
    }
}

pub fn direct_state_success(
    outcome: forge_server::ForgeServerDirectStateOutcome,
) -> forge_server::ForgeServerDirectState {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct state, got {other:?}"),
    }
}

pub fn direct_retained_posture_success(
    outcome: forge_proof::TransitionOutcome<
        forge_server::ForgeServerDirectRetainedPosture,
        forge_server::ForgeServerQueryHandoffDenial,
        forge_server::ForgeServerQueryHandoffDeferred,
        forge_server::ForgeServerQueryHandoffStale,
        forge_server::ForgeServerQueryHandoffRebindRequired,
        forge_server::ForgeServerQueryHandoffFailure,
    >,
) -> forge_server::ForgeServerDirectRetainedPosture {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful retained posture, got {other:?}"),
    }
}

pub fn direct_projection_success(
    outcome: forge_server::ForgeServerDirectProjectionOutcome,
) -> forge_server::ForgeServerDirectProjection {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct projection, got {other:?}"),
    }
}

pub fn direct_delivery_success(
    outcome: forge_server::ForgeServerDirectDeliveryOutcome,
) -> forge_server::ForgeServerDirectDeliveryContract {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct delivery contract, got {other:?}"),
    }
}

pub fn direct_delivery_denied(
    outcome: forge_server::ForgeServerDirectDeliveryOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected denied direct delivery contract, got {other:?}"),
    }
}

pub fn direct_lease_success(
    outcome: forge_server::ForgeServerDirectLeaseDeclarationOutcome,
) -> forge_server::ForgeServerDirectLeaseDeclaration {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct lease declaration, got {other:?}"),
    }
}
