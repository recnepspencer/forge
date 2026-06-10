use forge_foundational::facade::{
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceSupportTruthKind,
};

use crate::{
    ForgeServerEvidenceInput, ForgeServerOperatorEvidenceFacade,
    ForgeServerOperatorEvidenceMaterializationError, ForgeServerOperatorEvidenceRecord,
    ForgeServerResponseEnvelope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerExternalEvidenceRecord {
    surface_label: String,
    operator_record: ForgeServerOperatorEvidenceRecord,
    classification_label: String,
    canonical_digest: String,
}

impl ForgeServerExternalEvidenceRecord {
    pub fn from_response_envelope(
        surface_label: impl Into<String>,
        response: ForgeServerResponseEnvelope,
        facade: &ForgeServerOperatorEvidenceFacade,
    ) -> Result<Self, ForgeServerOperatorEvidenceMaterializationError> {
        Self::project(surface_label, response, &facade)
    }

    pub(crate) fn project(
        surface_label: impl Into<String>,
        response: ForgeServerResponseEnvelope,
        facade: &ForgeServerOperatorEvidenceFacade,
    ) -> Result<Self, ForgeServerOperatorEvidenceMaterializationError> {
        let surface_label = normalize_surface_label(surface_label);
        let operator_record =
            facade.record_with_defaults(ForgeServerEvidenceInput::response_envelope(response))?;
        let classification_label = format!(
            "{}_{}",
            surface_label,
            classification_suffix(operator_record.classification())
        );
        let canonical_digest = format!(
            "forge-server-external-evidence-v1|surface={surface_label}|response={}|classification={classification_label}|support={:?}|diagnostics={:?}",
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

    pub fn operator_record(&self) -> &ForgeServerOperatorEvidenceRecord {
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

fn classification_suffix(classification: &crate::ForgeServerOperatorEvidenceClass) -> &'static str {
    match classification {
        crate::ForgeServerOperatorEvidenceClass::RequestContextDenied(_) => {
            "request_context_denied"
        }
        crate::ForgeServerOperatorEvidenceClass::MiddlewareDenied(_) => "middleware_denied",
        crate::ForgeServerOperatorEvidenceClass::QueryHandoffDenied(_) => "query_handoff_denied",
        crate::ForgeServerOperatorEvidenceClass::QueryReadSucceeded
        | crate::ForgeServerOperatorEvidenceClass::QueryMutationSucceeded
        | crate::ForgeServerOperatorEvidenceClass::DownstreamDeliverySucceeded => "succeeded",
    }
}
