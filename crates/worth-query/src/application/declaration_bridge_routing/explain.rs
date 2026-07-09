use crate::application::{
    WorthQueryDeclarationEnvelopeEvidenceOrigin, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationRoutePlanDenialCause,
};

use super::{
    contract::WorthQueryDeclarationBridgeContinuationFamily,
    request::{
        WorthQueryDeclarationBridgeContinuationMode, WorthQueryDeclarationBridgeTruthContext,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationBridgeRoutingExplanation {
    crossing_posture: &'static str,
    continuation_mode: WorthQueryDeclarationBridgeContinuationMode,
    truth_context: WorthQueryDeclarationBridgeTruthContext,
    continuation_family: WorthQueryDeclarationBridgeContinuationFamily,
    binding_surface: &'static str,
    retained_truths: Vec<String>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
    mixed_origin: bool,
}

impl WorthQueryDeclarationBridgeRoutingExplanation {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        crossing_posture: &'static str,
        continuation_mode: WorthQueryDeclarationBridgeContinuationMode,
        truth_context: WorthQueryDeclarationBridgeTruthContext,
        continuation_family: WorthQueryDeclarationBridgeContinuationFamily,
        binding_surface: &'static str,
        retained_truths: Vec<String>,
        route_governing_reason: Option<String>,
        route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
        receipt_governing_reason: String,
        receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
        evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
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

    pub fn continuation_mode(&self) -> WorthQueryDeclarationBridgeContinuationMode {
        self.continuation_mode
    }

    pub fn truth_context(&self) -> WorthQueryDeclarationBridgeTruthContext {
        self.truth_context
    }

    pub fn continuation_family(&self) -> WorthQueryDeclarationBridgeContinuationFamily {
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

    pub fn route_denial_cause(&self) -> Option<WorthQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }

    pub fn receipt_governing_reason(&self) -> &str {
        &self.receipt_governing_reason
    }

    pub fn receipt_denial_cause(&self) -> Option<WorthQueryDeclarationReceiptDenialCause> {
        self.receipt_denial_cause
    }

    pub fn evidence_origin(&self) -> WorthQueryDeclarationEnvelopeEvidenceOrigin {
        self.evidence_origin
    }

    pub fn mixed_origin(&self) -> bool {
        self.mixed_origin
    }
}
