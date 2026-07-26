use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
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
