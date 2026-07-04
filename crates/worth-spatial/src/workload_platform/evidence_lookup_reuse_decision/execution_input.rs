use crate::spatial_compiled_product_family::SpatialCompiledProductLoweredIdentity;
use crate::workload_platform::evidence_ledger::{SelectedLookupSliceLedger, WorkloadEvidenceRow};
use crate::workload_platform::evidence_lookup_index_product::{
    admit_and_lower_index_family_identity, lifecycle_posture, selected_lookup_counters,
    selected_plan_query_surface_contract_rows, EvidenceLookupIndexDisposalPosture,
    EvidenceLookupIndexLifecyclePosture, EvidenceLookupIndexProductCounters,
};
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupProductQuerySurfaceContractRow;
use crate::workload_platform::selected_equivalence_family::SelectedSpatialEquivalenceFamily;

use super::super::evidence_lookup_index_product::EvidenceLookupIndexProductError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupIndexReuseExecutionInput {
    lowered_identity: SpatialCompiledProductLoweredIdentity,
    selected_equivalence_family: SelectedSpatialEquivalenceFamily,
    selected_plan_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    evidence_ledger_basis_digest: String,
    topology_support_digest: String,
    query_support_digest: String,
    query_surface_contract_rows: Vec<EvidenceLookupProductQuerySurfaceContractRow>,
    lifecycle_posture: EvidenceLookupIndexLifecyclePosture,
    disposal_posture: EvidenceLookupIndexDisposalPosture,
    counters: EvidenceLookupIndexProductCounters,
    raw_evidence_row_scan_count: usize,
    broad_receipt_scan_count: usize,
    caller_owned_evidence_work_count: usize,
    rows: Vec<WorkloadEvidenceRow>,
}

impl EvidenceLookupIndexReuseExecutionInput {
    pub(crate) fn lower(
        selected_plan: &EvidenceLookupSelectedPlan,
        ledger: &SelectedLookupSliceLedger,
    ) -> Result<Self, EvidenceLookupIndexProductError> {
        let admitted_family = admit_and_lower_index_family_identity(selected_plan, ledger)?;
        let counters = selected_lookup_counters(selected_plan, ledger);
        Ok(Self {
            lowered_identity: admitted_family.lowered_identity().clone(),
            selected_equivalence_family: admitted_family.selected_equivalence_family().clone(),
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            spatial_touch_digest: selected_plan.spatial_touch_digest().to_string(),
            stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
            evidence_ledger_basis_digest: admitted_family
                .evidence_ledger_basis_digest()
                .to_string(),
            topology_support_digest: admitted_family.topology_support_digest().to_string(),
            query_support_digest: admitted_family.query_support_digest().to_string(),
            query_surface_contract_rows: selected_plan_query_surface_contract_rows(selected_plan),
            lifecycle_posture: lifecycle_posture(selected_plan),
            disposal_posture: EvidenceLookupIndexDisposalPosture::destroy_and_rebuild_required(),
            counters,
            raw_evidence_row_scan_count: selected_plan.counters().raw_evidence_row_scan_count(),
            broad_receipt_scan_count: selected_plan.counters().broad_receipt_scan_count(),
            caller_owned_evidence_work_count: selected_plan
                .counters()
                .caller_owned_evidence_work_count(),
            rows: ledger.complete_ledger().rows().to_vec(),
        })
    }

    pub(crate) fn lowered_identity(&self) -> &SpatialCompiledProductLoweredIdentity {
        &self.lowered_identity
    }

    pub(crate) fn selected_equivalence_family(&self) -> &SelectedSpatialEquivalenceFamily {
        &self.selected_equivalence_family
    }

    pub(crate) fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub(crate) fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub(crate) fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub(crate) fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }

    pub(crate) fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }

    pub(crate) fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub(crate) fn query_surface_contract_rows(
        &self,
    ) -> &[EvidenceLookupProductQuerySurfaceContractRow] {
        &self.query_surface_contract_rows
    }

    pub(crate) const fn lifecycle_posture(&self) -> EvidenceLookupIndexLifecyclePosture {
        self.lifecycle_posture
    }

    pub(crate) const fn disposal_posture(&self) -> EvidenceLookupIndexDisposalPosture {
        self.disposal_posture
    }

    pub(crate) const fn counters(&self) -> &EvidenceLookupIndexProductCounters {
        &self.counters
    }

    pub(crate) const fn raw_evidence_row_scan_count(&self) -> usize {
        self.raw_evidence_row_scan_count
    }

    pub(crate) const fn broad_receipt_scan_count(&self) -> usize {
        self.broad_receipt_scan_count
    }

    pub(crate) const fn caller_owned_evidence_work_count(&self) -> usize {
        self.caller_owned_evidence_work_count
    }

    pub(crate) fn rows(&self) -> &[WorkloadEvidenceRow] {
        &self.rows
    }
}
