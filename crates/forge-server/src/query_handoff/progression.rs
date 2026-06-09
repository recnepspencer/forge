use forge_foundational::DiagnosticRichnessProfile;
use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryWorkspace,
};

use crate::{config::ForgeServerQueryHandoffConfig, ForgeServerPreparedQueryHandoffKind};

use super::{
    ForgeServerQueryHandoff, ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerQueryRequestedResume, ForgeServerQuerySupportPosture,
    ForgeServerQueryWorkspaceBindingRequest,
};

pub(crate) fn prepare_query_handoff(
    config: &ForgeServerQueryHandoffConfig,
    input: ForgeServerQueryHandoffInput,
) -> super::ForgeServerQueryHandoffOutcome {
    let (admission, operation) = input.into_parts();
    let diagnostics_profile = admission.request_context().diagnostics_profile();

    if let Some(denial) = validate_prepared_intent(&admission, &operation) {
        return TransitionOutcome::Denied(denial);
    }

    let binding_request = ForgeServerQueryWorkspaceBindingRequest::for_query_handoff(
        admission.resolved_request_context().clone(),
        operation.clone(),
    );
    let workspace = match config.workspace_provider().bind_workspace(&binding_request) {
        Ok(workspace) => workspace,
        Err(error) => {
            return TransitionOutcome::Denied(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::WorkspaceBindingFailed,
                diagnostics_profile,
                format!("{}: {}", error.stage(), error.message()),
            ));
        }
    };

    let downstream_delivery_contract = workspace.public_downstream_delivery_contract();
    let support_posture = match derive_support_posture(
        admission.query_handoff_intent().kind(),
        &operation,
        &workspace,
        &downstream_delivery_contract,
        diagnostics_profile,
    ) {
        Ok(support_posture) => support_posture,
        Err(denial) => return TransitionOutcome::Denied(denial),
    };

    let canonical_digest = canonical_digest(
        admission
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .tenant_id(),
        admission
            .resolved_request_context()
            .request_context()
            .workspace_target()
            .workspace_id(),
        workspace.name(),
        &operation,
        &support_posture,
        downstream_delivery_contract.contract_digest(),
    );

    TransitionOutcome::Success(ForgeServerQueryHandoff::new(
        admission,
        operation,
        workspace,
        downstream_delivery_contract,
        support_posture,
        canonical_digest,
    ))
}

fn validate_prepared_intent(
    admission: &crate::ForgeServerAdmission,
    operation: &ForgeServerQueryHandoffOperation,
) -> Option<ForgeServerQueryHandoffDenial> {
    let prepared = admission.query_handoff_intent();
    match operation {
        ForgeServerQueryHandoffOperation::QueryRead { operation_name }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::QueryRead
                && prepared.operation_name() == operation_name =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::QueryMutation { operation_name }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::QueryMutation
                && prepared.operation_name() == operation_name =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::DirectRead { .. }
        | ForgeServerQueryHandoffOperation::DirectState { .. }
        | ForgeServerQueryHandoffOperation::DirectInspection { .. }
        | ForgeServerQueryHandoffOperation::DirectProjection { .. }
        | ForgeServerQueryHandoffOperation::DirectMutation { .. }
        | ForgeServerQueryHandoffOperation::DownstreamDelivery { .. }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::ForgeNativeSession =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::DirectRead { .. }
        | ForgeServerQueryHandoffOperation::DirectState { .. }
        | ForgeServerQueryHandoffOperation::DirectInspection { .. }
        | ForgeServerQueryHandoffOperation::DirectProjection { .. }
            if prepared.kind() == ForgeServerPreparedQueryHandoffKind::QueryRead =>
        {
            None
        }
        ForgeServerQueryHandoffOperation::DownstreamDelivery { .. } => None,
        _ => Some(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::PreparedIntentMismatch,
            admission.request_context().diagnostics_profile(),
            "query handoff operation does not match the middleware-admitted prepared intent",
        )),
    }
}

fn derive_support_posture(
    prepared_kind: crate::ForgeServerPreparedQueryHandoffKind,
    operation: &ForgeServerQueryHandoffOperation,
    workspace: &ForgeQueryWorkspace,
    contract: &ForgeQueryRuntimeDownstreamDeliveryContract,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<ForgeServerQuerySupportPosture, ForgeServerQueryHandoffDenial> {
    if prepared_kind == ForgeServerPreparedQueryHandoffKind::ForgeNativeSession {
        match operation {
            ForgeServerQueryHandoffOperation::DirectRead { .. }
            | ForgeServerQueryHandoffOperation::DirectState { .. }
            | ForgeServerQueryHandoffOperation::DirectInspection { .. }
            | ForgeServerQueryHandoffOperation::DirectProjection { .. }
            | ForgeServerQueryHandoffOperation::DirectMutation { .. }
            | ForgeServerQueryHandoffOperation::DownstreamDelivery { .. } => {}
            _ => {
                return Err(ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::PreparedIntentMismatch,
                    diagnostics_profile,
                    "forge-native session entry only supports direct read/state/inspection/projection/mutation/downstream-delivery handoff operations",
                ));
            }
        }
    }

    match operation {
        ForgeServerQueryHandoffOperation::QueryRead { .. } => {
            Ok(ForgeServerQuerySupportPosture::QueryReadSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Read,
                    diagnostics_profile,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectRead { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectReadSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Read,
                    diagnostics_profile,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectState { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectStateSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Live,
                    diagnostics_profile,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectInspection { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectInspectionSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Inspect,
                    diagnostics_profile,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectProjection { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectProjectionSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Read,
                    diagnostics_profile,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectMutation { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectMutationSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Write,
                    diagnostics_profile,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::QueryMutation { .. } => {
            Ok(ForgeServerQuerySupportPosture::QueryMutationSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Write,
                    diagnostics_profile,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DownstreamDelivery {
            requested_resume, ..
        } => {
            if !matches!(
                prepared_kind,
                ForgeServerPreparedQueryHandoffKind::QueryRead
                    | ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
            ) {
                return Err(ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::DownstreamDeliveryRequiresReadIntent,
                    diagnostics_profile,
                    "downstream delivery handoff requires a read-admitted middleware intent or a forge-native direct session",
                ));
            }

            let family_contract = admit_query_family(
                workspace,
                ForgeQueryRuntimeFacadeFamily::Live,
                diagnostics_profile,
            )?;
            match requested_resume {
                ForgeServerQueryRequestedResume::None => Ok(
                    ForgeServerQuerySupportPosture::DownstreamDeliverySupported {
                        family_contract,
                        runtime_resume_support_posture: contract.runtime_resume_support_posture(),
                        durable_resume_support_posture: contract.durable_resume_support_posture(),
                        contract_digest: contract.contract_digest().to_string(),
                    },
                ),
                ForgeServerQueryRequestedResume::RuntimeBacked { .. }
                    if contract.runtime_backed_resume_supported() =>
                {
                    Ok(
                        ForgeServerQuerySupportPosture::RuntimeBackedResumeSupported {
                            family_contract,
                            runtime_resume_support_posture: contract
                                .runtime_resume_support_posture(),
                            support_digest: contract.runtime_resume_support_digest().to_string(),
                            contract_digest: contract.contract_digest().to_string(),
                        },
                    )
                }
                ForgeServerQueryRequestedResume::RuntimeBacked { .. } => {
                    Err(ForgeServerQueryHandoffDenial::new(
                        ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeUnsupported,
                        diagnostics_profile,
                        format!(
                            "runtime-backed resume posture is {:?}",
                            contract.runtime_resume_support_posture()
                        ),
                    ))
                }
                ForgeServerQueryRequestedResume::Durable if contract.durable_resume_deferred() => {
                    Err(ForgeServerQueryHandoffDenial::new(
                        ForgeServerQueryHandoffDenialCode::DurableResumeDeferred,
                        diagnostics_profile,
                        format!(
                            "durable resume remains deferred with digest {}",
                            contract.durable_resume_support_digest()
                        ),
                    ))
                }
                ForgeServerQueryRequestedResume::Durable => {
                    Ok(ForgeServerQuerySupportPosture::DurableResumeSupported {
                        family_contract,
                        durable_resume_support_posture: contract.durable_resume_support_posture(),
                        support_digest: contract.durable_resume_support_digest().to_string(),
                        contract_digest: contract.contract_digest().to_string(),
                    })
                }
            }
        }
    }
}

fn admit_query_family(
    workspace: &ForgeQueryWorkspace,
    family: ForgeQueryRuntimeFacadeFamily,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<ForgeQueryRuntimePublicApiFamilyContract, ForgeServerQueryHandoffDenial> {
    workspace.admit_public_api_family(family).map_err(|error| {
        ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily,
            diagnostics_profile,
            format!(
                "query workspace does not admit `{}` facade family: {error}",
                family.as_str()
            ),
        )
    })
}

fn canonical_digest(
    tenant_id: &str,
    workspace_id: &str,
    workspace_name: &str,
    operation: &ForgeServerQueryHandoffOperation,
    support_posture: &ForgeServerQuerySupportPosture,
    contract_digest: &str,
) -> String {
    format!(
        "forge-server-query-handoff-v1|tenant:{tenant_id}|workspace:{workspace_id}|bound:{workspace_name}|operation:{}|support:{}|contract:{contract_digest}",
        operation.canonical_label(),
        support_posture.canonical_label(),
    )
}
