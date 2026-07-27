use worth_ui_inspection::{UiEvidenceRichness, UiInspectionQuery, UiInspectionScope};

use super::WorthUiApplicationSessionState;
use crate::facade::inspection_bridge::{
    UiInspectionClosureReport, UiInspectionFacadeObservation, UiInspectionReceipt,
};
#[cfg(any(test, feature = "certification-support"))]
use crate::runtime::{WorthUiActiveRuntimeObservation, WorthUiStateQueryResidueScan};

impl WorthUiApplicationSessionState {
    pub(crate) fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        self.app.inspect(query)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_active_runtime(&self) -> WorthUiActiveRuntimeObservation {
        self.runtime.inspect_active()
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_query_state_residue(&self) -> WorthUiStateQueryResidueScan {
        self.runtime.inspect_query_state_residue()
    }

    pub(crate) fn expand_evidence_ref(
        &self,
        evidence_ref: crate::evidence::UiEvidenceRef,
        requested_richness: UiEvidenceRichness,
    ) -> crate::evidence::UiEvidenceExpansion {
        self.app
            .expand_evidence_ref(evidence_ref, requested_richness)
    }

    pub(crate) fn discard_evidence_slice(
        &self,
        slice_ref: crate::evidence::UiEvidenceSliceRef,
    ) -> bool {
        self.app.discard_evidence_slice(slice_ref)
    }

    pub(crate) fn inspection_support_report(
        &self,
        scope: UiInspectionScope,
    ) -> worth_ui_inspection::UiInspectionSupportReport {
        self.app.inspection_support_report(scope)
    }

    pub(crate) fn inspection_support_report_for(
        &self,
        query: &UiInspectionQuery,
    ) -> worth_ui_inspection::UiInspectionSupportReport {
        self.app.inspection_support_report_for(query)
    }

    pub(crate) fn inspection_closure_report(&self) -> UiInspectionClosureReport {
        self.app.inspection_closure_report()
    }

    pub(crate) fn inspection_observation(&self) -> UiInspectionFacadeObservation {
        self.app.inspection_observation()
    }
}
