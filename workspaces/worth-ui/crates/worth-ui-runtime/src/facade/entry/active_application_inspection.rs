use super::WorthUiActiveApplicationSession;

/// Inspection evidence bound to the exact generation currently executing.
pub struct WorthUiActiveInspectionReceipt {
    generation_identity:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    receipt: crate::facade::inspection_bridge::UiInspectionReceipt,
}

impl WorthUiActiveApplicationSession {
    pub fn inspect(
        &self,
        query: worth_ui_inspection::UiInspectionQuery,
    ) -> WorthUiActiveInspectionReceipt {
        WorthUiActiveInspectionReceipt {
            generation_identity: self.generation_identity().clone(),
            receipt: self.application.inspect(query),
        }
    }

    /// Expand evidence through the exact active generation's retained
    /// inspection authorities.
    pub fn expand_evidence_ref(
        &self,
        evidence_ref: crate::evidence::UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> crate::evidence::UiEvidenceExpansion {
        self.application
            .expand_evidence_ref(evidence_ref, requested_richness)
    }

    pub fn discard_evidence_slice(&self, slice_ref: crate::evidence::UiEvidenceSliceRef) -> bool {
        self.application.discard_evidence_slice(slice_ref)
    }

    pub fn inspection_support_report(
        &self,
        scope: worth_ui_inspection::UiInspectionScope,
    ) -> worth_ui_inspection::UiInspectionSupportReport {
        self.application.inspection_support_report(scope)
    }

    pub fn inspection_support_report_for(
        &self,
        query: &worth_ui_inspection::UiInspectionQuery,
    ) -> worth_ui_inspection::UiInspectionSupportReport {
        self.application.inspection_support_report_for(query)
    }

    pub fn inspection_closure_report(
        &self,
    ) -> crate::facade::inspection_bridge::UiInspectionClosureReport {
        self.application.inspection_closure_report()
    }

    pub fn inspection_observation(
        &self,
    ) -> crate::facade::inspection_bridge::UiInspectionFacadeObservation {
        self.application.inspection_observation()
    }
}

impl WorthUiActiveInspectionReceipt {
    pub fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation_identity
    }

    pub fn receipt(&self) -> &crate::facade::inspection_bridge::UiInspectionReceipt {
        &self.receipt
    }
}
