use crate::runtime::{
    WorthUiActivationGateCounters, WorthUiActivationGateDenial, WorthUiActivationGateDenialReason,
    WorthUiExecutionPlan, WorthUiExecutionPlanDigest, WorthUiExecutionPlanInput,
    WorthUiLaneParityReport, WorthUiPendingActivation, WorthUiRuntimeHandleAllocation,
};

pub(super) struct UiActivationInputValidationFacts {
    pub(super) candidate_execution_plan_digest: WorthUiExecutionPlanDigest,
    pub(super) handle_allocation_basis_digest: u64,
    pub(super) node_classification_count: usize,
    pub(super) lane_changed_node_count: usize,
    pub(super) reconciliation_basis_digest: u64,
    pub(super) query_rebind_basis_digest: u64,
    pub(super) query_rebind_denied_count: usize,
    pub(super) lane_parity_semantic_reference_digest: Option<u64>,
    pub(super) counters: WorthUiActivationGateCounters,
}

pub(super) fn validate_activation_inputs(
    pending: &WorthUiPendingActivation,
    plan_input: &WorthUiExecutionPlanInput,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    allocation_catalog: &crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
    candidate_bundle: &crate::runtime::active::WorthUiSealedExecutionPlanBundle,
    lane_parity_report: Option<&WorthUiLaneParityReport>,
) -> Result<UiActivationInputValidationFacts, WorthUiActivationGateDenial> {
    let mut counters = WorthUiActivationGateCounters::default();
    reject_not_ready(pending, &mut counters)?;
    reject_plan_input_mismatch(pending, plan_input, &mut counters)?;
    reject_handle_receipt_mismatch(
        plan_input,
        handle_allocation,
        allocation_catalog,
        &mut counters,
    )?;
    reject_execution_plan_receipt_mismatch(
        handle_allocation,
        candidate_bundle.execution_plan(),
        &mut counters,
    )?;
    let query_rebind_denied_count = reject_query_blockers(pending, &mut counters)?;
    let lane_parity_semantic_reference_digest = lane_parity_digest(
        pending,
        candidate_bundle.digest(),
        lane_parity_report,
        &mut counters,
    )?;
    let staged = pending.staged_replacement();
    let node_plan_counters = staged.node_plan().counters();
    Ok(UiActivationInputValidationFacts {
        candidate_execution_plan_digest: candidate_bundle.digest(),
        handle_allocation_basis_digest: handle_allocation.receipt().basis_digest(),
        node_classification_count: staged.node_plan().classifications().len(),
        lane_changed_node_count: node_plan_counters.lane_changed_node_count(),
        reconciliation_basis_digest: staged.reconciliation_plan().basis_digest(),
        query_rebind_basis_digest: staged.query_rebind_plan().basis_digest(),
        query_rebind_denied_count,
        lane_parity_semantic_reference_digest,
        counters,
    })
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
    let structural_node_count = staged.node_plan().candidate_structural_node_count();
    let query_binding_input_count = staged.query_rebind_plan().live_candidate_binding_count();
    let component_hook_input_count = plan_input.counters().component_hook_input_count();
    let matches_basis = basis.prior_artifact_digest() == Some(staged.active_artifact_digest())
        && basis.candidate_artifact_digest() == staged.candidate_artifact_digest()
        && basis.frame_epoch() == pending.frame_epoch()
        && basis.candidate_node_input_count()
            == structural_node_count + query_binding_input_count + component_hook_input_count
        && basis.reconciliation_receipt_count() == staged.reconciliation_plan().receipts().len()
        && basis.query_binding_input_count() == query_binding_input_count;
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
    allocation_catalog: &crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
    counters: &mut WorthUiActivationGateCounters,
) -> Result<(), WorthUiActivationGateDenial> {
    counters.record_digest_check();
    if handle_allocation
        .receipt()
        .certifies_basis(handle_allocation.basis())
        && allocation_catalog.certifies_activation_binding(
            handle_allocation
                .basis()
                .allocation_planning_identity_digest(),
        )
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
            handle_allocation
                .basis()
                .prior_artifact_digest()
                .unwrap_or_default(),
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
    let denied_count = plan.counters().denied_binding_count();
    counters.record_query_rebind_entry_checks(0);
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
    candidate_plan_digest: WorthUiExecutionPlanDigest,
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
    let staged = pending.staged_replacement();
    let matches_report = certification.active_artifact_digest() == staged.active_artifact_digest()
        && certification.candidate_artifact_digest() == staged.candidate_artifact_digest()
        && certification.candidate_plan_digest() == candidate_plan_digest.raw();
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
        basis.prior_artifact_digest().unwrap_or_default(),
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
