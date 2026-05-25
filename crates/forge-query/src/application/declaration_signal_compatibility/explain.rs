use crate::application::{
    ForgeQueryDeclarationEnvelopeEvidenceOrigin, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause,
};
use crate::basis_lifecycle::BasisFamily;

use super::contract::ForgeQueryDeclarationSignalExecutionFamily;

pub struct ForgeQueryDeclarationSignalCompatibilityExplanation {
    compatibility_posture: &'static str,
    execution_family: ForgeQueryDeclarationSignalExecutionFamily,
    basis_families: Vec<BasisFamily>,
    retained_truths: Vec<String>,
    route_governing_reason: Option<String>,
    route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_governing_reason: String,
    receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    evidence_origin: ForgeQueryDeclarationEnvelopeEvidenceOrigin,
}

impl ForgeQueryDeclarationSignalCompatibilityExplanation {
    pub(crate) fn new(
        compatibility_posture: &'static str,
        execution_family: ForgeQueryDeclarationSignalExecutionFamily,
        basis_families: Vec<BasisFamily>,
        retained_truths: Vec<String>,
        route_governing_reason: Option<String>,
        route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
        receipt_governing_reason: String,
        receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
        evidence_origin: ForgeQueryDeclarationEnvelopeEvidenceOrigin,
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

    pub fn execution_family(&self) -> ForgeQueryDeclarationSignalExecutionFamily {
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
}
