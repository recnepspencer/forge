use forge_foundational::DiagnosticRichnessProfile;
use forge_query::facade::{
    ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryWorkspace,
};

use crate::{
    ForgeServerPreparedQueryHandoffKind, ForgeServerQueryHandoffOperation,
    ForgeServerQueryRequestedResume, ForgeServerQuerySupportPosture,
};

use super::{ForgeServerOperationReadinessDenial, ForgeServerOperationReadinessDenialCode};

pub(crate) fn derive_query_support_posture(
    prepared_kind: ForgeServerPreparedQueryHandoffKind,
    operation: &ForgeServerQueryHandoffOperation,
    workspace: &ForgeQueryWorkspace,
    contract: &ForgeQueryRuntimeDownstreamDeliveryContract,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<ForgeServerQuerySupportPosture, ForgeServerOperationReadinessDenial> {
    if prepared_kind == ForgeServerPreparedQueryHandoffKind::ForgeNativeSession {
        match operation {
            ForgeServerQueryHandoffOperation::QueryRead { .. }
            | ForgeServerQueryHandoffOperation::QueryMutation { .. } => {
                return Err(ForgeServerOperationReadinessDenial::new(
                    ForgeServerOperationReadinessDenialCode::UnsupportedQuerySupport,
                    format!(
                        "forge-native session entry does not support `{}` query handoff posture under {:?} diagnostics",
                        operation.canonical_label(),
                        diagnostics_profile,
                    ),
                ));
            }
            _ => {}
        }
    }

    match operation {
        ForgeServerQueryHandoffOperation::QueryRead { .. } => {
            Ok(ForgeServerQuerySupportPosture::QueryReadSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Read,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectRead { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectReadSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Read,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectState { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectStateSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Live,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectInspection { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectInspectionSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Inspect,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectProjection { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectProjectionSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Read,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DirectMutation { .. } => {
            Ok(ForgeServerQuerySupportPosture::DirectMutationSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Write,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::QueryMutation { .. } => {
            Ok(ForgeServerQuerySupportPosture::QueryMutationSupported {
                family_contract: admit_query_family(
                    workspace,
                    ForgeQueryRuntimeFacadeFamily::Write,
                )?,
            })
        }
        ForgeServerQueryHandoffOperation::DownstreamDelivery {
            requested_resume, ..
        } => {
            let family_contract =
                admit_query_family(workspace, ForgeQueryRuntimeFacadeFamily::Live)?;
            derive_delivery_support(prepared_kind, requested_resume, contract, family_contract)
        }
    }
}

fn derive_delivery_support(
    prepared_kind: ForgeServerPreparedQueryHandoffKind,
    requested_resume: &ForgeServerQueryRequestedResume,
    contract: &ForgeQueryRuntimeDownstreamDeliveryContract,
    family_contract: ForgeQueryRuntimePublicApiFamilyContract,
) -> Result<ForgeServerQuerySupportPosture, ForgeServerOperationReadinessDenial> {
    if !matches!(
        prepared_kind,
        ForgeServerPreparedQueryHandoffKind::QueryRead
            | ForgeServerPreparedQueryHandoffKind::ForgeNativeSession
    ) {
        return Err(ForgeServerOperationReadinessDenial::new(
            ForgeServerOperationReadinessDenialCode::DownstreamDeliveryRequiresReadIntent,
            "downstream delivery handoff requires a read-admitted middleware intent or a forge-native direct session",
        ));
    }
    match requested_resume {
        ForgeServerQueryRequestedResume::None => Ok(
            ForgeServerQuerySupportPosture::DownstreamDeliverySupported {
                family_contract,
                runtime_resume_support_posture: contract.runtime_resume_support_posture(),
                durable_resume_support_posture: contract.durable_resume_support_posture(),
                contract_digest: contract.contract_for_reporting().to_string(),
            },
        ),
        ForgeServerQueryRequestedResume::RuntimeBacked { .. }
            if contract.runtime_backed_resume_supported() =>
        {
            Ok(
                ForgeServerQuerySupportPosture::RuntimeBackedResumeSupported {
                    family_contract,
                    runtime_resume_support_posture: contract.runtime_resume_support_posture(),
                    support_digest: contract.runtime_resume_support_for_reporting().to_string(),
                    contract_digest: contract.contract_for_reporting().to_string(),
                },
            )
        }
        ForgeServerQueryRequestedResume::RuntimeBacked { .. } => {
            Err(ForgeServerOperationReadinessDenial::new(
                ForgeServerOperationReadinessDenialCode::RuntimeBackedResumeUnsupported,
                format!(
                    "runtime-backed resume posture is {:?}",
                    contract.runtime_resume_support_posture()
                ),
            ))
        }
        ForgeServerQueryRequestedResume::Durable if contract.durable_resume_deferred() => {
            Err(ForgeServerOperationReadinessDenial::new(
                ForgeServerOperationReadinessDenialCode::DurableResumeDeferred,
                format!(
                    "durable resume remains deferred with digest {}",
                    contract.durable_resume_support_for_reporting()
                ),
            ))
        }
        ForgeServerQueryRequestedResume::Durable => {
            Ok(ForgeServerQuerySupportPosture::DurableResumeSupported {
                family_contract,
                durable_resume_support_posture: contract.durable_resume_support_posture(),
                support_digest: contract.durable_resume_support_for_reporting().to_string(),
                contract_digest: contract.contract_for_reporting().to_string(),
            })
        }
    }
}

fn admit_query_family(
    workspace: &ForgeQueryWorkspace,
    family: ForgeQueryRuntimeFacadeFamily,
) -> Result<ForgeQueryRuntimePublicApiFamilyContract, ForgeServerOperationReadinessDenial> {
    workspace.admit_public_api_family(family).map_err(|error| {
        ForgeServerOperationReadinessDenial::new(
            ForgeServerOperationReadinessDenialCode::UnsupportedQuerySupport,
            format!(
                "query workspace does not admit `{}` facade family: {error}",
                family.as_str()
            ),
        )
    })
}
