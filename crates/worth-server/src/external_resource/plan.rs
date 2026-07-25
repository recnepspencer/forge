use worth_query::facade::foundation::{
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture, WorthQueryAsyncSourceFamily,
};

use super::{WorthServerExternalResourceBudget, WorthServerExternalResourceIntent};

#[derive(Clone, Debug)]
pub struct WorthServerLoweredExternalResourcePlan {
    intent: WorthServerExternalResourceIntent,
    canonical_digest: String,
}

impl WorthServerLoweredExternalResourcePlan {
    pub(crate) fn lower(
        intent: WorthServerExternalResourceIntent,
    ) -> Result<Self, WorthServerExternalResourcePlanDenial> {
        let request_identity = intent.request_identity();
        if !matches!(
            request_identity.source_family(),
            WorthQueryAsyncSourceFamily::ExternalResource
                | WorthQueryAsyncSourceFamily::HostResource
        ) {
            return Err(WorthServerExternalResourcePlanDenial::new(
                WorthServerExternalResourcePlanDenialCode::UnsupportedSourceFamily,
                "one-shot server resource execution admits only host or external resources",
            ));
        }
        if request_identity.loading_posture() != WorthQueryAsyncLoadingPosture::Blocking
            || request_identity.failure_posture() != WorthQueryAsyncFailurePosture::FailClosed
        {
            return Err(WorthServerExternalResourcePlanDenial::new(
                WorthServerExternalResourcePlanDenialCode::UnsupportedLifecyclePosture,
                "one-shot server resource execution requires blocking fail-closed posture",
            ));
        }
        if intent.request_body().len() > intent.budget().max_request_bytes() {
            return Err(WorthServerExternalResourcePlanDenial::new(
                WorthServerExternalResourcePlanDenialCode::RequestBudgetExceeded,
                "external resource request exceeded its admitted byte budget",
            ));
        }
        let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-external-resource-plan-v1",
        )
        .field("request_identity", request_identity.canonical_identity())
        .field("provider_identity", intent.provider_identity())
        .field("contract_identity", intent.contract_identity())
        .field("basis_identity", intent.basis_identity())
        .field_bytes("request_body", intent.request_body())
        .field(
            "max_request_bytes",
            &intent.budget().max_request_bytes().to_string(),
        )
        .field(
            "max_response_bytes",
            &intent.budget().max_response_bytes().to_string(),
        )
        .field(
            "deadline_millis",
            &intent.budget().deadline_millis().to_string(),
        )
        .finish();
        Ok(Self {
            intent,
            canonical_digest,
        })
    }

    pub fn request_identity(&self) -> &str {
        self.intent.request_identity().canonical_identity()
    }

    pub fn provider_identity(&self) -> &str {
        self.intent.provider_identity()
    }

    pub fn contract_identity(&self) -> &str {
        self.intent.contract_identity()
    }

    pub fn basis_identity(&self) -> &str {
        self.intent.basis_identity()
    }

    pub fn request_body(&self) -> &[u8] {
        self.intent.request_body()
    }

    pub fn budget(&self) -> WorthServerExternalResourceBudget {
        self.intent.budget()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerExternalResourcePlanDenialCode {
    UnsupportedSourceFamily,
    UnsupportedLifecyclePosture,
    RequestBudgetExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalResourcePlanDenial {
    code: WorthServerExternalResourcePlanDenialCode,
    detail: String,
}

impl WorthServerExternalResourcePlanDenial {
    fn new(code: WorthServerExternalResourcePlanDenialCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> WorthServerExternalResourcePlanDenialCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
