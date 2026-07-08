use worth_ui_inspection::{
    UiInspectionQuery, UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget,
};

use crate::evidence::{
    UiAllocationPlanningInspectionReceipt, UiEvidenceAuthorityGeneration,
    UiEvidenceSliceAssembly, UiEvidenceSliceAssemblyInput, UiInspectionCostMetrics,
};
use crate::facade::inspection::planning_detail_admission::classify_planning_detail_admission;
use crate::facade::inspection_bridge::UiInspectionReceipt;
use crate::facade::WorthUiApp;
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
        if !admit_planning_scope(&query) {
            return None;
        }

        let support_report = app.inspection_support_report(query.scope());
        let relevance_admission = admit_planning_relevance(&query, support_report);
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
        let Some(authority_generation) = resolve_shared_authority_generation(&receipts) else {
            return Some(UiInspectionReceipt::from_support(
                query,
                relevance_admission,
                support_report,
                None,
            ));
        };

        let refs = collect_planning_evidence_refs(&receipts);
        let detail_admission = classify_planning_detail_admission(&query, &receipts);
        let assembly = assemble_planning_evidence_slice(
            &query,
            authority_generation,
            refs,
            detail_admission,
            receipts.len(),
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

fn admit_planning_scope(query: &UiInspectionQuery) -> bool {
    query.scope() == UiInspectionScope::Planning
        && matches!(query.target(), UiInspectionTarget::ProductRoot)
}

fn admit_planning_relevance(
    query: &UiInspectionQuery,
    support_report: worth_ui_inspection::UiInspectionSupportReport,
) -> worth_ui_inspection::UiInspectionRelevanceAdmission {
    query
        .admit_relevance()
        .refined_for_support_report(support_report)
}

fn resolve_shared_authority_generation(
    receipts: &[UiAllocationPlanningInspectionReceipt],
) -> Option<UiEvidenceAuthorityGeneration> {
    if receipts.is_empty() {
        return None;
    }

    let first = receipts
        .first()?
        .evidence_slice()
        .authority_generation();
    receipts
        .iter()
        .all(|receipt| receipt.evidence_slice().authority_generation() == first)
        .then_some(first)
}

fn collect_planning_evidence_refs(
    receipts: &[UiAllocationPlanningInspectionReceipt],
) -> Box<[crate::evidence::UiEvidenceRef]> {
    receipts
        .iter()
        .flat_map(|receipt| receipt.evidence_slice().refs().iter().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn assemble_planning_evidence_slice(
    query: &UiInspectionQuery,
    authority_generation: UiEvidenceAuthorityGeneration,
    refs: Box<[crate::evidence::UiEvidenceRef]>,
    detail_admission: crate::facade::inspection::planning_detail_admission::PlanningDetailAdmission,
    receipt_count: usize,
) -> UiEvidenceSliceAssembly {
    UiEvidenceSliceAssembly::assemble(
        query,
        UiEvidenceSliceAssemblyInput::new(authority_generation, refs)
            .with_materialized_detail(detail_admission.materialized_detail())
            .with_detail_available(true)
            .with_omission(detail_admission.omission())
            .with_cost_metrics(UiInspectionCostMetrics::new(1, receipt_count, 0, false)),
    )
}