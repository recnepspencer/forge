use crate::runtime::frame_activation_gate::query_blockers::denied_query_rebind_count;
use crate::runtime::frame_activation_gate::query_rebind_basis::query_rebind_basis_digest;
use crate::runtime::frame_activation_gate::reconciliation_basis::reconciliation_basis_digest;
use crate::runtime::plan_equivalence::WorthUiExecutionPlanDigestor;
use crate::runtime::{
    WorthUiActivationGateCounters, WorthUiActivationGateDenial, WorthUiActivationGateDenialReason,
    WorthUiExecutionPlan, WorthUiExecutionPlanDigest, WorthUiExecutionPlanInput,
    WorthUiLaneParityReport, WorthUiPendingActivation, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeHandleAllocation,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiReadyActivation {
    pending_activation: WorthUiPendingActivation,
    candidate_execution_plan_digest: WorthUiExecutionPlanDigest,
    handle_allocation_basis_digest: u64,
    node_classification_count: usize,
    lane_changed_node_count: usize,
    reconciliation_basis_digest: u64,
    query_rebind_basis_digest: u64,
    query_rebind_denied_count: usize,
    lane_parity_semantic_reference_digest: Option<u64>,
    counters: WorthUiActivationGateCounters,
}

impl WorthUiReadyActivation {
    pub(crate) fn prepare(
        pending_activation: WorthUiPendingActivation,
        plan_input: &WorthUiExecutionPlanInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        candidate_plan: &WorthUiExecutionPlan,
        lane_parity_report: Option<&WorthUiLaneParityReport>,
    ) -> Result<Self, WorthUiActivationGateDenial> {
        let mut counters = WorthUiActivationGateCounters::default();
        reject_not_ready(&pending_activation, &mut counters)?;
        reject_plan_input_mismatch(&pending_activation, plan_input, &mut counters)?;
        reject_handle_receipt_mismatch(plan_input, handle_allocation, &mut counters)?;
        reject_execution_plan_receipt_mismatch(handle_allocation, candidate_plan, &mut counters)?;
        let query_rebind_denied_count = reject_query_blockers(&pending_activation, &mut counters)?;
        let lane_digest = lane_parity_digest(
            &pending_activation,
            candidate_plan,
            lane_parity_report,
            &mut counters,
        )?;
        let candidate_execution_plan_digest =
            WorthUiExecutionPlanDigestor::digest(candidate_plan).0;
        let (
            node_classification_count,
            lane_changed_node_count,
            reconciliation_basis_digest,
            query_rebind_basis_digest,
        ) = {
            let staged = pending_activation.staged_replacement();
            let node_plan_counters = staged.node_plan().counters();
            (
                staged.node_plan().classifications().len(),
                node_plan_counters.lane_changed_node_count(),
                reconciliation_basis_digest(staged.reconciliation_plan()),
                query_rebind_basis_digest(staged.query_rebind_plan()),
            )
        };
        Ok(Self {
            pending_activation,
            candidate_execution_plan_digest,
            handle_allocation_basis_digest: handle_allocation.receipt().basis_digest(),
            node_classification_count,
            lane_changed_node_count,
            reconciliation_basis_digest,
            query_rebind_basis_digest,
            query_rebind_denied_count,
            lane_parity_semantic_reference_digest: lane_digest,
            counters,
        })
    }

    pub fn readiness_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.pending_activation.frame_epoch()
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.pending_activation
            .staged_replacement()
            .active_artifact_digest()
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.pending_activation
            .staged_replacement()
            .candidate_artifact_digest()
    }

    pub fn candidate_execution_plan_digest(&self) -> u64 {
        self.candidate_execution_plan_digest.raw()
    }

    pub fn handle_allocation_basis_digest(&self) -> u64 {
        self.handle_allocation_basis_digest
    }

    pub fn node_classification_count(&self) -> usize {
        self.node_classification_count
    }

    pub fn lane_changed_node_count(&self) -> usize {
        self.lane_changed_node_count
    }

    pub fn reconciliation_basis_digest(&self) -> u64 {
        self.reconciliation_basis_digest
    }

    pub fn reconciliation_receipt_count(&self) -> usize {
        self.pending_activation
            .staged_replacement()
            .reconciliation_plan()
            .receipts()
            .len()
    }

    pub fn query_rebind_entry_count(&self) -> usize {
        self.pending_activation
            .staged_replacement()
            .query_rebind_plan()
            .entries()
            .len()
    }

    pub fn query_rebind_basis_digest(&self) -> u64 {
        self.query_rebind_basis_digest
    }

    pub fn query_rebind_denied_count(&self) -> usize {
        self.query_rebind_denied_count
    }

    pub fn lane_parity_semantic_reference_digest(&self) -> Option<u64> {
        self.lane_parity_semantic_reference_digest
    }

    pub fn counters(&self) -> WorthUiActivationGateCounters {
        self.counters
    }

    pub(crate) fn pending_activation(&self) -> &WorthUiPendingActivation {
        &self.pending_activation
    }
}

fn reject_not_ready(
    pending: &WorthUiPendingActivation,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    counters.record_readiness_check();
    if pending.readiness().is_ready_for_execution_plan_input() {
        Ok(())
    } else {
        Err(denial(
            pending,
            WorthUiActivationGateDenialReason::PendingActivationNotReady,
            *counters,
        ))
    }
}

fn reject_plan_input_mismatch(
    pending: &WorthUiPendingActivation,
    plan_input: &WorthUiExecutionPlanInput,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    let staged = pending.staged_replacement();
    counters.record_digest_check();
    let basis = plan_input.basis();
    let matches_basis = basis.active_artifact_digest() == staged.active_artifact_digest()
        && basis.candidate_artifact_digest() == staged.candidate_artifact_digest()
        && basis.frame_epoch() == pending.frame_epoch()
        && basis.staged_node_classification_count() == staged.node_plan().classifications().len()
        && basis.staged_reconciliation_receipt_count()
            == staged.reconciliation_plan().receipts().len()
        && basis.staged_query_rebind_entry_count() == staged.query_rebind_plan().entries().len();
    if matches_basis {
        Ok(())
    } else {
        Err(denial(
            pending,
            WorthUiActivationGateDenialReason::PendingAndPlanInputMismatch,
            *counters,
        ))
    }
}

fn reject_handle_receipt_mismatch(
    plan_input: &WorthUiExecutionPlanInput,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    counters.record_digest_check();
    if handle_allocation
        .receipt()
        .certifies_basis(handle_allocation.basis())
    {
        Ok(())
    } else {
        Err(denial_from_basis(
            plan_input.basis(),
            WorthUiActivationGateDenialReason::HandleAllocationReceiptMismatch,
            *counters,
        ))
    }
}

fn reject_execution_plan_receipt_mismatch(
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    candidate_plan: &WorthUiExecutionPlan,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    counters.record_digest_check();
    if candidate_plan.handle_receipt() == handle_allocation.receipt() {
        Ok(())
    } else {
        Err(WorthUiActivationGateDenial::new(
            handle_allocation.basis().active_artifact_digest(),
            handle_allocation.basis().candidate_artifact_digest(),
            handle_allocation.basis().frame_epoch(),
            handle_allocation.basis().frame_epoch(),
            WorthUiActivationGateDenialReason::ExecutionPlanHandleReceiptMismatch,
            denied(*counters),
        ))
    }
}

fn reject_query_blockers(
    pending: &WorthUiPendingActivation,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<usize, WorthUiActivationGateDenial> {
    let plan = pending.staged_replacement().query_rebind_plan();
    counters.record_query_rebind_entry_checks(plan.entries().len());
    let denied_count = denied_query_rebind_count(plan);
    if denied_count == 0 {
        Ok(denied_count)
    } else {
        Err(denial(
            pending,
            WorthUiActivationGateDenialReason::QueryRebindDenied,
            *counters,
        ))
    }
}

fn lane_parity_digest(
    pending: &WorthUiPendingActivation,
    candidate_plan: &WorthUiExecutionPlan,
    report: Option<&WorthUiLaneParityReport>,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<Option<u64>, WorthUiActivationGateDenial> {
    let requires_parity = pending
        .staged_replacement()
        .node_plan()
        .counters()
        .lane_changed_node_count()
        > 0;
    if !requires_parity {
        return Ok(None);
    }
    counters.record_lane_parity_check();
    let Some(report) = report else {
        return Err(denial(
            pending,
            WorthUiActivationGateDenialReason::MissingLaneParityReport,
            *counters,
        ));
    };
    if !report.certifies_activation() {
        return Err(denial(
            pending,
            WorthUiActivationGateDenialReason::LaneParityDoesNotCertifyActivation,
            *counters,
        ));
    }
    let certification = report.certification();
    let candidate_digest = WorthUiExecutionPlanDigestor::digest(candidate_plan).0.raw();
    let staged = pending.staged_replacement();
    let matches_report = certification.active_artifact_digest() == staged.active_artifact_digest()
        && certification.candidate_artifact_digest() == staged.candidate_artifact_digest()
        && certification.candidate_plan_digest() == candidate_digest;
    if matches_report {
        Ok(Some(certification.semantic_reference_digest()))
    } else {
        Err(denial(
            pending,
            WorthUiActivationGateDenialReason::LaneParityDigestMismatch,
            *counters,
        ))
    }
}

fn denial(
    pending: &WorthUiPendingActivation,
    reason: WorthUiActivationGateDenialReason,
    counters: WorthUiActivationGateCounters,
) -> WorthUiActivationGateDenial {
    WorthUiActivationGateDenial::new(
        pending.staged_replacement().active_artifact_digest(),
        pending.staged_replacement().candidate_artifact_digest(),
        pending.frame_epoch(),
        pending.frame_epoch(),
        reason,
        denied(counters),
    )
}

fn denial_from_basis(
    basis: &crate::runtime::WorthUiPlanLoweringBasis,
    reason: WorthUiActivationGateDenialReason,
    counters: WorthUiActivationGateCounters,
) -> WorthUiActivationGateDenial {
    WorthUiActivationGateDenial::new(
        basis.active_artifact_digest(),
        basis.candidate_artifact_digest(),
        basis.frame_epoch(),
        basis.frame_epoch(),
        reason,
        denied(counters),
    )
}

fn denied(mut counters: WorthUiActivationGateCounters) -> WorthUiActivationGateCounters {
    counters.record_denial();
    counters
}
