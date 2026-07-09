use crate::application::{
    WorthQueryDeclarationEnvelopeEvidenceOrigin, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationRoutePlanDenialCause,
};

use super::contract::{
    WorthQueryDeclarationRelationalAuthorityFamily, WorthQueryDeclarationRelationalTruthClaim,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationRelationalRoutingExplanation {
    crossing_posture: &'static str,
    truth_claim: WorthQueryDeclarationRelationalTruthClaim,
    authority_family: WorthQueryDeclarationRelationalAuthorityFamily,
    binding_surface: &'static str,
    retained_truths: Vec<String>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
    mixed_origin: bool,
}

impl WorthQueryDeclarationRelationalRoutingExplanation {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        crossing_posture: &'static str,
        truth_claim: WorthQueryDeclarationRelationalTruthClaim,
        authority_family: WorthQueryDeclarationRelationalAuthorityFamily,
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
            truth_claim,
            authority_family,
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

    pub fn truth_claim(&self) -> WorthQueryDeclarationRelationalTruthClaim {
        self.truth_claim
    }

    pub fn authority_family(&self) -> WorthQueryDeclarationRelationalAuthorityFamily {
        self.authority_family
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
