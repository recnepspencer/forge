use worth_foundational::facade::{
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceSupportTruthKind,
};

use crate::{
    WorthServerEvidenceInput, WorthServerOperatorEvidenceFacade,
    WorthServerOperatorEvidenceMaterializationError, WorthServerOperatorEvidenceRecord,
    WorthServerResponseEnvelope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalEvidenceRecord {
    surface_label: String,
    operator_record: WorthServerOperatorEvidenceRecord,
    classification_label: String,
    canonical_digest: String,
}

impl WorthServerExternalEvidenceRecord {
    pub fn from_response_envelope(
        surface_label: impl Into<String>,
        response: WorthServerResponseEnvelope,
        facade: &WorthServerOperatorEvidenceFacade,
    ) -> Result<Self, WorthServerOperatorEvidenceMaterializationError> {
        Self::project(surface_label, response, &facade)
    }

    pub(crate) fn project(
        surface_label: impl Into<String>,
        response: WorthServerResponseEnvelope,
        facade: &WorthServerOperatorEvidenceFacade,
    ) -> Result<Self, WorthServerOperatorEvidenceMaterializationError> {
        let surface_label = normalize_surface_label(surface_label);
        let operator_record =
            facade.record_with_defaults(WorthServerEvidenceInput::response_envelope(response))?;
        let classification_label = format!(
            "{}_{}",
            surface_label,
            classification_suffix(operator_record.classification())
        );
        let canonical_digest = format!(
            "worth-server-external-evidence-v1|surface={surface_label}|response={}|classification={classification_label}|support={:?}|diagnostics={:?}",
            operator_record.response_digest(),
            operator_record.support_truth_kind(),
            operator_record.diagnostics_profile(),
        );
        Ok(Self {
            surface_label,
            operator_record,
            classification_label,
            canonical_digest,
        })
    }

    pub fn surface_label(&self) -> &str {
        &self.surface_label
    }

    pub fn operator_record(&self) -> &WorthServerOperatorEvidenceRecord {
        &self.operator_record
    }

    pub fn classification_label(&self) -> &str {
        &self.classification_label
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.operator_record.diagnostics_profile()
    }

    pub fn support_truth_kind(&self) -> FoundationalBoundaryEvidenceSupportTruthKind {
        self.operator_record.support_truth_kind()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn normalize_surface_label(surface_label: impl Into<String>) -> String {
    surface_label.into().trim().to_string()
}

fn classification_suffix(classification: &crate::WorthServerOperatorEvidenceClass) -> &'static str {
    match classification {
        crate::WorthServerOperatorEvidenceClass::RequestContextDenied(_) => {
            "request_context_denied"
        }
        crate::WorthServerOperatorEvidenceClass::MiddlewareDenied(_) => "middleware_denied",
        crate::WorthServerOperatorEvidenceClass::QueryHandoffDenied(_) => "query_handoff_denied",
        crate::WorthServerOperatorEvidenceClass::QueryReadSucceeded
        | crate::WorthServerOperatorEvidenceClass::QueryMutationSucceeded
        | crate::WorthServerOperatorEvidenceClass::DownstreamDeliverySucceeded => "succeeded",
    }
}
