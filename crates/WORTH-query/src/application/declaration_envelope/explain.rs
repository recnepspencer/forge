use crate::application::{
    WorthQueryDeclarationReceiptDenialCause, WorthQueryDeclarationRoutePlanDenialCause,
};

use super::class::WorthQueryDeclarationEnvelopeEvidenceOrigin;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationEnvelopeExplanation {
    crossing_posture: &'static str,
    evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
    route_reference: Option<String>,
    retained_truths: Vec<String>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
}

impl WorthQueryDeclarationEnvelopeExplanation {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        crossing_posture: &'static str,
        evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
        route_reference: Option<String>,
        retained_truths: Vec<String>,
        route_governing_reason: Option<String>,
        route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
        receipt_governing_reason: String,
        receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    ) -> Self {
        Self {
            crossing_posture,
            evidence_origin,
            route_reference,
            retained_truths,
            route_governing_reason,
            route_denial_cause,
            receipt_governing_reason,
            receipt_denial_cause,
        }
    }

    pub fn crossing_posture(&self) -> &'static str {
        self.crossing_posture
    }

    pub fn evidence_origin(&self) -> WorthQueryDeclarationEnvelopeEvidenceOrigin {
        self.evidence_origin
    }

    pub fn route_reference(&self) -> Option<&str> {
        self.route_reference.as_deref()
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
}
