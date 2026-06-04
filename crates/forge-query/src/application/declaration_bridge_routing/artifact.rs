use forge_runtime_bridge::facade::{
    BridgeRouteRequest, BridgeSpeculativeSessionRequest,
    BridgeSubscriptionContinuationCandidateInput, BridgeTruthViewEvaluationRequest,
    BridgeWritebackDeclaration, BridgeWritebackEffectIntent, BridgeWritebackNativeCausalityInputs,
};

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
};

use super::{
    contract::ForgeQueryDeclarationBridgeContinuationFamily,
    explain::ForgeQueryDeclarationBridgeRoutingExplanation,
    request::ForgeQueryDeclarationBridgeContinuationRequest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationBridgeRoutingClass {
    ExclusiveBridgeContinuation,
    MixedAuthorityBridgeContinuation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ForgeQueryDeclarationBridgeBinding {
    RuntimeRoute(BridgeRouteRequest),
    TruthView(BridgeTruthViewEvaluationRequest),
    PreviewSession(BridgeSpeculativeSessionRequest),
    PreviewPromotion(ForgeQueryPreviewPromotionContinuationBinding),
    SubscriptionPreparation(BridgeSubscriptionContinuationCandidateInput),
    WritebackPreparation(ForgeQueryWritebackPreparationBinding),
}

impl ForgeQueryDeclarationBridgeBinding {
    pub fn surface(&self) -> &'static str {
        match self {
            Self::RuntimeRoute(_) => "forge_runtime_bridge::facade::BridgeRouteRequest",
            Self::TruthView(_) => "forge_runtime_bridge::facade::BridgeTruthViewEvaluationRequest",
            Self::PreviewSession(_) => {
                "forge_runtime_bridge::facade::BridgeSpeculativeSessionRequest"
            }
            Self::PreviewPromotion(_) => {
                "forge_query::application::ForgeQueryPreviewPromotionContinuationBinding"
            }
            Self::SubscriptionPreparation(_) => {
                "forge_runtime_bridge::facade::BridgeSubscriptionContinuationCandidateInput"
            }
            Self::WritebackPreparation(_) => {
                "forge_query::application::ForgeQueryWritebackPreparationBinding"
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForgeQueryWritebackPreparationBinding {
    declaration: BridgeWritebackDeclaration,
    causality: BridgeWritebackNativeCausalityInputs,
    effect_intent: BridgeWritebackEffectIntent,
}

impl ForgeQueryWritebackPreparationBinding {
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
pub struct ForgeQueryPreviewPromotionContinuationBinding {
    preview_basis_digest: String,
    promotion_continuation_digest: String,
    declaration_digest: String,
}

impl ForgeQueryPreviewPromotionContinuationBinding {
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

pub struct ForgeQueryDeclarationBridgeRouting<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    class: ForgeQueryDeclarationBridgeRoutingClass,
    continuation_request: ForgeQueryDeclarationBridgeContinuationRequest,
    continuation_family: ForgeQueryDeclarationBridgeContinuationFamily,
    binding: ForgeQueryDeclarationBridgeBinding,
    aspect_contract: ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    aspect_fit: ForgeQueryDeclarationAspectFit,
    mapped_aspects: ForgeQueryDeclarationAspectCoverage,
    mapping_fit: ForgeQueryDeclarationAspectFit,
    envelope: ForgeQueryDeclarationEnvelope<D, I>,
    bridge_routing_digest: String,
    explanation: ForgeQueryDeclarationBridgeRoutingExplanation,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationBridgeRouting<D, I>
{
    pub(crate) fn new(
        class: ForgeQueryDeclarationBridgeRoutingClass,
        continuation_request: ForgeQueryDeclarationBridgeContinuationRequest,
        continuation_family: ForgeQueryDeclarationBridgeContinuationFamily,
        binding: ForgeQueryDeclarationBridgeBinding,
        aspect_contract: ForgeQueryDeclarationAspectContract,
        aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
        aspect_fit: ForgeQueryDeclarationAspectFit,
        mapped_aspects: ForgeQueryDeclarationAspectCoverage,
        mapping_fit: ForgeQueryDeclarationAspectFit,
        envelope: ForgeQueryDeclarationEnvelope<D, I>,
        bridge_routing_digest: String,
        explanation: ForgeQueryDeclarationBridgeRoutingExplanation,
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
            envelope,
            bridge_routing_digest,
            explanation,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationBridgeRoutingClass {
        self.class
    }

    pub fn continuation_request(&self) -> ForgeQueryDeclarationBridgeContinuationRequest {
        self.continuation_request
    }

    pub fn truth_context(&self) -> super::request::ForgeQueryDeclarationBridgeTruthContext {
        self.continuation_request.truth_context()
    }

    pub fn continuation_family(&self) -> ForgeQueryDeclarationBridgeContinuationFamily {
        self.continuation_family
    }

    pub fn binding(&self) -> &ForgeQueryDeclarationBridgeBinding {
        &self.binding
    }

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn mapped_aspects(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.mapped_aspects
    }

    pub fn mapping_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.mapping_fit
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

    pub fn receipt_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        self.envelope.receipt_digest()
    }

    pub fn envelope_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        self.envelope.envelope_digest()
    }

    pub fn bridge_routing_digest(&self) -> &str {
        &self.bridge_routing_digest
    }

    pub fn envelope(&self) -> &ForgeQueryDeclarationEnvelope<D, I> {
        &self.envelope
    }

    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.envelope.route_denial_cause()
    }

    pub fn receipt_denial_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.envelope.receipt_denial_cause()
    }

    pub fn evidence_origin(&self) -> ForgeQueryDeclarationEnvelopeEvidenceOrigin {
        self.envelope.evidence_origin()
    }

    pub fn explain(&self) -> &ForgeQueryDeclarationBridgeRoutingExplanation {
        &self.explanation
    }

    pub(crate) fn into_envelope(self) -> ForgeQueryDeclarationEnvelope<D, I> {
        self.envelope
    }
}
