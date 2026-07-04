use worth_ui_inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiEvidenceSliceOmission, UiInspectionCostReceipt,
    UiInspectionQuery,
};

use super::{
    evidence_family_summary, evidence_slice, UiEvidenceAuthorityGeneration,
    UiEvidenceMaterializedDetail, UiEvidenceRef, UiEvidenceSlice,
};
use super::{evidence_slice_ordering::order_refs, inspection_cost_receipt::UiInspectionCostMetrics};

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

    pub(crate) fn with_cost_metrics(mut self, cost_metrics: UiInspectionCostMetrics) -> Self {
        self.cost_metrics = cost_metrics;
        self
    }
}

impl UiEvidenceSliceAssembly {
    pub(crate) fn assemble(
        query: &UiInspectionQuery,
        input: UiEvidenceSliceAssemblyInput,
    ) -> Self {
        let refs = order_refs(input.refs.into_vec());
        let materialized_detail = if input.detail_available
            && query.richness() == UiEvidenceRichness::materialized_detail()
            && query.budget() == UiEvidenceBudget::Narrow
        {
            None
        } else {
            input.materialized_detail
        };
        let family_summaries = refs
            .iter()
            .fold(std::collections::BTreeMap::new(), |mut counts, evidence_ref| {
                *counts.entry(evidence_ref.family()).or_insert(0usize) += 1;
                counts
            })
            .into_iter()
            .map(|(family, ref_count)| evidence_family_summary(family, ref_count))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let omission = input.omission.or_else(|| {
            if input.detail_available
                && query.richness() == UiEvidenceRichness::materialized_detail()
                && query.budget() == UiEvidenceBudget::Narrow
            {
                Some(UiEvidenceSliceOmission::ByBudget {
                    budget: query.budget(),
                })
            } else if input.detail_available
                && query.richness() != UiEvidenceRichness::materialized_detail()
            {
                Some(UiEvidenceSliceOmission::ByScope {
                    scope: query.scope(),
                })
            } else {
                None
            }
        });
        let materialized_records = materialized_detail
            .as_ref()
            .map(materialized_record_count)
            .unwrap_or(0);
        let omitted_by_budget = usize::from(matches!(
            omission,
            Some(UiEvidenceSliceOmission::ByBudget { .. })
        ));
        let cost =
            input
                .cost_metrics
                .finalize(refs.len(), materialized_records, omitted_by_budget);
        let slice = evidence_slice(
            input.authority_generation,
            refs,
            family_summaries,
            materialized_detail,
            omission,
        );

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

fn materialized_record_count(detail: &UiEvidenceMaterializedDetail) -> usize {
    match detail {
        UiEvidenceMaterializedDetail::Obligation(receipt) => receipt.projections().len(),
    }
}
