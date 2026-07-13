use worth_foundational::DiagnosticRichnessProfile;
use worth_query::facade::runtime::{
    WorthQueryRuntimeDownstreamDeliveryContract, WorthQueryRuntimeFacadeFamily,
    WorthQueryRuntimePublicApiFamilyContract, WorthQueryWorkspace,
};

use crate::{
    WorthServerPreparedQueryHandoffKind, WorthServerQueryHandoffOperation,
    WorthServerQueryRequestedResume, WorthServerQuerySupportPosture,
};

use super::{WorthServerOperationReadinessDenial, WorthServerOperationReadinessDenialCode};

pub(crate) fn derive_query_support_posture(
    prepared_kind: WorthServerPreparedQueryHandoffKind,
    operation: &WorthServerQueryHandoffOperation,
    workspace: &WorthQueryWorkspace,
    contract: &WorthQueryRuntimeDownstreamDeliveryContract,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> Result<WorthServerQuerySupportPosture, WorthServerOperationReadinessDenial> {
    if prepared_kind == WorthServerPreparedQueryHandoffKind::WorthNativeSession {
        match operation {
            WorthServerQueryHandoffOperation::QueryRead { .. }
            | WorthServerQueryHandoffOperation::QueryMutation { .. } => {
                return Err(WorthServerOperationReadinessDenial::new(
                    WorthServerOperationReadinessDenialCode::UnsupportedQuerySupport,
                    format!(
                        "WORTH-native session entry does not support `{}` query handoff posture under {:?} diagnostics",
                        operation.canonical_label(),
                        diagnostics_profile,
                    ),
                ));
            }
            _ => {}
        }
    }

    match operation {
        WorthServerQueryHandoffOperation::QueryRead { .. } => {
            Ok(WorthServerQuerySupportPosture::QueryReadSupported {
                family_contract: admit_query_family(
                    workspace,
                    WorthQueryRuntimeFacadeFamily::Read,
                )?,
            })
        }
        WorthServerQueryHandoffOperation::DirectRead { .. } => {
            Ok(WorthServerQuerySupportPosture::DirectReadSupported {
                family_contract: admit_query_family(
                    workspace,
                    WorthQueryRuntimeFacadeFamily::Read,
                )?,
            })
        }
        WorthServerQueryHandoffOperation::DirectState { .. } => {
            Ok(WorthServerQuerySupportPosture::DirectStateSupported {
                family_contract: admit_query_family(
                    workspace,
                    WorthQueryRuntimeFacadeFamily::Live,
                )?,
            })
        }
        WorthServerQueryHandoffOperation::DirectInspection { .. } => {
            Ok(WorthServerQuerySupportPosture::DirectInspectionSupported {
                family_contract: admit_query_family(
                    workspace,
                    WorthQueryRuntimeFacadeFamily::Inspect,
                )?,
            })
        }
        WorthServerQueryHandoffOperation::DirectProjection { .. } => {
            Ok(WorthServerQuerySupportPosture::DirectProjectionSupported {
                family_contract: admit_query_family(
                    workspace,
                    WorthQueryRuntimeFacadeFamily::Read,
                )?,
            })
        }
        WorthServerQueryHandoffOperation::DirectMutation { .. } => {
            Ok(WorthServerQuerySupportPosture::DirectMutationSupported {
                family_contract: admit_query_family(
                    workspace,
                    WorthQueryRuntimeFacadeFamily::Write,
                )?,
            })
        }
        WorthServerQueryHandoffOperation::QueryMutation { .. } => {
            Ok(WorthServerQuerySupportPosture::QueryMutationSupported {
                family_contract: admit_query_family(
                    workspace,
                    WorthQueryRuntimeFacadeFamily::Write,
                )?,
            })
        }
        WorthServerQueryHandoffOperation::DownstreamDelivery {
            requested_resume, ..
        } => {
            let family_contract =
                admit_query_family(workspace, WorthQueryRuntimeFacadeFamily::Live)?;
            derive_delivery_support(prepared_kind, requested_resume, contract, family_contract)
        }
    }
}

fn derive_delivery_support(
    prepared_kind: WorthServerPreparedQueryHandoffKind,
    requested_resume: &WorthServerQueryRequestedResume,
    contract: &WorthQueryRuntimeDownstreamDeliveryContract,
    family_contract: WorthQueryRuntimePublicApiFamilyContract,
) -> Result<WorthServerQuerySupportPosture, WorthServerOperationReadinessDenial> {
    if !matches!(
        prepared_kind,
        WorthServerPreparedQueryHandoffKind::QueryRead
            | WorthServerPreparedQueryHandoffKind::WorthNativeSession
    ) {
        return Err(WorthServerOperationReadinessDenial::new(
            WorthServerOperationReadinessDenialCode::DownstreamDeliveryRequiresReadIntent,
            "downstream delivery handoff requires a read-admitted middleware intent or a WORTH-native direct session",
        ));
    }
    match requested_resume {
        WorthServerQueryRequestedResume::None => Ok(
            WorthServerQuerySupportPosture::DownstreamDeliverySupported {
                family_contract,
                runtime_resume_support_posture: contract.runtime_resume_support_posture(),
                durable_resume_support_posture: contract.durable_resume_support_posture(),
                contract_digest: contract.contract_for_reporting().to_string(),
            },
        ),
        WorthServerQueryRequestedResume::RuntimeBacked { .. }
            if contract.runtime_backed_resume_supported() =>
        {
            Ok(
                WorthServerQuerySupportPosture::RuntimeBackedResumeSupported {
                    family_contract,
                    runtime_resume_support_posture: contract.runtime_resume_support_posture(),
                    support_digest: contract.runtime_resume_support_for_reporting().to_string(),
                    contract_digest: contract.contract_for_reporting().to_string(),
                },
            )
        }
        WorthServerQueryRequestedResume::RuntimeBacked { .. } => {
            Err(WorthServerOperationReadinessDenial::new(
                WorthServerOperationReadinessDenialCode::RuntimeBackedResumeUnsupported,
                format!(
                    "runtime-backed resume posture is {:?}",
                    contract.runtime_resume_support_posture()
                ),
            ))
        }
        WorthServerQueryRequestedResume::Durable if contract.durable_resume_deferred() => {
            Err(WorthServerOperationReadinessDenial::new(
                WorthServerOperationReadinessDenialCode::DurableResumeDeferred,
                format!(
                    "durable resume remains deferred with digest {}",
                    contract.durable_resume_support_for_reporting()
                ),
            ))
        }
        WorthServerQueryRequestedResume::Durable => {
            Ok(WorthServerQuerySupportPosture::DurableResumeSupported {
                family_contract,
                durable_resume_support_posture: contract.durable_resume_support_posture(),
                support_digest: contract.durable_resume_support_for_reporting().to_string(),
                contract_digest: contract.contract_for_reporting().to_string(),
            })
        }
    }
}

fn admit_query_family(
    workspace: &WorthQueryWorkspace,
    family: WorthQueryRuntimeFacadeFamily,
) -> Result<WorthQueryRuntimePublicApiFamilyContract, WorthServerOperationReadinessDenial> {
    workspace.admit_public_api_family(family).map_err(|error| {
        WorthServerOperationReadinessDenial::new(
            WorthServerOperationReadinessDenialCode::UnsupportedQuerySupport,
            format!(
                "query workspace does not admit `{}` facade family: {error}",
                family.as_str()
            ),
        )
    })
}
