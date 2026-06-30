use crate::workload_platform::evidence_ledger::CompleteWorkloadEvidenceLedger;
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupSelectedPlan, EvidenceLookupSelectedStrategyKind,
};

use super::basis::EvidenceLookupLedgerBasis;
use super::disposal_posture::EvidenceLookupIndexDisposalPosture;
use super::error::{EvidenceLookupIndexProductError, EvidenceLookupIndexProductErrorKind};
use super::lifecycle_posture::EvidenceLookupIndexLifecyclePosture;
use super::product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupProductQuerySurfaceContractRow;

pub fn admit_evidence_lookup_index_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> {
    let basis = EvidenceLookupLedgerBasis::from_selected_plan(selected_plan, ledger);
    admit_with_basis(selected_plan, basis)
}

pub fn reuse_evidence_lookup_index_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &CompleteWorkloadEvidenceLedger,
    prior_product: &EvidenceLookupIndexProduct,
) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> {
    let basis = EvidenceLookupLedgerBasis::from_selected_plan(selected_plan, ledger);
    let counters = basis.counters(selected_plan);
    if prior_product.selected_plan_digest() != selected_plan.selected_plan_digest()
        || prior_product.spatial_touch_digest() != selected_plan.spatial_touch_digest()
        || prior_product.stage_receipt_digest() != selected_plan.stage_receipt_digest()
        || prior_product.evidence_ledger_basis_digest() != basis.basis_digest()
    {
        return Err(EvidenceLookupIndexProductError::new(
            EvidenceLookupIndexProductErrorKind::ReusedIndexBasisMismatch,
            "index reuse requires matching selected plan, spatial touch, stage receipt, and evidence ledger basis digests",
        )
        .with_counters(counters));
    }

    Ok(EvidenceLookupIndexProduct::new(
        selected_plan.selected_plan_digest().to_string(),
        selected_plan.spatial_touch_digest().to_string(),
        selected_plan.stage_receipt_digest().to_string(),
        basis.basis_digest().to_string(),
        basis.topology_support_digest().to_string(),
        basis.query_support_digest().to_string(),
        selected_plan_query_surface_contract_rows(selected_plan),
        EvidenceLookupIndexLifecyclePosture::equivalent_reuse(),
        EvidenceLookupIndexDisposalPosture::destroy_and_rebuild_required(),
        prior_product.counters().reused_from(),
        prior_product.rows().to_vec(),
    ))
}

pub fn require_persistent_evidence_lookup_index_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> {
    let basis = EvidenceLookupLedgerBasis::from_selected_plan(selected_plan, ledger);
    Err(EvidenceLookupIndexProductError::new(
        EvidenceLookupIndexProductErrorKind::PersistentCapabilitySupportRequired,
        "persistent or restart-stable index posture requires admitted support before execution",
    )
    .with_required_lifecycle_posture(
        EvidenceLookupIndexLifecyclePosture::persistent_capability_required(),
    )
    .with_counters(basis.counters(selected_plan)))
}

pub(crate) fn admit_with_basis(
    selected_plan: &EvidenceLookupSelectedPlan,
    basis: EvidenceLookupLedgerBasis,
) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> {
    let counters = basis.counters(selected_plan);
    if basis
        .rows()
        .iter()
        .all(|row| row.stage() != selected_plan.stage())
    {
        return Err(EvidenceLookupIndexProductError::new(
            EvidenceLookupIndexProductErrorKind::MissingSelectedStageLedgerRow,
            selected_plan.stage().human_name(),
        )
        .with_counters(counters));
    }
    if basis.exceeds_selected_scope() {
        return Err(EvidenceLookupIndexProductError::new(
            EvidenceLookupIndexProductErrorKind::LedgerBasisExceedsSelectedScope,
            "selected lookup index basis cannot include unrelated ledger rows beyond the selected lookup plan",
        )
        .with_counters(counters));
    }

    Ok(EvidenceLookupIndexProduct::new(
        selected_plan.selected_plan_digest().to_string(),
        selected_plan.spatial_touch_digest().to_string(),
        selected_plan.stage_receipt_digest().to_string(),
        basis.basis_digest().to_string(),
        basis.topology_support_digest().to_string(),
        basis.query_support_digest().to_string(),
        selected_plan_query_surface_contract_rows(selected_plan),
        lifecycle_posture(selected_plan),
        EvidenceLookupIndexDisposalPosture::destroy_and_rebuild_required(),
        counters,
        basis.rows().to_vec(),
    ))
}

fn selected_plan_query_surface_contract_rows(
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

fn lifecycle_posture(
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
