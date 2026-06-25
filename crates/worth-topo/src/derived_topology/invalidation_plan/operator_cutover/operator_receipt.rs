use serde::Serialize;

use super::super::execution::DerivedInvalidationExecutionReceipt;
use super::super::migrated_products::CoveredDerivedProductMigrationSweepCloseout;
use super::super::selection::{DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure};
use super::counters::DerivedInvalidationOperatorCutoverCounters;
use super::error::{
    DerivedInvalidationOperatorCutoverError, DerivedInvalidationOperatorCutoverErrorKind,
};
use crate::topology_operators::application::TopologyMutationApplicationEvidence;
use crate::topology_operators::TopologyDeclaredTouchedGraphBasisProof;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationOperatorCutoverReceipt {
    phase_seven_seed_digest: String,
    operator_touched_basis_digest: String,
    selected_plan_digest: String,
    execution_receipt_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    graph_obligation_envelope_digest: String,
    graph_obligation_dispatch_digest: Option<String>,
    counters: DerivedInvalidationOperatorCutoverCounters,
    receipt_digest: String,
}

#[cfg_attr(not(test), allow(dead_code))]
impl DerivedInvalidationOperatorCutoverReceipt {
    pub(crate) fn bind_operator_cutover(
        phase_six_closeout: &CoveredDerivedProductMigrationSweepCloseout,
        selected_plan: &DerivedInvalidationSelectedPlan,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
        operator_touched_basis: &TopologyDeclaredTouchedGraphBasisProof,
        mutation_evidence: &TopologyMutationApplicationEvidence,
    ) -> Result<Self, DerivedInvalidationOperatorCutoverError> {
        require_matching_plan_proofs(phase_six_closeout, selected_plan, execution_receipt)?;
        require_complete_phase_six_ordinary_product_coverage(phase_six_closeout)?;
        require_matching_touched_authority(execution_receipt, operator_touched_basis)?;
        require_operator_graph_obligation_proof(mutation_evidence)?;
        require_ordinary_execution_without_old_authority(execution_receipt)?;

        let graph_obligation_envelope_digest = mutation_evidence
            .graph_obligation_envelope_digest()
            .expect("checked by require_operator_graph_obligation_proof")
            .to_string();
        let graph_obligation_dispatch_digest = mutation_evidence
            .graph_obligation_dispatch_digest()
            .map(str::to_string);
        let operator_touched_closure =
            DerivedInvalidationTouchedClosure::from_declared_touch(operator_touched_basis);
        let counters = DerivedInvalidationOperatorCutoverCounters::from_proofs(
            selected_plan,
            execution_receipt,
            phase_six_closeout,
            0,
        );
        let receipt_digest = operator_receipt_digest(
            phase_six_closeout.phase_seven_seed().seed_digest(),
            operator_touched_basis.basis_digest(),
            operator_touched_closure.closure_digest(),
            selected_plan,
            execution_receipt,
            &graph_obligation_envelope_digest,
            graph_obligation_dispatch_digest.as_deref(),
            &counters,
        );
        Ok(Self {
            phase_seven_seed_digest: phase_six_closeout
                .phase_seven_seed()
                .seed_digest()
                .to_string(),
            operator_touched_basis_digest: operator_touched_basis.basis_digest().to_string(),
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            execution_receipt_digest: execution_receipt.execution_receipt_digest().to_string(),
            touched_closure_digest: execution_receipt.touched_closure_digest().to_string(),
            query_support_digest: execution_receipt.query_support_digest().to_string(),
            legality_support_digest: execution_receipt.legality_support_digest().to_string(),
            graph_obligation_envelope_digest,
            graph_obligation_dispatch_digest,
            counters,
            receipt_digest,
        })
    }

    pub fn phase_seven_seed_digest(&self) -> &str {
        &self.phase_seven_seed_digest
    }

    pub fn operator_touched_basis_digest(&self) -> &str {
        &self.operator_touched_basis_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub fn graph_obligation_envelope_digest(&self) -> &str {
        &self.graph_obligation_envelope_digest
    }

    pub fn graph_obligation_dispatch_digest(&self) -> Option<&str> {
        self.graph_obligation_dispatch_digest.as_deref()
    }

    pub const fn counters(&self) -> &DerivedInvalidationOperatorCutoverCounters {
        &self.counters
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}

fn require_matching_plan_proofs(
    phase_six_closeout: &CoveredDerivedProductMigrationSweepCloseout,
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<(), DerivedInvalidationOperatorCutoverError> {
    if phase_six_closeout.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::PhaseSixSweepDoesNotMatchSelectedPlan,
            "phase six covered-product migration sweep must close the same selected invalidation plan consumed by operator cutover",
        ));
    }
    if execution_receipt.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::ExecutionReceiptDoesNotMatchSelectedPlan,
            "execution receipt must be minted from the selected invalidation plan consumed by operator cutover",
        ));
    }
    Ok(())
}

fn require_complete_phase_six_ordinary_product_coverage(
    phase_six_closeout: &CoveredDerivedProductMigrationSweepCloseout,
) -> Result<(), DerivedInvalidationOperatorCutoverError> {
    if phase_six_closeout
        .counters()
        .ordinary_consumable_family_count()
        != phase_six_closeout.counters().required_family_count()
    {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::PhaseSixSweepIncomplete,
            "operator cutover requires every covered ordinary derived product family to be migrated or deleted before Phase 7 can attach ordinary proof",
        ));
    }
    Ok(())
}

fn require_matching_touched_authority(
    execution_receipt: &DerivedInvalidationExecutionReceipt,
    operator_touched_basis: &TopologyDeclaredTouchedGraphBasisProof,
) -> Result<(), DerivedInvalidationOperatorCutoverError> {
    let operator_touched_closure =
        DerivedInvalidationTouchedClosure::from_declared_touch(operator_touched_basis);
    if execution_receipt.touched_closure_digest() != operator_touched_closure.closure_digest() {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::OperatorTouchedBasisDoesNotMatchExecutionReceipt,
            "operator touched-basis proof must be the touched closure carried by the invalidation execution receipt",
        ));
    }
    Ok(())
}

fn require_operator_graph_obligation_proof(
    mutation_evidence: &TopologyMutationApplicationEvidence,
) -> Result<(), DerivedInvalidationOperatorCutoverError> {
    if mutation_evidence
        .graph_obligation_envelope_digest()
        .is_none()
        || mutation_evidence.graph_obligation_selected_count() == 0
    {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::MissingOperatorGraphObligationProof,
            "operator cutover requires selected graph-obligation proof from the Query-authored mutation artifact",
        ));
    }
    Ok(())
}

fn require_ordinary_execution_without_old_authority(
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<(), DerivedInvalidationOperatorCutoverError> {
    if !execution_receipt.denied_rows().is_empty() {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::ExecutionReceiptCarriesDeniedProducts,
            "ordinary operator cutover cannot treat denied derived products as successful maintenance",
        ));
    }
    if execution_receipt.counters().whole_view_fallback_count() != 0 {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::ExecutionReceiptCarriesWholeViewFallback,
            "ordinary operator cutover cannot close on whole-view fallback execution",
        ));
    }
    if execution_receipt.counters().caller_owned_graph_work_count() != 0 {
        return Err(DerivedInvalidationOperatorCutoverError::new(
            DerivedInvalidationOperatorCutoverErrorKind::ExecutionReceiptCarriesCallerOwnedGraphWork,
            "ordinary operator cutover cannot close when execution externalized graph work to the caller",
        ));
    }
    Ok(())
}

fn operator_receipt_digest(
    phase_seven_seed_digest: &str,
    operator_touched_basis_digest: &str,
    operator_touched_closure_digest: &str,
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
    graph_obligation_envelope_digest: &str,
    graph_obligation_dispatch_digest: Option<&str>,
    counters: &DerivedInvalidationOperatorCutoverCounters,
) -> String {
    super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-operator-cutover-receipt:v1".to_string(),
        format!("phase-seven-seed:{phase_seven_seed_digest}"),
        format!("operator-touch:{operator_touched_basis_digest}"),
        format!("operator-touched-closure:{operator_touched_closure_digest}"),
        format!("selected-plan:{}", selected_plan.selected_plan_digest()),
        format!("execution:{}", execution_receipt.execution_receipt_digest()),
        format!(
            "touched-closure:{}",
            execution_receipt.touched_closure_digest()
        ),
        format!("query-support:{}", execution_receipt.query_support_digest()),
        format!(
            "legality-support:{}",
            execution_receipt.legality_support_digest()
        ),
        format!("graph-obligation-envelope:{graph_obligation_envelope_digest}"),
        format!(
            "graph-obligation-dispatch:{}",
            graph_obligation_dispatch_digest.unwrap_or("none")
        ),
        format!("counters:{}", counters.counters_digest()),
    ])
}
