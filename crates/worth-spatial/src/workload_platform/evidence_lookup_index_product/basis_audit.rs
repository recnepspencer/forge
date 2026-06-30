use crate::workload_platform::evidence_ledger::CompleteWorkloadEvidenceLedger;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;

use super::basis::EvidenceLookupLedgerBasis;
use super::construction::{
    indexed_family_count, lifecycle_posture, resident_byte_count,
    selected_plan_query_surface_contract_rows,
};
use super::error::{EvidenceLookupIndexProductError, EvidenceLookupIndexProductErrorKind};
use super::identity::lower_index_family_identity_from_basis;
use super::product::EvidenceLookupIndexProduct;
use super::topology_support::topology_receipt_ref_count;
use super::{counters::EvidenceLookupIndexProductCounters, query_support::query_support_row_count};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupIndexBasisAuditScope {
    SelectedPlanBounded,
    CompleteLedgerUnbounded,
}

pub(crate) fn audit_evidence_lookup_index_product_basis(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &CompleteWorkloadEvidenceLedger,
    scope: EvidenceLookupIndexBasisAuditScope,
) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> {
    let basis = match scope {
        EvidenceLookupIndexBasisAuditScope::SelectedPlanBounded => {
            EvidenceLookupLedgerBasis::from_selected_plan(selected_plan, ledger)
        }
        EvidenceLookupIndexBasisAuditScope::CompleteLedgerUnbounded => {
            EvidenceLookupLedgerBasis::from_complete_ledger_scope(selected_plan, ledger)
        }
    };
    let counters = EvidenceLookupIndexProductCounters::new(
        basis.rows().len(),
        ledger.counters().rows(),
        indexed_family_count(selected_plan),
        topology_receipt_ref_count(selected_plan.rows()),
        query_support_row_count(selected_plan.rows()),
        resident_byte_count(basis.rows()),
    );
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

    let lowered_identity = lower_index_family_identity_from_basis(selected_plan, &basis);
    Ok(EvidenceLookupIndexProduct::new(
        &lowered_identity,
        selected_plan.selected_plan_digest().to_string(),
        selected_plan.spatial_touch_digest().to_string(),
        selected_plan.stage_receipt_digest().to_string(),
        basis.basis_digest().to_string(),
        basis.topology_support_digest().to_string(),
        basis.query_support_digest().to_string(),
        None,
        selected_plan_query_surface_contract_rows(selected_plan),
        lifecycle_posture(selected_plan),
        super::disposal_posture::EvidenceLookupIndexDisposalPosture::destroy_and_rebuild_required(),
        counters,
        basis.rows().to_vec(),
    ))
}
