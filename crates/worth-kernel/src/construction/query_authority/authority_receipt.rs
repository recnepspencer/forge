use crate::construction::digest::digest_owned_parts;

use super::authority_request::PrimitiveConstructionQueryAuthorityRequest;
use super::support_summary::PrimitiveConstructionQueryAuthoritySupportSummary;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryAuthorityReceipt {
    subject: String,
    purpose: String,
    authority_basis_digest: String,
    request_digest: String,
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    configured_handle_support_snapshot_digest: String,
    validated_config_digest: String,
    support_pin_contract_digest: String,
    support_pin_report_digest: String,
    evaluated_support_snapshot_digest: String,
    evaluated_support_source_matrix_digest: String,
    support_summary: PrimitiveConstructionQueryAuthoritySupportSummary,
    authority_receipt_digest: String,
}

impl PrimitiveConstructionQueryAuthorityReceipt {
    pub(crate) fn new(
        request: &PrimitiveConstructionQueryAuthorityRequest,
        handle_identity_digest: impl Into<String>,
        operating_context_identity_digest: impl Into<String>,
        configured_handle_support_snapshot_digest: impl Into<String>,
        validated_config_digest: impl Into<String>,
        support_pin_contract_digest: impl Into<String>,
        support_pin_report_digest: impl Into<String>,
        evaluated_support_snapshot_digest: impl Into<String>,
        evaluated_support_source_matrix_digest: impl Into<String>,
        support_summary: PrimitiveConstructionQueryAuthoritySupportSummary,
    ) -> Self {
        let handle_identity_digest = handle_identity_digest.into();
        let operating_context_identity_digest = operating_context_identity_digest.into();
        let configured_handle_support_snapshot_digest =
            configured_handle_support_snapshot_digest.into();
        let validated_config_digest = validated_config_digest.into();
        let support_pin_contract_digest = support_pin_contract_digest.into();
        let support_pin_report_digest = support_pin_report_digest.into();
        let evaluated_support_snapshot_digest = evaluated_support_snapshot_digest.into();
        let evaluated_support_source_matrix_digest = evaluated_support_source_matrix_digest.into();
        let authority_receipt_digest = digest_owned_parts(&[
            "primitive-construction-query-authority-receipt".to_string(),
            format!("request:{}", request.request_digest()),
            format!("authority-basis:{}", request.authority_basis_digest()),
            format!("handle:{handle_identity_digest}"),
            format!("context:{operating_context_identity_digest}"),
            format!("configured-handle-support:{configured_handle_support_snapshot_digest}"),
            format!("validated-config:{validated_config_digest}"),
            format!("support-pin-contract:{support_pin_contract_digest}"),
            format!("support-pin-report:{support_pin_report_digest}"),
            format!("evaluated-support-snapshot:{evaluated_support_snapshot_digest}"),
            format!("evaluated-support-source-matrix:{evaluated_support_source_matrix_digest}"),
            format!(
                "support-pin-requirements:{}",
                support_summary.requirement_count()
            ),
            format!(
                "support-pin-matched-required:{}",
                support_summary.matched_required_count()
            ),
            format!("support-pin-findings:{}", support_summary.finding_count()),
            format!(
                "support-pin-blocking-findings:{}",
                support_summary.blocking_finding_count()
            ),
            format!("support-pin-satisfied:{}", support_summary.satisfied()),
        ]);
        Self {
            subject: request.subject().to_string(),
            purpose: request.purpose().to_string(),
            authority_basis_digest: request.authority_basis_digest().to_string(),
            request_digest: request.request_digest().to_string(),
            handle_identity_digest,
            operating_context_identity_digest,
            configured_handle_support_snapshot_digest,
            validated_config_digest,
            support_pin_contract_digest,
            support_pin_report_digest,
            evaluated_support_snapshot_digest,
            evaluated_support_source_matrix_digest,
            support_summary,
            authority_receipt_digest,
        }
    }

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }

    pub(crate) fn purpose(&self) -> &str {
        &self.purpose
    }

    pub(crate) fn authority_basis_digest(&self) -> &str {
        &self.authority_basis_digest
    }

    pub(crate) fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub(crate) fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub(crate) fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub(crate) fn configured_handle_support_snapshot_digest(&self) -> &str {
        &self.configured_handle_support_snapshot_digest
    }

    pub(crate) fn validated_config_digest(&self) -> &str {
        &self.validated_config_digest
    }

    pub(crate) fn support_pin_contract_digest(&self) -> &str {
        &self.support_pin_contract_digest
    }

    pub(crate) fn support_pin_report_digest(&self) -> &str {
        &self.support_pin_report_digest
    }

    pub(crate) fn evaluated_support_snapshot_digest(&self) -> &str {
        &self.evaluated_support_snapshot_digest
    }

    pub(crate) fn evaluated_support_source_matrix_digest(&self) -> &str {
        &self.evaluated_support_source_matrix_digest
    }

    pub(crate) fn evaluated_support_pin_count(&self) -> usize {
        self.support_summary.requirement_count()
    }

    pub(crate) fn matched_support_pin_count(&self) -> usize {
        self.support_summary.matched_required_count()
    }

    pub(crate) fn support_pin_finding_count(&self) -> usize {
        self.support_summary.finding_count()
    }

    pub(crate) fn support_pin_blocking_finding_count(&self) -> usize {
        self.support_summary.blocking_finding_count()
    }

    pub(crate) fn support_pins_satisfied(&self) -> bool {
        self.support_summary.satisfied()
    }

    pub(crate) fn authority_receipt_digest(&self) -> &str {
        &self.authority_receipt_digest
    }
}
