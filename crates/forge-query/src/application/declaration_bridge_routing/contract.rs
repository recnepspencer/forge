use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationAuthorityAspectMismatch, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

use super::request::{
    ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeTruthContext,
};

const WORKFLOW_AND_BRIDGE: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::WorkflowOrchestration,
    ForgeQueryCapabilityFamily::PreviewSession,
];
const HISTORY_AND_BRIDGE: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::HistoricalEvaluation,
    ForgeQueryCapabilityFamily::PreviewSession,
];
const LIVE_AND_BRIDGE: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::LiveQuery,
    ForgeQueryCapabilityFamily::PreviewSession,
];
const RUNTIME_BRIDGE_ONLY: &[ForgeQueryConfigSectionFamily] =
    &[ForgeQueryConfigSectionFamily::RuntimeBridge];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationBridgeContinuationFamily {
    RuntimeRoute,
    TruthView,
    PreviewSession,
    PreviewPromotion,
    SubscriptionPreparation,
    WritebackPreparation,
    MixedBridgeContinuation,
}

impl ForgeQueryDeclarationBridgeContinuationFamily {
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
pub struct ForgeQueryDeclarationBridgeContinuationContract {
    family: ForgeQueryDeclarationBridgeContinuationFamily,
    request: ForgeQueryDeclarationBridgeContinuationRequest,
    required_capability_families: &'static [ForgeQueryCapabilityFamily],
    required_config_sections: &'static [ForgeQueryConfigSectionFamily],
    required_aspects: ForgeQueryDeclarationAspectContract,
    reason: &'static str,
}

impl ForgeQueryDeclarationBridgeContinuationContract {
    pub fn runtime_route_current() -> Self {
        Self {
            family: ForgeQueryDeclarationBridgeContinuationFamily::RuntimeRoute,
            request: ForgeQueryDeclarationBridgeContinuationRequest::new(
                ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
                ForgeQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: ForgeQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge runtime route request",
        }
    }

    pub fn truth_view_current() -> Self {
        Self {
            family: ForgeQueryDeclarationBridgeContinuationFamily::TruthView,
            request: ForgeQueryDeclarationBridgeContinuationRequest::new(
                ForgeQueryDeclarationBridgeContinuationMode::TruthView,
                ForgeQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: ForgeQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge truth-view request over current truth",
        }
    }

    pub fn truth_view_historical() -> Self {
        Self {
            family: ForgeQueryDeclarationBridgeContinuationFamily::TruthView,
            request: ForgeQueryDeclarationBridgeContinuationRequest::new(
                ForgeQueryDeclarationBridgeContinuationMode::TruthView,
                ForgeQueryDeclarationBridgeTruthContext::Historical,
            ),
            required_capability_families: HISTORY_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: ForgeQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge truth-view request over historical truth",
        }
    }

    pub fn preview_session() -> Self {
        Self {
            family: ForgeQueryDeclarationBridgeContinuationFamily::PreviewSession,
            request: ForgeQueryDeclarationBridgeContinuationRequest::new(
                ForgeQueryDeclarationBridgeContinuationMode::PreviewSession,
                ForgeQueryDeclarationBridgeTruthContext::Preview,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: ForgeQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge preview-session request",
        }
    }

    pub fn preview_promotion() -> Self {
        Self {
            family: ForgeQueryDeclarationBridgeContinuationFamily::PreviewPromotion,
            request: ForgeQueryDeclarationBridgeContinuationRequest::new(
                ForgeQueryDeclarationBridgeContinuationMode::PreviewPromotion,
                ForgeQueryDeclarationBridgeTruthContext::Preview,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: ForgeQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge preview-promotion request",
        }
    }

    pub fn subscription_preparation() -> Self {
        Self {
            family: ForgeQueryDeclarationBridgeContinuationFamily::SubscriptionPreparation,
            request: ForgeQueryDeclarationBridgeContinuationRequest::new(
                ForgeQueryDeclarationBridgeContinuationMode::SubscriptionPreparation,
                ForgeQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: LIVE_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: ForgeQueryDeclarationAspectContract::empty(),
            reason:
                "the declaration lowers into a bridge subscription-continuation preparation request",
        }
    }

    pub fn writeback_preparation() -> Self {
        Self {
            family: ForgeQueryDeclarationBridgeContinuationFamily::WritebackPreparation,
            request: ForgeQueryDeclarationBridgeContinuationRequest::new(
                ForgeQueryDeclarationBridgeContinuationMode::WritebackPreparation,
                ForgeQueryDeclarationBridgeTruthContext::Current,
            ),
            required_capability_families: WORKFLOW_AND_BRIDGE,
            required_config_sections: RUNTIME_BRIDGE_ONLY,
            required_aspects: ForgeQueryDeclarationAspectContract::empty(),
            reason: "the declaration lowers into a bridge writeback-preparation request",
        }
    }

    pub fn family(&self) -> ForgeQueryDeclarationBridgeContinuationFamily {
        self.family
    }

    pub fn request(&self) -> ForgeQueryDeclarationBridgeContinuationRequest {
        self.request
    }

    pub fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        self.required_capability_families
    }

    pub fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        self.required_config_sections
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn required_aspects(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.required_aspects
    }

    pub fn with_required_aspects(
        mut self,
        required_aspects: ForgeQueryDeclarationAspectContract,
    ) -> Self {
        self.required_aspects = required_aspects;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationBridgeRoutingSupportStatus {
    Admitted,
    Unsupported,
    InvalidContext,
}

impl ForgeQueryDeclarationBridgeRoutingSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Unsupported => "unsupported",
            Self::InvalidContext => "invalid_context",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationBridgeRoutingSupportRow {
    continuation_mode: ForgeQueryDeclarationBridgeContinuationMode,
    truth_context: ForgeQueryDeclarationBridgeTruthContext,
    family: ForgeQueryDeclarationBridgeContinuationFamily,
    required_aspect_slice: ForgeQueryDeclarationAspectContract,
    available_aspect_slice: ForgeQueryDeclarationAspectCoverage,
    aspect_fit: ForgeQueryDeclarationAspectFit,
    aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
    mapped_aspect_slice: ForgeQueryDeclarationAspectCoverage,
    mapping_fit: ForgeQueryDeclarationAspectFit,
    status: ForgeQueryDeclarationBridgeRoutingSupportStatus,
    reason: &'static str,
}

impl ForgeQueryDeclarationBridgeRoutingSupportRow {
    pub(crate) fn new(
        continuation_mode: ForgeQueryDeclarationBridgeContinuationMode,
        truth_context: ForgeQueryDeclarationBridgeTruthContext,
        family: ForgeQueryDeclarationBridgeContinuationFamily,
        required_aspect_slice: ForgeQueryDeclarationAspectContract,
        available_aspect_slice: ForgeQueryDeclarationAspectCoverage,
        aspect_fit: ForgeQueryDeclarationAspectFit,
        aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
        mapped_aspect_slice: ForgeQueryDeclarationAspectCoverage,
        mapping_fit: ForgeQueryDeclarationAspectFit,
        status: ForgeQueryDeclarationBridgeRoutingSupportStatus,
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

    pub fn continuation_mode(&self) -> ForgeQueryDeclarationBridgeContinuationMode {
        self.continuation_mode
    }

    pub fn truth_context(&self) -> ForgeQueryDeclarationBridgeTruthContext {
        self.truth_context
    }

    pub fn family(&self) -> ForgeQueryDeclarationBridgeContinuationFamily {
        self.family
    }

    pub fn required_aspect_slice(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.required_aspect_slice
    }

    pub fn available_aspect_slice(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.available_aspect_slice
    }

    pub fn aspect_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<ForgeQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }

    pub fn mapped_aspect_slice(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.mapped_aspect_slice
    }

    pub fn mapping_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.mapping_fit
    }

    pub fn status(&self) -> ForgeQueryDeclarationBridgeRoutingSupportStatus {
        self.status
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationBridgeRoutingSupportReport<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<ForgeQueryDeclarationBridgeRoutingSupportRow>,
    support_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationBridgeRoutingSupportReport<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<ForgeQueryDeclarationBridgeRoutingSupportRow>,
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

    pub fn rows(&self) -> &[ForgeQueryDeclarationBridgeRoutingSupportRow] {
        &self.rows
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

pub(crate) fn derive_bridge_routing_support_report<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationBridgeRoutingSupportReport<D, I> {
    crate::application::forge_query_bridge_routing_support_from_entry_readiness::<D, C, I>(handle)
}
