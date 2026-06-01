use crate::application::{
    ForgeQueryDeclarationEnvelopeEvidenceOrigin, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause,
};

use super::{
    contract::ForgeQueryDeclarationBridgeContinuationFamily,
    request::{
        ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeTruthContext,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationBridgeRoutingExplanation {
    crossing_posture: &'static str,
    continuation_mode: ForgeQueryDeclarationBridgeContinuationMode,
    truth_context: ForgeQueryDeclarationBridgeTruthContext,
    continuation_family: ForgeQueryDeclarationBridgeContinuationFamily,
    binding_surface: &'static str,
    retained_truths: Vec<String>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    evidence_origin: ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    mixed_origin: bool,
}

impl ForgeQueryDeclarationBridgeRoutingExplanation {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        crossing_posture: &'static str,
        continuation_mode: ForgeQueryDeclarationBridgeContinuationMode,
        truth_context: ForgeQueryDeclarationBridgeTruthContext,
        continuation_family: ForgeQueryDeclarationBridgeContinuationFamily,
        binding_surface: &'static str,
        retained_truths: Vec<String>,
        route_governing_reason: Option<String>,
        route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
        receipt_governing_reason: String,
        receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
        evidence_origin: ForgeQueryDeclarationEnvelopeEvidenceOrigin,
        mixed_origin: bool,
    ) -> Self {
        Self {
            crossing_posture,
            continuation_mode,
            truth_context,
            continuation_family,
            binding_surface,
            retained_truths,
            route_governing_reason,
            route_denial_cause,
            receipt_governing_reason,
            receipt_denial_cause,
            evidence_origin,
            mixed_origin,
        }
    }

    pub fn crossing_posture(&self) -> &'static str {
        self.crossing_posture
    }

    pub fn continuation_mode(&self) -> ForgeQueryDeclarationBridgeContinuationMode {
        self.continuation_mode
    }

    pub fn truth_context(&self) -> ForgeQueryDeclarationBridgeTruthContext {
        self.truth_context
    }

    pub fn continuation_family(&self) -> ForgeQueryDeclarationBridgeContinuationFamily {
        self.continuation_family
    }

    pub fn binding_surface(&self) -> &'static str {
        self.binding_surface
    }

    pub fn retained_truths(&self) -> &[String] {
        &self.retained_truths
    }

    pub fn route_governing_reason(&self) -> Option<&str> {
        self.route_governing_reason.as_deref()
    }

    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }

    pub fn receipt_governing_reason(&self) -> &str {
        &self.receipt_governing_reason
    }

    pub fn receipt_denial_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.receipt_denial_cause
    }

    pub fn evidence_origin(&self) -> ForgeQueryDeclarationEnvelopeEvidenceOrigin {
        self.evidence_origin
    }

    pub fn mixed_origin(&self) -> bool {
        self.mixed_origin
    }
}
