use forge_foundational::facade::DiagnosticRichnessProfile;

use crate::{config::ForgeServerResponseConfig, query_handoff::ForgeServerQueryHandoffOperation};

use super::{
    denial::{ForgeServerDenialCause, ForgeServerDenialEnvelope},
    envelope::ForgeServerResponseEnvelope,
    input::ForgeServerResponseInput,
    provenance::build_provenance,
    receipt::{build_denial_receipt, build_success_receipt},
    success::{ForgeServerSuccessEnvelope, ForgeServerSuccessKind, ForgeServerSuccessPayload},
    ForgeServerResponseTransform,
};

#[derive(Clone, Debug)]
pub struct ForgeServerResponsePlan {
    planned: PlannedResponse,
}

#[derive(Clone, Debug)]
enum PlannedResponse {
    Success {
        transform: ForgeServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        payload: ForgeServerSuccessPayload,
        boundary_label: String,
        canonical_digest: String,
    },
    Denial {
        transform: ForgeServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        cause: ForgeServerDenialCause,
        boundary_label: String,
        canonical_digest: String,
    },
}

impl ForgeServerResponsePlan {
    pub(crate) fn new(
        config: &ForgeServerResponseConfig,
        input: ForgeServerResponseInput,
        transform: Option<ForgeServerResponseTransform>,
    ) -> Self {
        let planned = match input {
            ForgeServerResponseInput::QueryHandoffSuccess(handoff) => {
                let diagnostics_profile = richest_profile(
                    handoff.admission().request_context().diagnostics_profile(),
                    config.success_minimum_diagnostics_profile(),
                );
                let transform = transform.unwrap_or(config.default_success_transform());
                let payload = ForgeServerSuccessPayload::new(
                    success_kind(handoff.operation()),
                    handoff.operation().clone(),
                    handoff.support_posture().clone(),
                    handoff.workspace().name().to_string(),
                );
                let canonical_digest = format!(
                    "response-success:{}:{}:{}",
                    payload.operation().canonical_label(),
                    payload.support_posture().canonical_label(),
                    handoff.canonical_digest()
                );
                let boundary_label = format!(
                    "server response success {} {}",
                    payload.operation().canonical_label(),
                    handoff.workspace().name()
                );
                PlannedResponse::Success {
                    transform,
                    diagnostics_profile,
                    payload,
                    boundary_label,
                    canonical_digest,
                }
            }
            ForgeServerResponseInput::RequestContextDenied(denial) => planned_denial(
                transform.unwrap_or(config.default_denial_transform()),
                config.denial_minimum_diagnostics_profile(),
                ForgeServerDenialCause::from_request_context(denial),
            ),
            ForgeServerResponseInput::MiddlewareDenied(denial) => planned_denial(
                transform.unwrap_or(config.default_denial_transform()),
                config.denial_minimum_diagnostics_profile(),
                ForgeServerDenialCause::from_middleware(denial),
            ),
            ForgeServerResponseInput::QueryHandoffDenied(denial) => planned_denial(
                transform.unwrap_or(config.default_denial_transform()),
                config.denial_minimum_diagnostics_profile(),
                ForgeServerDenialCause::from_query_handoff(denial),
            ),
        };
        Self { planned }
    }

    pub fn materialize(self) -> ForgeServerResponseEnvelope {
        match self.planned {
            PlannedResponse::Success {
                transform,
                diagnostics_profile,
                payload,
                boundary_label,
                canonical_digest,
            } => {
                let provenance = build_provenance("success", &canonical_digest);
                let receipt =
                    build_success_receipt(&boundary_label, &canonical_digest, provenance.clone());
                ForgeServerResponseEnvelope::from_success(ForgeServerSuccessEnvelope::new(
                    transform,
                    diagnostics_profile,
                    payload,
                    provenance,
                    receipt,
                    canonical_digest,
                ))
            }
            PlannedResponse::Denial {
                transform,
                diagnostics_profile,
                cause,
                boundary_label,
                canonical_digest,
            } => {
                let provenance = build_provenance("denial", &canonical_digest);
                let receipt =
                    build_denial_receipt(&boundary_label, &canonical_digest, provenance.clone());
                ForgeServerResponseEnvelope::from_denial(ForgeServerDenialEnvelope::new(
                    transform,
                    diagnostics_profile,
                    cause,
                    provenance,
                    receipt,
                    canonical_digest,
                ))
            }
        }
    }
}

fn planned_denial(
    transform: ForgeServerResponseTransform,
    minimum_diagnostics_profile: DiagnosticRichnessProfile,
    cause: ForgeServerDenialCause,
) -> PlannedResponse {
    let diagnostics_profile =
        richest_profile(cause.diagnostics_profile(), minimum_diagnostics_profile);
    let boundary_label = format!("server denial {:?} {}", cause.boundary(), cause.detail());
    let canonical_digest = match &cause {
        ForgeServerDenialCause::RequestContext { code, detail, .. } => {
            format!("response-denial:request-context:{code:?}:{detail}")
        }
        ForgeServerDenialCause::Middleware {
            code,
            priority,
            step,
            detail,
            ..
        } => format!("response-denial:middleware:{code:?}:{priority:?}:{step:?}:{detail}"),
        ForgeServerDenialCause::QueryHandoff { code, detail, .. } => {
            format!("response-denial:query-handoff:{code:?}:{detail}")
        }
    };
    PlannedResponse::Denial {
        transform,
        diagnostics_profile,
        cause,
        boundary_label,
        canonical_digest,
    }
}

fn richest_profile(
    requested: DiagnosticRichnessProfile,
    minimum: DiagnosticRichnessProfile,
) -> DiagnosticRichnessProfile {
    requested.max(minimum)
}

fn success_kind(operation: &ForgeServerQueryHandoffOperation) -> ForgeServerSuccessKind {
    match operation {
        ForgeServerQueryHandoffOperation::QueryRead { .. } => ForgeServerSuccessKind::QueryRead,
        ForgeServerQueryHandoffOperation::QueryMutation { .. } => {
            ForgeServerSuccessKind::QueryMutation
        }
        ForgeServerQueryHandoffOperation::DownstreamDelivery { .. } => {
            ForgeServerSuccessKind::DownstreamDelivery
        }
    }
}
