use worth_ui_inspection::{UiEvidenceSliceOmission, UiInspectionQuery};

use crate::evidence::{UiAllocationPlanningInspectionReceipt, UiEvidenceMaterializedDetail};

pub(crate) struct PlanningDetailAdmission {
    materialized_detail: Option<UiEvidenceMaterializedDetail>,
    omission: Option<UiEvidenceSliceOmission>,
}

impl PlanningDetailAdmission {
    pub(crate) fn materialized_detail(&self) -> Option<UiEvidenceMaterializedDetail> {
        self.materialized_detail.clone()
    }

    pub(crate) fn omission(&self) -> Option<UiEvidenceSliceOmission> {
        self.omission
    }
}

pub(crate) fn classify_planning_detail_admission(
    query: &UiInspectionQuery,
    receipts: &[UiAllocationPlanningInspectionReceipt],
) -> PlanningDetailAdmission {
    let materialized_detail = selected_planning_detail(query, receipts);
    let omission = classify_planning_detail_omission(query, receipts).then_some(
        UiEvidenceSliceOmission::ByScope {
            scope: query.scope(),
        },
    );
    PlanningDetailAdmission {
        materialized_detail,
        omission,
    }
}

fn classify_planning_detail_omission(
    query: &UiInspectionQuery,
    receipts: &[UiAllocationPlanningInspectionReceipt],
) -> bool {
    query.allocation_planning_question().is_none() || receipts.len() != 1
}

fn selected_planning_detail(
    query: &UiInspectionQuery,
    receipts: &[UiAllocationPlanningInspectionReceipt],
) -> Option<UiEvidenceMaterializedDetail> {
    query.allocation_planning_question()?;
    if receipts.len() != 1 {
        return None;
    }

    receipts[0].evidence_slice().materialized_detail().cloned()
}
