use crate::application::{
    WorthQueryDeclarationEnvelopeEvidenceOrigin, WorthQueryDeclarationReceiptDenialCause,
    WorthQueryDeclarationRoutePlanDenialCause,
};
use crate::basis_lifecycle::BasisFamily;

use super::contract::WorthQueryDeclarationSignalExecutionFamily;

pub struct WorthQueryDeclarationSignalCompatibilityExplanation {
    compatibility_posture: &'static str,
    execution_family: WorthQueryDeclarationSignalExecutionFamily,
    basis_families: Vec<BasisFamily>,
    retained_truths: Vec<String>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
}

impl WorthQueryDeclarationSignalCompatibilityExplanation {
    pub(crate) fn new(
        compatibility_posture: &'static str,
        execution_family: WorthQueryDeclarationSignalExecutionFamily,
        basis_families: Vec<BasisFamily>,
        retained_truths: Vec<String>,
        route_governing_reason: Option<String>,
        route_denial_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
        receipt_governing_reason: String,
        receipt_denial_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
        evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
    ) -> Self {
        Self {
            compatibility_posture,
            execution_family,
            basis_families,
            retained_truths,
            route_governing_reason,
            route_denial_cause,
            receipt_governing_reason,
            receipt_denial_cause,
            evidence_origin,
        }
    }

    pub fn compatibility_posture(&self) -> &'static str {
        self.compatibility_posture
    }

    pub fn execution_family(&self) -> WorthQueryDeclarationSignalExecutionFamily {
        self.execution_family
    }

    pub fn basis_families(&self) -> &[BasisFamily] {
        &self.basis_families
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
}
