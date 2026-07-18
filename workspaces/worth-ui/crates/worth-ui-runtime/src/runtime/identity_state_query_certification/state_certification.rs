use crate::runtime::identity_state_query_certification::scenario::WorthUiStateCertificationScenarioStepReconciliation;
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiDurableStateReconciliationOutcome,
    WorthUiIdentityStateQueryCertificationCounters, WorthUiIdentityStateQueryCertificationDenial,
    WorthUiIdentityStateQueryCertificationDenialReason, WorthUiStateCarryForwardReceipt,
    WorthUiStateLifecycleReceipt,
};

pub(crate) fn certify_state_step_receipts(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    active_observation: &WorthUiActiveRuntimeObservation,
    counters: &mut WorthUiIdentityStateQueryCertificationCounters,
    state_receipts: &mut Vec<WorthUiStateLifecycleReceipt>,
    carry_forward_receipts: &mut Vec<WorthUiStateCarryForwardReceipt>,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    reject_state_step_active_runtime_mismatch(step, active_observation, *counters)?;
    match step.reconciliation() {
        WorthUiStateCertificationScenarioStepReconciliation::Denial(denial_reason) => {
            certify_state_reconciliation_denial(step, denial_reason, counters)
        }
        WorthUiStateCertificationScenarioStepReconciliation::Plan(plan) => {
            reject_state_reconciliation_digest_mismatch(step, plan, *counters)?;
            certify_state_reconciliation_plan_receipts(
                step,
                plan,
                counters,
                state_receipts,
                carry_forward_receipts,
            )
        }
    }
}

fn certify_state_reconciliation_denial(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    denial_reason: &crate::runtime::WorthUiDurableStateReconciliationDenial,
    counters: &mut WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if !step.node_plan().is_unambiguous() {
        counters.record_ambiguous_identity_denial();
        return Ok(());
    }
    Err(denial(
        WorthUiIdentityStateQueryCertificationDenialReason::StateReconciliationDenied {
            label: step.label().to_owned(),
            denial: denial_reason.clone(),
        },
        *counters,
    ))
}

fn certify_state_reconciliation_plan_receipts(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    plan: &crate::runtime::WorthUiDurableStateReconciliationPlan,
    counters: &mut WorthUiIdentityStateQueryCertificationCounters,
    state_receipts: &mut Vec<WorthUiStateLifecycleReceipt>,
    carry_forward_receipts: &mut Vec<WorthUiStateCarryForwardReceipt>,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    for receipt in plan.receipts() {
        reject_ambiguous_identity_carry_forward(step, receipt, *counters)?;
        let lifecycle = certify_state_receipt_transition_matches_plan(step, receipt, *counters)?;
        counters.record_state_receipt(receipt.outcome());
        if receipt.outcome() == WorthUiDurableStateReconciliationOutcome::CarryForward {
            carry_forward_receipts.push(WorthUiStateCarryForwardReceipt::from_source(
                receipt.clone(),
            ));
        }
        state_receipts.push(lifecycle);
    }
    Ok(())
}

fn reject_state_step_active_runtime_mismatch(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    active_observation: &WorthUiActiveRuntimeObservation,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if step.node_plan().active_artifact_digest() == active_observation.artifact_digest() {
        return Ok(());
    }
    Err(denial(
        WorthUiIdentityStateQueryCertificationDenialReason::StatePlanActiveRuntimeMismatch {
            label: step.label().to_owned(),
            active_runtime_artifact_digest: active_observation.artifact_digest(),
            plan_active_artifact_digest: step.node_plan().active_artifact_digest(),
        },
        counters,
    ))
}

fn reject_state_reconciliation_digest_mismatch(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    plan: &crate::runtime::WorthUiDurableStateReconciliationPlan,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if step.node_plan().active_artifact_digest() == plan.active_artifact_digest()
        && step.node_plan().candidate_artifact_digest() == plan.candidate_artifact_digest()
    {
        return Ok(());
    }
    Err(denial(
        WorthUiIdentityStateQueryCertificationDenialReason::StatePlanDigestMismatch {
            plan_active_artifact_digest: step.node_plan().active_artifact_digest(),
            reconciliation_active_artifact_digest: plan.active_artifact_digest(),
            plan_candidate_artifact_digest: step.node_plan().candidate_artifact_digest(),
            reconciliation_candidate_artifact_digest: plan.candidate_artifact_digest(),
        },
        counters,
    ))
}

fn reject_ambiguous_identity_carry_forward(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    receipt: &crate::runtime::WorthUiDurableStateReconciliationReceipt,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<(), WorthUiIdentityStateQueryCertificationDenial> {
    if step.node_plan().is_unambiguous()
        || receipt.outcome() != WorthUiDurableStateReconciliationOutcome::CarryForward
    {
        return Ok(());
    }
    Err(denial(
        WorthUiIdentityStateQueryCertificationDenialReason::AmbiguousIdentityPreservedDurableState {
            label: step.label().to_owned(),
            identity_basis: receipt.identity_basis().to_owned(),
            family_id: receipt.family_id().clone(),
        },
        counters,
    ))
}

fn certify_state_receipt_transition_matches_plan(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    receipt: &crate::runtime::WorthUiDurableStateReconciliationReceipt,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> Result<WorthUiStateLifecycleReceipt, WorthUiIdentityStateQueryCertificationDenial> {
    let Some(transition) = step
        .node_plan()
        .transition_for_identity(receipt.identity_basis())
    else {
        return Err(state_receipt_transition_mismatch(step, receipt, counters));
    };
    let lifecycle = WorthUiStateLifecycleReceipt::from_source(receipt.clone());
    if lifecycle.transition() != transition {
        return Err(state_receipt_transition_mismatch(step, receipt, counters));
    }
    Ok(lifecycle)
}

fn state_receipt_transition_mismatch(
    step: &crate::runtime::WorthUiStateCertificationScenarioStep,
    receipt: &crate::runtime::WorthUiDurableStateReconciliationReceipt,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> WorthUiIdentityStateQueryCertificationDenial {
    denial(
        WorthUiIdentityStateQueryCertificationDenialReason::StateReceiptTransitionMismatch {
            label: step.label().to_owned(),
            identity_basis: receipt.identity_basis().to_owned(),
            outcome: receipt.outcome(),
        },
        counters,
    )
}

fn denial(
    reason: WorthUiIdentityStateQueryCertificationDenialReason,
    counters: WorthUiIdentityStateQueryCertificationCounters,
) -> WorthUiIdentityStateQueryCertificationDenial {
    WorthUiIdentityStateQueryCertificationDenial::new(reason, counters)
}
