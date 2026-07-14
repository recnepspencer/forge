use worth_runtime_bridge::facade::{
    BridgeRouteRequest, BridgeSpeculativeSessionRequest,
    BridgeSubscriptionContinuationCandidateInput, BridgeTruthViewEvaluationRequest,
    BridgeWritebackDeclaration, BridgeWritebackEffectIntent, BridgeWritebackNativeCausalityInputs,
};

use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationEnvelopeEvidenceOrigin,
    WorthQueryDeclarationFutureProjection, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptDenialCause, WorthQueryDeclarationRoutePlanDenialCause,
    WorthQueryDomainEntryMarker,
};

use super::{
    contract::WorthQueryDeclarationBridgeContinuationFamily,
    explain::WorthQueryDeclarationBridgeRoutingExplanation,
    request::WorthQueryDeclarationBridgeContinuationRequest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationBridgeRoutingClass {
    ExclusiveBridgeContinuation,
    MixedAuthorityBridgeContinuation,
}

impl WorthQueryDeclarationBridgeRoutingClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExclusiveBridgeContinuation => "exclusive-bridge-continuation",
            Self::MixedAuthorityBridgeContinuation => "mixed-authority-bridge-continuation",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorthQueryDeclarationBridgeBinding {
    RuntimeRoute(BridgeRouteRequest),
    TruthView(BridgeTruthViewEvaluationRequest),
    PreviewSession(BridgeSpeculativeSessionRequest),
    PreviewPromotion(WorthQueryPreviewPromotionContinuationBinding),
    SubscriptionPreparation(BridgeSubscriptionContinuationCandidateInput),
    WritebackPreparation(WorthQueryWritebackPreparationBinding),
}

impl WorthQueryDeclarationBridgeBinding {
    pub fn surface(&self) -> &'static str {
        match self {
            Self::RuntimeRoute(_) => "worth_runtime_bridge::facade::BridgeRouteRequest",
            Self::TruthView(_) => "worth_runtime_bridge::facade::BridgeTruthViewEvaluationRequest",
            Self::PreviewSession(_) => {
                "worth_runtime_bridge::facade::BridgeSpeculativeSessionRequest"
            }
            Self::PreviewPromotion(_) => {
                "worth_query::application::WorthQueryPreviewPromotionContinuationBinding"
            }
            Self::SubscriptionPreparation(_) => {
                "worth_runtime_bridge::facade::BridgeSubscriptionContinuationCandidateInput"
            }
            Self::WritebackPreparation(_) => {
                "worth_query::application::WorthQueryWritebackPreparationBinding"
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthQueryWritebackPreparationBinding {
    declaration: BridgeWritebackDeclaration,
    causality: BridgeWritebackNativeCausalityInputs,
    effect_intent: BridgeWritebackEffectIntent,
}

impl WorthQueryWritebackPreparationBinding {
    pub(crate) fn new(
        declaration: BridgeWritebackDeclaration,
        causality: BridgeWritebackNativeCausalityInputs,
        effect_intent: BridgeWritebackEffectIntent,
    ) -> Self {
        Self {
            declaration,
            causality,
            effect_intent,
        }
    }

    pub fn declaration(&self) -> &BridgeWritebackDeclaration {
        &self.declaration
    }

    pub fn causality(&self) -> &BridgeWritebackNativeCausalityInputs {
        &self.causality
    }

    pub fn effect_intent(&self) -> &BridgeWritebackEffectIntent {
        &self.effect_intent
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthQueryPreviewPromotionContinuationBinding {
    preview_basis_digest: String,
    promotion_continuation_digest: String,
    declaration_digest: String,
}

impl WorthQueryPreviewPromotionContinuationBinding {
    pub(crate) fn new(
        preview_basis_digest: String,
        promotion_continuation_digest: String,
        declaration_digest: String,
    ) -> Self {
        Self {
            preview_basis_digest,
            promotion_continuation_digest,
            declaration_digest,
        }
    }

    pub fn preview_basis_digest(&self) -> &str {
        &self.preview_basis_digest
    }

    pub fn promotion_continuation_digest(&self) -> &str {
        &self.promotion_continuation_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
}

pub struct WorthQueryDeclarationBridgeRouting<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    class: WorthQueryDeclarationBridgeRoutingClass,
    continuation_request: WorthQueryDeclarationBridgeContinuationRequest,
    continuation_family: WorthQueryDeclarationBridgeContinuationFamily,
    binding: WorthQueryDeclarationBridgeBinding,
    aspect_contract: WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    aspect_fit: WorthQueryDeclarationAspectFit,
    mapped_aspects: WorthQueryDeclarationAspectCoverage,
    mapping_fit: WorthQueryDeclarationAspectFit,
    future_projection: WorthQueryDeclarationFutureProjection,
    basis_lifecycle_support_digest: String,
    envelope: WorthQueryDeclarationEnvelope<D, I>,
    bridge_routing_digest: String,
    explanation: WorthQueryDeclarationBridgeRoutingExplanation,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationBridgeRouting<D, I>
{
    pub(crate) fn new(
        class: WorthQueryDeclarationBridgeRoutingClass,
        continuation_request: WorthQueryDeclarationBridgeContinuationRequest,
        continuation_family: WorthQueryDeclarationBridgeContinuationFamily,
        binding: WorthQueryDeclarationBridgeBinding,
        aspect_contract: WorthQueryDeclarationAspectContract,
        aspect_coverage: WorthQueryDeclarationAspectCoverage,
        aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
        aspect_fit: WorthQueryDeclarationAspectFit,
        mapped_aspects: WorthQueryDeclarationAspectCoverage,
        mapping_fit: WorthQueryDeclarationAspectFit,
        future_projection: WorthQueryDeclarationFutureProjection,
        basis_lifecycle_support_digest: String,
        envelope: WorthQueryDeclarationEnvelope<D, I>,
        bridge_routing_digest: String,
        explanation: WorthQueryDeclarationBridgeRoutingExplanation,
    ) -> Self {
        Self {
            class,
            continuation_request,
            continuation_family,
            binding,
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
            aspect_fit,
            mapped_aspects,
            mapping_fit,
            future_projection,
            basis_lifecycle_support_digest,
            envelope,
            bridge_routing_digest,
            explanation,
        }
    }

    pub fn class(&self) -> WorthQueryDeclarationBridgeRoutingClass {
        self.class
    }

    pub fn continuation_request(&self) -> WorthQueryDeclarationBridgeContinuationRequest {
        self.continuation_request
    }

    pub fn truth_context(&self) -> super::request::WorthQueryDeclarationBridgeTruthContext {
        self.continuation_request.truth_context()
    }

    pub fn continuation_family(&self) -> WorthQueryDeclarationBridgeContinuationFamily {
        self.continuation_family
    }

    pub fn binding(&self) -> &WorthQueryDeclarationBridgeBinding {
        &self.binding
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn mapped_aspects(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.mapped_aspects
    }

    pub fn mapping_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.mapping_fit
    }

    pub fn future_projection(&self) -> &WorthQueryDeclarationFutureProjection {
        &self.future_projection
    }

    pub fn basis_lifecycle_support_digest(&self) -> &str {
        &self.basis_lifecycle_support_digest
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.envelope.declaration_family_key()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.envelope.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.envelope.operating_context_identity_digest()
    }

    pub fn declaration_digest(&self) -> &str {
        self.envelope.declaration_digest()
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.envelope.progression_digest()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.envelope.route_plan_digest()
    }

    pub fn receipt_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        self.envelope.receipt_digest()
    }

    pub fn envelope_digest(&self) -> &worth_foundational::facade::CanonicalDerivedDigest {
        self.envelope.envelope_digest()
    }

    pub fn bridge_routing_digest(&self) -> &str {
        &self.bridge_routing_digest
    }

    pub fn envelope(&self) -> &WorthQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn route_denial_cause(&self) -> Option<WorthQueryDeclarationRoutePlanDenialCause> {
        self.envelope.route_denial_cause()
    }

    pub fn receipt_denial_cause(&self) -> Option<WorthQueryDeclarationReceiptDenialCause> {
        self.envelope.receipt_denial_cause()
    }

    pub fn evidence_origin(&self) -> WorthQueryDeclarationEnvelopeEvidenceOrigin {
        self.envelope.evidence_origin()
    }

    pub fn explain(&self) -> &WorthQueryDeclarationBridgeRoutingExplanation {
        &self.explanation
    }

    pub(crate) fn into_envelope(self) -> WorthQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}
