use crate::runtime::{
    WorthUiQueryBindingIdentity, WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataNode,
    WorthUiVirtualizedDataPlan, WorthUiVisibleRange,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedPlanSummaryRequest {
    row_budget: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiVirtualizedPlanSummary {
    row: WorthUiVirtualizedDataNode,
    evidence: Option<worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceReference>,
    total_view_row_count: usize,
    family_index_lookup_count: usize,
    direct_row_lookup_count: usize,
    evidence_reference_lookup_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiVirtualizedPlanSummaryDenial {
    ActivePlanIsQueryFree,
    ZeroRowBudget,
    CorruptViewIndex,
    QueryNotInstalled,
    ForeignInstalledReference,
}

impl WorthUiVirtualizedPlanSummaryRequest {
    pub fn first_view() -> Self {
        Self { row_budget: 1 }
    }

    pub fn new(row_budget: usize) -> Self {
        Self { row_budget }
    }

    pub fn row_budget(self) -> usize {
        self.row_budget
    }
}

impl WorthUiVirtualizedPlanSummary {
    pub(crate) fn from_plan(
        plan: &WorthUiVirtualizedDataPlan,
        query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
        request: WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<Self, WorthUiVirtualizedPlanSummaryDenial> {
        if request.row_budget == 0 {
            return Err(WorthUiVirtualizedPlanSummaryDenial::ZeroRowBudget);
        }
        let row = plan
            .first_row()
            .ok_or(WorthUiVirtualizedPlanSummaryDenial::CorruptViewIndex)?;
        let evidence = match query_binding.execution_evidence_for(row.installed_reference()) {
            Ok(evidence) => Some(evidence),
            Err(worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial::ProjectionNotAdmitted) => None,
            Err(worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial::QueryNotInstalled) => {
                return Err(WorthUiVirtualizedPlanSummaryDenial::QueryNotInstalled);
            }
            Err(worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceDenial::ForeignInstalledReference) => {
                return Err(WorthUiVirtualizedPlanSummaryDenial::ForeignInstalledReference);
            }
        };
        Ok(Self {
            row,
            evidence,
            total_view_row_count: plan.row_count(),
            family_index_lookup_count: 1,
            direct_row_lookup_count: 1,
            evidence_reference_lookup_count: 1,
        })
    }

    pub fn target(&self, range: WorthUiVisibleRange) -> WorthUiVirtualizedDataFrameTarget {
        WorthUiVirtualizedDataFrameTarget::view_binding(
            crate::runtime::WorthUiViewBindingHandle::from_runtime_handle(
                self.row.runtime_handle(),
            ),
            range,
        )
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        self.row.binding_identity()
    }

    pub fn definition(&self) -> &worth_ui_query_binding::WorthUiQueryViewDefinition {
        self.row.definition()
    }

    pub fn evidence(
        &self,
    ) -> Option<&worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceReference> {
        self.evidence.as_ref()
    }

    pub fn total_view_row_count(&self) -> usize {
        self.total_view_row_count
    }

    pub fn family_index_lookup_count(&self) -> usize {
        self.family_index_lookup_count
    }

    pub fn direct_row_lookup_count(&self) -> usize {
        self.direct_row_lookup_count
    }

    pub fn evidence_reference_lookup_count(&self) -> usize {
        self.evidence_reference_lookup_count
    }
}
