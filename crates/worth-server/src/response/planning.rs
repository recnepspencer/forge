use worth_foundational::facade::DiagnosticRichnessProfile;

use crate::{config::WorthServerResponseConfig, query_handoff::WorthServerQueryHandoffOperation};

use super::{
    denial::{WorthServerDenialCause, WorthServerDenialEnvelope},
    envelope::WorthServerResponseEnvelope,
    input::WorthServerResponseInput,
    provenance::build_provenance,
    receipt::{build_denial_receipt, build_success_receipt},
    success::{WorthServerSuccessEnvelope, WorthServerSuccessKind, WorthServerSuccessPayload},
    WorthServerResponseTransform,
};

#[derive(Clone, Debug)]
pub struct WorthServerResponsePlan {
    planned: PlannedResponse,
}

#[derive(Clone, Debug)]
enum PlannedResponse {
    Success {
        transform: WorthServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        payload: Box<WorthServerSuccessPayload>,
        boundary_label: String,
        canonical_digest: String,
    },
    Denial {
        transform: WorthServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        cause: Box<WorthServerDenialCause>,
        boundary_label: String,
        canonical_digest: String,
    },
}

impl WorthServerResponsePlan {
    pub(crate) fn new(
        config: &WorthServerResponseConfig,
        input: WorthServerResponseInput,
        transform: Option<WorthServerResponseTransform>,
    ) -> Self {
        let planned = match input {
            WorthServerResponseInput::QueryHandoffSuccess(handoff) => {
                let diagnostics_profile = richest_profile(
                    handoff.admission().request_context().diagnostics_profile(),
                    config.success_minimum_diagnostics_profile(),
                );
                let transform = transform.unwrap_or(config.default_success_transform());
                let payload = WorthServerSuccessPayload::new(
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
                    payload: Box::new(payload),
                    boundary_label,
                    canonical_digest,
                }
            }
            WorthServerResponseInput::RequestContextDenied(denial) => planned_denial(
                transform.unwrap_or(config.default_denial_transform()),
                config.denial_minimum_diagnostics_profile(),
                WorthServerDenialCause::from_request_context(*denial),
            ),
            WorthServerResponseInput::MiddlewareDenied(denial) => planned_denial(
                transform.unwrap_or(config.default_denial_transform()),
                config.denial_minimum_diagnostics_profile(),
                WorthServerDenialCause::from_middleware(*denial),
            ),
            WorthServerResponseInput::QueryHandoffDenied(denial) => planned_denial(
                transform.unwrap_or(config.default_denial_transform()),
                config.denial_minimum_diagnostics_profile(),
                WorthServerDenialCause::from_query_handoff(*denial),
            ),
        };
        Self { planned }
    }

    pub fn materialize(self) -> WorthServerResponseEnvelope {
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
                WorthServerResponseEnvelope::from_success(WorthServerSuccessEnvelope::new(
                    transform,
                    diagnostics_profile,
                    *payload,
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
                WorthServerResponseEnvelope::from_denial(WorthServerDenialEnvelope::new(
                    transform,
                    diagnostics_profile,
                    *cause,
                    provenance,
                    receipt,
                    canonical_digest,
                ))
            }
        }
    }
}

fn planned_denial(
    transform: WorthServerResponseTransform,
    minimum_diagnostics_profile: DiagnosticRichnessProfile,
    cause: WorthServerDenialCause,
) -> PlannedResponse {
    let diagnostics_profile =
        richest_profile(cause.diagnostics_profile(), minimum_diagnostics_profile);
    let boundary_label = format!("server denial {:?} {}", cause.boundary(), cause.detail());
    let canonical_digest = match &cause {
        WorthServerDenialCause::RequestContext { code, detail, .. } => {
            format!("response-denial:request-context:{code:?}:{detail}")
        }
        WorthServerDenialCause::Middleware {
            code,
            priority,
            step,
            detail,
            ..
        } => format!("response-denial:middleware:{code:?}:{priority:?}:{step:?}:{detail}"),
        WorthServerDenialCause::QueryHandoff { code, detail, .. } => {
            format!("response-denial:query-handoff:{code:?}:{detail}")
        }
    };
    PlannedResponse::Denial {
        transform,
        diagnostics_profile,
        cause: Box::new(cause),
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

fn success_kind(operation: &WorthServerQueryHandoffOperation) -> WorthServerSuccessKind {
    match operation {
        WorthServerQueryHandoffOperation::QueryRead { .. } => WorthServerSuccessKind::QueryRead,
        WorthServerQueryHandoffOperation::DirectRead { .. } => WorthServerSuccessKind::DirectRead,
        WorthServerQueryHandoffOperation::DirectState { .. } => WorthServerSuccessKind::DirectState,
        WorthServerQueryHandoffOperation::DirectInspection { .. } => {
            WorthServerSuccessKind::DirectInspection
        }
        WorthServerQueryHandoffOperation::DirectProjection { .. } => {
            WorthServerSuccessKind::DirectProjection
        }
        WorthServerQueryHandoffOperation::DirectMutation { .. } => {
            WorthServerSuccessKind::DirectMutation
        }
        WorthServerQueryHandoffOperation::QueryMutation { .. } => {
            WorthServerSuccessKind::QueryMutation
        }
        WorthServerQueryHandoffOperation::DownstreamDelivery { .. } => {
            WorthServerSuccessKind::DownstreamDelivery
        }
    }
}
