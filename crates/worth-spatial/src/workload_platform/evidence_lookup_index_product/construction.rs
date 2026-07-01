use super::error::{EvidenceLookupIndexProductError, EvidenceLookupIndexProductErrorKind};
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;
use super::product::EvidenceLookupIndexProduct;
use super::{
    counters::EvidenceLookupIndexProductCounters, query_support::query_support_row_count,
    topology_support::topology_receipt_ref_count,
};
use crate::workload_platform::evidence_ledger::{SelectedLookupSliceLedger, WorkloadEvidenceRow};
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupSelectedPlan, EvidenceLookupSelectedStrategyKind,
};
use crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupProductQuerySurfaceContractRow;

pub fn require_persistent_evidence_lookup_index_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> {
    Err(EvidenceLookupIndexProductError::new(
        EvidenceLookupIndexProductErrorKind::PersistentCapabilitySupportRequired,
        "persistent or restart-stable index posture requires admitted support before execution",
    )
    .with_required_lifecycle_posture(
        EvidenceLookupIndexLifecyclePosture::persistent_capability_required(),
    )
    .with_counters(selected_lookup_counters(selected_plan, ledger)))
}

pub(crate) fn selected_plan_query_surface_contract_rows(
    selected_plan: &EvidenceLookupSelectedPlan,
) -> Vec<EvidenceLookupProductQuerySurfaceContractRow> {
    selected_plan
        .rows()
        .iter()
        .filter_map(|row| {
            row.query_surface_contract().cloned().map(|contract| {
                EvidenceLookupProductQuerySurfaceContractRow::new(
                    row.family_identity().to_string(),
                    contract,
                )
            })
        })
        .collect()
}

pub(crate) fn lifecycle_posture(
    selected_plan: &EvidenceLookupSelectedPlan,
) -> EvidenceLookupIndexLifecyclePosture {
    let has_dense = selected_plan.rows().iter().any(|row| {
        row.strategy().is_some_and(|strategy| {
            strategy.kind() == EvidenceLookupSelectedStrategyKind::BoundedDenseIndexedLookupPlan
        })
    });
    if has_dense {
        return EvidenceLookupIndexLifecyclePosture::bounded_dense_construction();
    }
    let has_sparse = selected_plan.rows().iter().any(|row| {
        row.strategy().is_some_and(|strategy| {
            strategy.kind() == EvidenceLookupSelectedStrategyKind::SparseIndexedLookupPlan
        })
    });
    if has_sparse {
        return EvidenceLookupIndexLifecyclePosture::sparse_lookup_only();
    }
    EvidenceLookupIndexLifecyclePosture::declaration_only_no_index()
}

pub(crate) fn selected_lookup_counters(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
) -> EvidenceLookupIndexProductCounters {
    let rows = ledger.complete_ledger().rows();
    EvidenceLookupIndexProductCounters::new(
        rows.len(),
        rows.len(),
        indexed_family_count(selected_plan),
        topology_receipt_ref_count(selected_plan.rows()),
        query_support_row_count(selected_plan.rows()),
        resident_byte_count(rows),
    )
}

pub(crate) fn indexed_family_count(selected_plan: &EvidenceLookupSelectedPlan) -> usize {
    selected_plan
        .rows()
        .iter()
        .filter(|row| {
            row.strategy()
                .is_some_and(|strategy| strategy.is_indexed_lookup_plan())
        })
        .count()
}

pub(crate) fn resident_byte_count(rows: &[WorkloadEvidenceRow]) -> usize {
    rows.iter()
        .map(|row| {
            std::mem::size_of::<WorkloadEvidenceRow>()
                + row.evidence_identity().len()
                + row
                    .upstream_stage_binding()
                    .map_or(0, |binding| binding.upstream_evidence_identity().len())
        })
        .sum()
}
