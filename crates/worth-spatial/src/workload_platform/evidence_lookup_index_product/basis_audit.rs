#![allow(dead_code)]

use crate::workload_platform::evidence_ledger::CompleteWorkloadEvidenceLedger;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;

use super::basis::EvidenceLookupLedgerBasis;
use super::construction::admit_with_basis;
use super::error::EvidenceLookupIndexProductError;
use super::product::EvidenceLookupIndexProduct;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupIndexBasisAuditScope {
    SelectedPlanBounded,
    CompleteLedgerUnbounded,
}

pub fn audit_evidence_lookup_index_product_basis(
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
    admit_with_basis(selected_plan, basis)
}
