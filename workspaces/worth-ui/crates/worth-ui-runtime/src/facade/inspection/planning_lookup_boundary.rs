use worth_ui_inspection::{
    UiEvidenceSliceOmission, UiInspectionQuery, UiInspectionRelevanceOutcome,
    UiInspectionScope, UiInspectionTarget,
};

use crate::evidence::{
    UiEvidenceAuthorityGeneration, UiEvidenceSliceAssembly, UiEvidenceSliceAssemblyInput,
    UiInspectionCostMetrics,
};
use crate::facade::{UiInspectionReceipt, WorthUiApp};
use crate::runtime::WorthUiRetainedAllocationPlanningEvidenceRegistry;

pub(crate) struct WorthUiPlanningInspectionBoundary<'a> {
    retained_planning: &'a WorthUiRetainedAllocationPlanningEvidenceRegistry,
}

impl<'a> WorthUiPlanningInspectionBoundary<'a> {
    pub(crate) const fn new(
        retained_planning: &'a WorthUiRetainedAllocationPlanningEvidenceRegistry,
    ) -> Self {
        Self { retained_planning }
    }

    pub(crate) fn inspect(
        &self,
        app: &WorthUiApp,
        query: UiInspectionQuery,
    ) -> Option<UiInspectionReceipt> {
        if query.scope() != UiInspectionScope::Planning
            || !matches!(query.target(), UiInspectionTarget::ProductRoot)
        {
            return None;
        }

        let support_report = app.inspection_support_report_for(&query);
        let relevance_admission = query
            .admit_relevance()
            .refined_for_support_report(support_report);
        if !matches!(
            relevance_admission.outcome(),
            UiInspectionRelevanceOutcome::Matched
        ) {
            return Some(UiInspectionReceipt::from_support(
                query,
                relevance_admission,
                support_report,
                None,
            ));
        }

        let receipts = self.retained_planning.retained_receipts();
        if receipts.is_empty() {
            return Some(UiInspectionReceipt::from_support(
                query,
                relevance_admission,
                support_report,
                None,
            ));
        }
        let Some(authority_generation) = shared_authority_generation(&receipts) else {
            return Some(UiInspectionReceipt::from_support(
                query,
                relevance_admission,
                support_report,
                None,
            ));
        };

        let refs = receipts
            .iter()
            .flat_map(
                |receipt: &crate::evidence::UiAllocationPlanningInspectionReceipt| {
                    receipt.evidence_slice().refs().iter().copied()
                },
            )
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let detail = selected_detail(&query, &receipts);
        let detail_omitted = query.allocation_planning_question().is_none()
            || (query.allocation_planning_question().is_some() && receipts.len() != 1);
        let assembly = UiEvidenceSliceAssembly::assemble(
            &query,
            UiEvidenceSliceAssemblyInput::new(authority_generation, refs)
                .with_materialized_detail(detail)
                .with_detail_available(true)
                .with_omission(detail_omitted.then_some(UiEvidenceSliceOmission::ByScope {
                    scope: query.scope(),
                }))
                .with_cost_metrics(UiInspectionCostMetrics::new(1, receipts.len(), 0, false)),
        );

        Some(UiInspectionReceipt::from_support_and_assembled_slice(
            query,
            relevance_admission,
            support_report,
            authority_generation,
            assembly,
        ))
    }
}

fn shared_authority_generation(
    receipts: &[crate::evidence::UiAllocationPlanningInspectionReceipt],
) -> Option<UiEvidenceAuthorityGeneration> {
    let first = receipts
        .first()?
        .evidence_slice()
        .authority_generation();
    receipts
        .iter()
        .all(|receipt| receipt.evidence_slice().authority_generation() == first)
        .then_some(first)
}

fn selected_detail(
    query: &UiInspectionQuery,
    receipts: &[crate::evidence::UiAllocationPlanningInspectionReceipt],
) -> Option<crate::evidence::UiEvidenceMaterializedDetail> {
    query.allocation_planning_question()?;
    if receipts.len() != 1 {
        return None;
    }

    receipts[0].evidence_slice().materialized_detail().cloned()
}
