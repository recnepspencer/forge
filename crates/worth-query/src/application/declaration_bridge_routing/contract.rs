use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationAuthorityAspectMismatch, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext,
};

use super::request::{
    WorthQueryDeclarationBridgeContinuationMode, WorthQueryDeclarationBridgeContinuationRequest,
    WorthQueryDeclarationBridgeTruthContext,
};

const WORKFLOW_AND_BRIDGE: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::WorkflowOrchestration,
    WorthQueryCapabilityFamily::PreviewSession,
];
const HISTORY_AND_BRIDGE: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::HistoricalEvaluation,
    WorthQueryCapabilityFamily::PreviewSession,
];
const LIVE_AND_BRIDGE: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::LiveQuery,
    WorthQueryCapabilityFamily::PreviewSession,
];
const RUNTIME_BRIDGE_ONLY: &[WorthQueryConfigSectionFamily] =
    &[WorthQueryConfigSectionFamily::RuntimeBridge];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationBridgeContinuationFamily {
    RuntimeRoute,
    TruthView,
    PreviewSession,
    PreviewPromotion,
    SubscriptionPreparation,
    WritebackPreparation,
    MixedBridgeContinuation,
}

impl WorthQueryDeclarationBridgeContinuationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeRoute => "runtime_route",
            Self::TruthView => "truth_view",
            Self::PreviewSession => "preview_session",
            Self::PreviewPromotion => "preview_promotion",
            Self::SubscriptionPreparation => "subscription_preparation",
            Self::WritebackPreparation => "writeback_preparation",
            Self::MixedBridgeContinuation => "mixed_bridge_continuation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationBridgeContinuationContract {
    family: WorthQueryDeclarationBridgeContinuationFamily,
    request: WorthQueryDeclarationBridgeContinuationRequest,
    required_capability_families: &'static [WorthQueryCapabilityFamily],
    required_config_sections: &'static [WorthQueryConfigSectionFamily],
    required_aspects: WorthQueryDeclarationAspectContract,
    reason: &'static str,
}

impl WorthQueryDeclarationBridgeContinuationContract {
    pub fn runtime_route_current() -> Self {
        Self {
            family: WorthQueryDeclarationBridgeContinuationFamily::RuntimeRoute,
            request: WorthQueryDeclarationBridgeContinuationRequest::new(
                WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute,
                WorthQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge runtime route request",
        }
    }

    pub fn truth_view_current() -> Self {
        Self {
            family: WorthQueryDeclarationBridgeContinuationFamily::TruthView,
            request: WorthQueryDeclarationBridgeContinuationRequest::new(
                WorthQueryDeclarationBridgeContinuationMode::TruthView,
                WorthQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge truth-view request over current truth",
        }
    }

    pub fn truth_view_historical() -> Self {
        Self {
            family: WorthQueryDeclarationBridgeContinuationFamily::TruthView,
            request: WorthQueryDeclarationBridgeContinuationRequest::new(
                WorthQueryDeclarationBridgeContinuationMode::TruthView,
                WorthQueryDeclarationBridgeTruthContext::Historical,
            ),
            required_capability_families: HISTORY_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge truth-view request over historical truth",
        }
    }

    pub fn preview_session() -> Self {
        Self {
            family: WorthQueryDeclarationBridgeContinuationFamily::PreviewSession,
            request: WorthQueryDeclarationBridgeContinuationRequest::new(
                WorthQueryDeclarationBridgeContinuationMode::PreviewSession,
                WorthQueryDeclarationBridgeTruthContext::Preview,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge preview-session request",
        }
    }

    pub fn preview_promotion() -> Self {
        Self {
            family: WorthQueryDeclarationBridgeContinuationFamily::PreviewPromotion,
            request: WorthQueryDeclarationBridgeContinuationRequest::new(
                WorthQueryDeclarationBridgeContinuationMode::PreviewPromotion,
                WorthQueryDeclarationBridgeTruthContext::Preview,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge preview-promotion request",
        }
    }

    pub fn subscription_preparation() -> Self {
        Self {
            family: WorthQueryDeclarationBridgeContinuationFamily::SubscriptionPreparation,
            request: WorthQueryDeclarationBridgeContinuationRequest::new(
                WorthQueryDeclarationBridgeContinuationMode::SubscriptionPreparation,
                WorthQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: LIVE_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason:
                "the declaration lowers into a bridge subscription-continuation preparation request",
        }
    }

    pub fn writeback_preparation() -> Self {
        Self {
            family: WorthQueryDeclarationBridgeContinuationFamily::WritebackPreparation,
            request: WorthQueryDeclarationBridgeContinuationRequest::new(
                WorthQueryDeclarationBridgeContinuationMode::WritebackPreparation,
                WorthQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: WorthQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge writeback-preparation request",
        }
    }

    pub fn family(&self) -> WorthQueryDeclarationBridgeContinuationFamily {
        self.family
    }

    pub fn request(&self) -> WorthQueryDeclarationBridgeContinuationRequest {
        self.request
    }

    pub fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        self.required_capability_families
    }

    pub fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        self.required_config_sections
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn required_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_aspects
    }

    pub fn with_required_aspects(
        mut self,
        required_aspects: WorthQueryDeclarationAspectContract,
    ) -> Self {
        self.required_aspects = required_aspects;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationBridgeRoutingSupportStatus {
    Admitted,
    Unsupported,
    InvalidContext,
}

impl WorthQueryDeclarationBridgeRoutingSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Unsupported => "unsupported",
            Self::InvalidContext => "invalid_context",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationBridgeRoutingSupportRow {
    continuation_mode: WorthQueryDeclarationBridgeContinuationMode,
    truth_context: WorthQueryDeclarationBridgeTruthContext,
    family: WorthQueryDeclarationBridgeContinuationFamily,
    required_aspect_slice: WorthQueryDeclarationAspectContract,
    available_aspect_slice: WorthQueryDeclarationAspectCoverage,
    aspect_fit: WorthQueryDeclarationAspectFit,
    aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
    mapped_aspect_slice: WorthQueryDeclarationAspectCoverage,
    mapping_fit: WorthQueryDeclarationAspectFit,
    status: WorthQueryDeclarationBridgeRoutingSupportStatus,
    reason: &'static str,
}

impl WorthQueryDeclarationBridgeRoutingSupportRow {
    pub(crate) fn new(
        continuation_mode: WorthQueryDeclarationBridgeContinuationMode,
        truth_context: WorthQueryDeclarationBridgeTruthContext,
        family: WorthQueryDeclarationBridgeContinuationFamily,
        required_aspect_slice: WorthQueryDeclarationAspectContract,
        available_aspect_slice: WorthQueryDeclarationAspectCoverage,
        aspect_fit: WorthQueryDeclarationAspectFit,
        aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
        mapped_aspect_slice: WorthQueryDeclarationAspectCoverage,
        mapping_fit: WorthQueryDeclarationAspectFit,
        status: WorthQueryDeclarationBridgeRoutingSupportStatus,
        reason: &'static str,
    ) -> Self {
        Self {
            continuation_mode,
            truth_context,
            family,
            required_aspect_slice,
            available_aspect_slice,
            aspect_fit,
            aspect_mismatch,
            mapped_aspect_slice,
            mapping_fit,
            status,
            reason,
        }
    }

    pub fn continuation_mode(&self) -> WorthQueryDeclarationBridgeContinuationMode {
        self.continuation_mode
    }

    pub fn truth_context(&self) -> WorthQueryDeclarationBridgeTruthContext {
        self.truth_context
    }

    pub fn family(&self) -> WorthQueryDeclarationBridgeContinuationFamily {
        self.family
    }

    pub fn required_aspect_slice(&self) -> &WorthQueryDeclarationAspectContract {
        &self.required_aspect_slice
    }

    pub fn available_aspect_slice(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.available_aspect_slice
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<WorthQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }

    pub fn mapped_aspect_slice(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.mapped_aspect_slice
    }

    pub fn mapping_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.mapping_fit
    }

    pub fn status(&self) -> WorthQueryDeclarationBridgeRoutingSupportStatus {
        self.status
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationBridgeRoutingSupportReport<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<WorthQueryDeclarationBridgeRoutingSupportRow>,
    support_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationBridgeRoutingSupportReport<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<WorthQueryDeclarationBridgeRoutingSupportRow>,
        support_digest: String,
    ) -> Self {
        Self {
            declaration_family_key,
            rows,
            support_digest,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn rows(&self) -> &[WorthQueryDeclarationBridgeRoutingSupportRow] {
        &self.rows
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

pub(crate) fn derive_bridge_routing_support_report<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
) -> WorthQueryDeclarationBridgeRoutingSupportReport<D, I> {
    crate::application::worth_query_bridge_routing_support_from_entry_readiness::<D, C, I>(handle)
}
