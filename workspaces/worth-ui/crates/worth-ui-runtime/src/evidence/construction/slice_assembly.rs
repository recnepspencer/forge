use worth_ui_inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiEvidenceSliceOmission, UiInspectionCostReceipt,
    UiInspectionQuery,
};

use super::{
    cost_receipt::UiInspectionCostMetrics, evidence_family_summary, evidence_slice,
    slice_ordering::order_refs,
};
use crate::evidence::{
    UiEvidenceAuthorityGeneration, UiEvidenceMaterializedDetail, UiEvidenceRef, UiEvidenceSlice,
};

pub(crate) struct UiEvidenceSliceAssemblyInput {
    authority_generation: UiEvidenceAuthorityGeneration,
    refs: Box<[UiEvidenceRef]>,
    materialized_detail: Option<UiEvidenceMaterializedDetail>,
    detail_available: bool,
    omission: Option<UiEvidenceSliceOmission>,
    cost_metrics: UiInspectionCostMetrics,
}

pub(crate) struct UiEvidenceSliceAssembly {
    slice: UiEvidenceSlice,
    cost: UiInspectionCostReceipt,
}

impl UiEvidenceSliceAssemblyInput {
    pub(crate) fn new(
        authority_generation: UiEvidenceAuthorityGeneration,
        refs: Box<[UiEvidenceRef]>,
    ) -> Self {
        Self {
            authority_generation,
            refs,
            materialized_detail: None,
            detail_available: false,
            omission: None,
            cost_metrics: UiInspectionCostMetrics::default(),
        }
    }

    pub(crate) fn with_materialized_detail(
        mut self,
        materialized_detail: Option<UiEvidenceMaterializedDetail>,
    ) -> Self {
        self.materialized_detail = materialized_detail;
        self
    }

    pub(crate) fn with_detail_available(mut self, detail_available: bool) -> Self {
        self.detail_available = detail_available;
        self
    }

    pub(crate) fn with_omission(mut self, omission: Option<UiEvidenceSliceOmission>) -> Self {
        self.omission = omission;
        self
    }

    pub(crate) fn with_cost_metrics(mut self, cost_metrics: UiInspectionCostMetrics) -> Self {
        self.cost_metrics = cost_metrics;
        self
    }
}

impl UiEvidenceSliceAssembly {
    pub(crate) fn assemble(query: &UiInspectionQuery, input: UiEvidenceSliceAssemblyInput) -> Self {
        let refs = order_refs(input.refs.into_vec());
        let materialized_detail =
            resolve_materialized_detail(query, input.materialized_detail, input.detail_available);
        let omission = classify_slice_omission(query, input.omission, input.detail_available);
        let slice = construct_evidence_slice(
            input.authority_generation,
            refs,
            materialized_detail,
            omission,
        );
        let cost = finalize_inspection_cost(&slice, &input.cost_metrics, &omission);

        Self { slice, cost }
    }

    pub(crate) fn slice(&self) -> &UiEvidenceSlice {
        &self.slice
    }

    pub(crate) fn into_slice(self) -> UiEvidenceSlice {
        self.slice
    }

    pub(crate) fn cost(&self) -> UiInspectionCostReceipt {
        self.cost
    }
}

fn classify_slice_omission(
    query: &UiInspectionQuery,
    explicit_omission: Option<UiEvidenceSliceOmission>,
    detail_available: bool,
) -> Option<UiEvidenceSliceOmission> {
    explicit_omission.or_else(|| {
        if detail_available
            && query.richness() == UiEvidenceRichness::materialized_detail()
            && query.budget() == UiEvidenceBudget::Narrow
        {
            Some(UiEvidenceSliceOmission::ByBudget {
                budget: query.budget(),
            })
        } else if detail_available && query.richness() != UiEvidenceRichness::materialized_detail()
        {
            Some(UiEvidenceSliceOmission::ByScope {
                scope: query.scope(),
            })
        } else {
            None
        }
    })
}

fn resolve_materialized_detail(
    query: &UiInspectionQuery,
    materialized_detail: Option<UiEvidenceMaterializedDetail>,
    detail_available: bool,
) -> Option<UiEvidenceMaterializedDetail> {
    if detail_available
        && query.richness() == UiEvidenceRichness::materialized_detail()
        && query.budget() == UiEvidenceBudget::Narrow
    {
        None
    } else {
        materialized_detail
    }
}

fn construct_evidence_slice(
    authority_generation: UiEvidenceAuthorityGeneration,
    refs: Box<[UiEvidenceRef]>,
    materialized_detail: Option<UiEvidenceMaterializedDetail>,
    omission: Option<UiEvidenceSliceOmission>,
) -> UiEvidenceSlice {
    let family_summaries = refs
        .iter()
        .fold(
            std::collections::BTreeMap::new(),
            |mut counts, evidence_ref| {
                *counts.entry(evidence_ref.family()).or_insert(0usize) += 1;
                counts
            },
        )
        .into_iter()
        .map(|(family, ref_count)| evidence_family_summary(family, ref_count))
        .collect::<Vec<_>>()
        .into_boxed_slice();

    evidence_slice(
        authority_generation,
        refs,
        family_summaries,
        materialized_detail,
        omission,
    )
}

fn finalize_inspection_cost(
    slice: &UiEvidenceSlice,
    cost_metrics: &UiInspectionCostMetrics,
    omission: &Option<UiEvidenceSliceOmission>,
) -> UiInspectionCostReceipt {
    let materialized_records = slice
        .materialized_detail()
        .map(materialized_record_count)
        .unwrap_or(0);
    let omitted_by_budget = usize::from(matches!(
        omission,
        Some(UiEvidenceSliceOmission::ByBudget { .. })
    ));
    cost_metrics.finalize(slice.refs().len(), materialized_records, omitted_by_budget)
}

fn materialized_record_count(detail: &UiEvidenceMaterializedDetail) -> usize {
    match detail {
        UiEvidenceMaterializedDetail::AllocationPlanning(_) => 1,
        UiEvidenceMaterializedDetail::Obligation(receipt) => receipt.projections().len(),
        UiEvidenceMaterializedDetail::Measurement(detail) => {
            detail.basis_inputs().len() + detail.dependency_lineage().len()
        }
    }
}
