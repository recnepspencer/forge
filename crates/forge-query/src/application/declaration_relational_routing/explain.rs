use crate::application::{
    ForgeQueryDeclarationEnvelopeEvidenceOrigin, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause,
};

use super::contract::{
    ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalTruthClaim,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRelationalRoutingExplanation {
    crossing_posture: &'static str,
    truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
    authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
    binding_surface: &'static str,
    retained_truths: Vec<String>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    evidence_origin: ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    mixed_origin: bool,
}

impl ForgeQueryDeclarationRelationalRoutingExplanation {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        crossing_posture: &'static str,
        truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
        authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
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

    pub fn truth_claim(&self) -> ForgeQueryDeclarationRelationalTruthClaim {
        self.truth_claim
    }

    pub fn authority_family(&self) -> ForgeQueryDeclarationRelationalAuthorityFamily {
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
