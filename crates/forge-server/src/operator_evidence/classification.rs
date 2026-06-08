use crate::{
    ForgeServerDenialCode, ForgeServerQueryHandoffDenialCode, ForgeServerRequestContextDenialCode,
    ForgeServerResponseEnvelope, ForgeServerSuccessKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperatorEvidenceClass {
    RequestContextDenied(ForgeServerRequestContextDenialCode),
    MiddlewareDenied(ForgeServerDenialCode),
    QueryHandoffDenied(ForgeServerQueryHandoffDenialCode),
    QueryReadSucceeded,
    QueryMutationSucceeded,
    DownstreamDeliverySucceeded,
}

impl ForgeServerOperatorEvidenceClass {
    pub(crate) fn from_response_envelope(response: &ForgeServerResponseEnvelope) -> Self {
        if let Some(success) = response.success() {
            return match success.payload().kind() {
                ForgeServerSuccessKind::QueryRead => Self::QueryReadSucceeded,
                ForgeServerSuccessKind::QueryMutation => Self::QueryMutationSucceeded,
                ForgeServerSuccessKind::DownstreamDelivery => Self::DownstreamDeliverySucceeded,
            };
        }

        let denial = response
            .denial()
            .expect("response envelope must contain a denial");
        if let Some(code) = denial.request_context_code() {
            Self::RequestContextDenied(code)
        } else if let Some(code) = denial.middleware_code() {
            Self::MiddlewareDenied(code)
        } else {
            Self::QueryHandoffDenied(
                denial
                    .query_handoff_code()
                    .expect("query handoff denial code must exist"),
            )
        }
    }

    pub(crate) fn contract_name(&self) -> &'static str {
        match self {
            Self::RequestContextDenied(_) => "server.operator_evidence.request_context_denied",
            Self::MiddlewareDenied(_) => "server.operator_evidence.middleware_denied",
            Self::QueryHandoffDenied(_) => "server.operator_evidence.query_handoff_denied",
            Self::QueryReadSucceeded => "server.operator_evidence.query_read_succeeded",
            Self::QueryMutationSucceeded => "server.operator_evidence.query_mutation_succeeded",
            Self::DownstreamDeliverySucceeded => {
                "server.operator_evidence.downstream_delivery_succeeded"
            }
        }
    }

    pub(crate) fn unsupported_capability(&self) -> bool {
        matches!(
            self,
            Self::QueryHandoffDenied(
                ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
                    | ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeUnsupported
                    | ForgeServerQueryHandoffDenialCode::DurableResumeDeferred
            )
        )
    }
}
