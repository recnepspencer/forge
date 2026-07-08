use crate::evidence::{UiEvidenceExpansion, UiEvidenceRef};
use crate::facade::inspection_bridge::{UiInspectionClosureReport, UiInspectionReceipt};
use crate::facade::WorthUiApp;
use worth_ui_inspection::{
    UiInspectionForeignEvidenceCitation, UiInspectionForeignEvidenceRef, UiInspectionQuery,
    UiInspectionScope, UiInspectionSupportReport,
};

pub struct UiInspectionAiHarness<'a> {
    app: &'a WorthUiApp,
}

impl<'a> UiInspectionAiHarness<'a> {
    pub const fn new(app: &'a WorthUiApp) -> Self {
        Self { app }
    }

    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        self.app.inspect(query)
    }

    pub fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        self.app.expand_evidence_ref(evidence_ref, richness)
    }

    pub fn support_report(&self, scope: UiInspectionScope) -> UiInspectionSupportReport {
        self.app.inspection_support_report(scope)
    }

    pub fn closure_report(&self) -> UiInspectionClosureReport {
        self.app.inspection_closure_report()
    }

    pub fn cite_foreign_evidence(
        &self,
        foreign_ref: UiInspectionForeignEvidenceRef,
    ) -> UiInspectionForeignEvidenceCitation {
        super::cite_foreign_evidence(self.app.retained_obligation_registry(), foreign_ref)
    }
}
