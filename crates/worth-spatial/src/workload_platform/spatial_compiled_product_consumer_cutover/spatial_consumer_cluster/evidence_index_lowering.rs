use crate::workload_platform::evidence_ledger::SelectedLookupSliceLedger;
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_index_product::{
    admit_and_lower_index_family_identity, lifecycle_posture, selected_lookup_counters,
    selected_plan_query_surface_contract_rows, EvidenceLookupIndexDisposalPosture,
    EvidenceLookupIndexProduct, EvidenceLookupIndexProductError,
    EvidenceLookupIndexReuseResolution,
};
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::evidence_lookup_reuse_decision::{
    decide_evidence_lookup_index_reuse, execute_evidence_lookup_index_reuse,
    EvidenceLookupIndexReuseExecutionInput,
};
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialLookupConsumerRouteDenialKind {
    LookupExecutionReceiptMismatch,
    SelectedPlanMismatch,
    SelectedEquivalenceFamilyMismatch,
    SelectedReuseBasisMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialLookupConsumerRouteDenial {
    kind: SpatialLookupConsumerRouteDenialKind,
    detail: &'static str,
}

impl SpatialLookupConsumerRouteDenial {
    const fn new(kind: SpatialLookupConsumerRouteDenialKind, detail: &'static str) -> Self {
        Self { kind, detail }
    }

    pub const fn kind(&self) -> SpatialLookupConsumerRouteDenialKind {
        self.kind
    }

    pub const fn detail(&self) -> &'static str {
        self.detail
    }
}

pub fn lower_evidence_lookup_index_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
) -> Result<EvidenceLookupIndexProduct, EvidenceLookupIndexProductError> {
    let admitted_family = admit_and_lower_index_family_identity(selected_plan, ledger)?;
    let counters = selected_lookup_counters(selected_plan, ledger);
    Ok(EvidenceLookupIndexProduct::new(
        admitted_family.lowered_identity(),
        admitted_family.selected_equivalence_family(),
        selected_plan.selected_plan_digest().to_string(),
        selected_plan.spatial_touch_digest().to_string(),
        selected_plan.stage_receipt_digest().to_string(),
        admitted_family.evidence_ledger_basis_digest().to_string(),
        admitted_family.topology_support_digest().to_string(),
        admitted_family.query_support_digest().to_string(),
        None,
        selected_plan_query_surface_contract_rows(selected_plan),
        lifecycle_posture(selected_plan),
        EvidenceLookupIndexDisposalPosture::destroy_and_rebuild_required(),
        counters,
        ledger.complete_ledger().rows().to_vec(),
    ))
}

pub fn reuse_evidence_lookup_index_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
    prior_product: &EvidenceLookupIndexProduct,
) -> Result<EvidenceLookupIndexReuseResolution, EvidenceLookupIndexProductError> {
    let current_input = EvidenceLookupIndexReuseExecutionInput::lower(selected_plan, ledger)?;
    let decision = decide_evidence_lookup_index_reuse(&current_input, prior_product);
    execute_evidence_lookup_index_reuse(decision, &current_input, prior_product)
}

pub fn admit_lookup_execution_handoff_match(
    handoff: &EvidenceLookupConsumedWorkloadHandoff,
    execution_receipt: &EvidenceLookupExecutionReceipt,
) -> Result<(), SpatialLookupConsumerRouteDenial> {
    if execution_receipt.execution_receipt_digest() != handoff.lookup_execution_receipt_digest() {
        return Err(SpatialLookupConsumerRouteDenial::new(
            SpatialLookupConsumerRouteDenialKind::LookupExecutionReceiptMismatch,
            "spatial conflict input requires one matching lookup execution receipt for the admitted lookup handoff",
        ));
    }
    require_selected_family_contract(
        handoff.selected_lookup_plan_digest(),
        handoff.selected_equivalence_family_identity(),
        handoff.selected_reuse_basis_identity_digest(),
        execution_receipt.selected_plan_digest(),
        execution_receipt.selected_equivalence_family_identity(),
        execution_receipt.selected_reuse_basis_identity_digest(),
    )
}

pub fn admit_lookup_product_handoff_match(
    handoff: &EvidenceLookupConsumedWorkloadHandoff,
    product: &EvidenceLookupIndexProduct,
) -> Result<(), SpatialLookupConsumerRouteDenial> {
    require_selected_family_contract(
        handoff.selected_lookup_plan_digest(),
        handoff.selected_equivalence_family_identity(),
        handoff.selected_reuse_basis_identity_digest(),
        product.selected_plan_digest(),
        product.selected_equivalence_family_identity().as_str(),
        product.selected_reuse_basis_identity_digest(),
    )
}

fn require_selected_family_contract(
    admitted_plan_digest: &str,
    admitted_selected_family_identity: &str,
    admitted_reuse_basis_identity_digest: &str,
    candidate_plan_digest: &str,
    candidate_selected_family_identity: &str,
    candidate_reuse_basis_identity_digest: &str,
) -> Result<(), SpatialLookupConsumerRouteDenial> {
    if candidate_plan_digest != admitted_plan_digest {
        return Err(SpatialLookupConsumerRouteDenial::new(
            SpatialLookupConsumerRouteDenialKind::SelectedPlanMismatch,
            "spatial consumer cutover requires compiled-product proof whose selected plan agrees with the admitted lookup handoff",
        ));
    }
    if candidate_selected_family_identity != admitted_selected_family_identity {
        return Err(SpatialLookupConsumerRouteDenial::new(
            SpatialLookupConsumerRouteDenialKind::SelectedEquivalenceFamilyMismatch,
            "spatial consumer cutover requires compiled-product proof whose selected equivalence family agrees with the admitted lookup handoff",
        ));
    }
    if candidate_reuse_basis_identity_digest != admitted_reuse_basis_identity_digest {
        return Err(SpatialLookupConsumerRouteDenial::new(
            SpatialLookupConsumerRouteDenialKind::SelectedReuseBasisMismatch,
            "spatial consumer cutover requires compiled-product proof whose selected reuse basis agrees with the admitted lookup handoff",
        ));
    }
    Ok(())
}
