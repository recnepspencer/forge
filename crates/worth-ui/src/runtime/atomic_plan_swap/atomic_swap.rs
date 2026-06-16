#[cfg(test)]
use super::WorthUiPlanSwapFailureInjection;
use super::{
    WorthUiPlanSwapReceiptParts, WorthUiPriorValidPlan, WorthUiReadyActivationSwapPayload,
};
use crate::runtime::active::WorthUiActiveRuntimeState;
use crate::runtime::frame_activation_gate::{
    WorthUiActivationGateReceipt, WorthUiFrameActivationGate,
};
use crate::runtime::plan_equivalence::WorthUiExecutionPlanDigestor;
use crate::runtime::{
    WorthUiAtomicPlanSwapCounters, WorthUiExecutionPlan, WorthUiFrameBoundary,
    WorthUiPlanSwapDenialReason, WorthUiPlanSwapReceipt, WorthUiPlanSwapRollback,
    WorthUiPriorValidPlanObservation, WorthUiReadyActivation, WorthUiRuntimeActivationStatus,
    WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
};

pub(crate) struct WorthUiAtomicPlanSwap;

impl WorthUiAtomicPlanSwap {
    pub(crate) fn swap(
        active: &mut WorthUiActiveRuntimeState,
        ready: WorthUiReadyActivation,
        candidate_plan: WorthUiExecutionPlan,
        boundary: WorthUiFrameBoundary,
        runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
    ) -> Result<WorthUiPlanSwapReceipt, WorthUiPlanSwapRollback> {
        Self::swap_inner(
            active,
            ready,
            candidate_plan,
            boundary,
            runtime_frame_epoch,
            WorthUiPlanSwapInjectionMode::None,
        )
    }

    #[cfg(test)]
    pub(crate) fn swap_with_injection(
        active: &mut WorthUiActiveRuntimeState,
        ready: WorthUiReadyActivation,
        candidate_plan: WorthUiExecutionPlan,
        boundary: WorthUiFrameBoundary,
        runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
        injection: WorthUiPlanSwapFailureInjection,
    ) -> Result<WorthUiPlanSwapReceipt, WorthUiPlanSwapRollback> {
        Self::swap_inner(
            active,
            ready,
            candidate_plan,
            boundary,
            runtime_frame_epoch,
            WorthUiPlanSwapInjectionMode::from(injection),
        )
    }

    fn swap_inner(
        active: &mut WorthUiActiveRuntimeState,
        ready: WorthUiReadyActivation,
        candidate_plan: WorthUiExecutionPlan,
        boundary: WorthUiFrameBoundary,
        runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
        _injection: WorthUiPlanSwapInjectionMode,
    ) -> Result<WorthUiPlanSwapReceipt, WorthUiPlanSwapRollback> {
        let mut counters = WorthUiAtomicPlanSwapCounters::new();
        let prior = capture_prior_valid_plan(active, &mut counters);
        let prior_observation = prior.observation();
        let gate_receipt = activate_swap_gate_or_rollback(
            active,
            &ready,
            boundary,
            runtime_frame_epoch,
            prior_observation,
            &mut counters,
        )?;
        let payload = build_swap_payload_or_rollback(
            active,
            ready,
            candidate_plan,
            prior_observation,
            &mut counters,
        )?;
        #[cfg(test)]
        if let Some(rollback) = injected_precommit_rollback(
            _injection,
            &payload,
            gate_receipt,
            prior_observation,
            counters,
        ) {
            return Err(rollback);
        }
        #[cfg(test)]
        if let Some(rollback) = injected_post_artifact_mutation_rollback(
            _injection,
            active,
            &payload,
            gate_receipt,
            &prior,
            prior_observation,
            &mut counters,
        ) {
            return Err(rollback);
        }
        let previous = active.observation();
        commit_swap_payload(active, payload, runtime_frame_epoch);
        counters.record_active_state_mutation();
        let next = active.observation();
        Ok(WorthUiPlanSwapReceipt::new(WorthUiPlanSwapReceiptParts {
            previous_active_artifact_digest: previous.artifact_digest(),
            previous_active_plan_digest: previous.active_plan_digest(),
            previous_active_snapshot_digest: previous.snapshot_digest(),
            next_active_artifact_digest: next.artifact_digest(),
            next_active_plan_digest: next.active_plan_digest(),
            next_active_snapshot_digest: next.snapshot_digest(),
            activation_gate_receipt: gate_receipt,
            prior_valid_plan: prior_observation,
            counters,
        }))
    }
}

fn capture_prior_valid_plan(
    active: &WorthUiActiveRuntimeState,
    counters: &mut WorthUiAtomicPlanSwapCounters,
) -> WorthUiPriorValidPlan {
    counters.record_prior_valid_capture();
    WorthUiPriorValidPlan::capture(active)
}

fn activate_swap_gate_or_rollback(
    active: &WorthUiActiveRuntimeState,
    ready: &WorthUiReadyActivation,
    boundary: WorthUiFrameBoundary,
    runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
    prior_observation: WorthUiPriorValidPlanObservation,
    counters: &mut WorthUiAtomicPlanSwapCounters,
) -> Result<WorthUiActivationGateReceipt, WorthUiPlanSwapRollback> {
    counters.record_activation_gate();
    match WorthUiFrameActivationGate::activate_at_boundary(
        active.observation(),
        ready,
        boundary,
        runtime_frame_epoch,
    ) {
        Ok(receipt) => Ok(receipt),
        Err(denial) => {
            counters.record_denial();
            Err(rollback(
                WorthUiPlanSwapDenialReason::ActivationGateDenied(denial.reason()),
                prior_observation,
                None,
                None,
                *counters,
            ))
        }
    }
}

fn build_swap_payload_or_rollback(
    active: &WorthUiActiveRuntimeState,
    ready: WorthUiReadyActivation,
    candidate_plan: WorthUiExecutionPlan,
    prior_observation: WorthUiPriorValidPlanObservation,
    counters: &mut WorthUiAtomicPlanSwapCounters,
) -> Result<WorthUiReadyActivationSwapPayload, WorthUiPlanSwapRollback> {
    counters.record_next_active_state_build();
    let candidate_plan_digest = WorthUiExecutionPlanDigestor::digest(&candidate_plan).0;
    let attempted_ready_artifact_digest = ready.candidate_artifact_digest();
    match WorthUiReadyActivationSwapPayload::from_ready_activation(
        ready,
        candidate_plan_digest,
        active.snapshot_digest(),
    ) {
        Ok(payload) => Ok(payload),
        Err(reason) => {
            counters.record_denial();
            Err(rollback(
                reason,
                prior_observation,
                Some(attempted_ready_artifact_digest),
                Some(candidate_plan_digest.raw()),
                *counters,
            ))
        }
    }
}

fn commit_swap_payload(
    active: &mut WorthUiActiveRuntimeState,
    payload: WorthUiReadyActivationSwapPayload,
    runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
) {
    let (next_artifact, next_plan, next_snapshot) = payload.into_parts();
    let preserved_snapshot = active.capability_snapshot().clone();
    *active = WorthUiActiveRuntimeState::from_preserved_authority(
        next_artifact,
        next_plan,
        preserved_snapshot,
        next_snapshot,
        WorthUiRuntimeLifecycle::Active,
        WorthUiRuntimeActivationStatus::Active,
        runtime_frame_epoch,
        active.diagnostic_policy(),
    );
}

#[cfg(test)]
fn injected_precommit_rollback(
    injection: WorthUiPlanSwapInjectionMode,
    payload: &WorthUiReadyActivationSwapPayload,
    gate_receipt: WorthUiActivationGateReceipt,
    prior_observation: WorthUiPriorValidPlanObservation,
    mut counters: WorthUiAtomicPlanSwapCounters,
) -> Option<WorthUiPlanSwapRollback> {
    if injection != WorthUiPlanSwapInjectionMode::BeforeCommit {
        return None;
    }
    counters.record_denial();
    Some(rollback(
        WorthUiPlanSwapDenialReason::InjectedFailureBeforeCommit,
        prior_observation,
        Some(payload.active_artifact().digest().raw()),
        Some(gate_receipt.candidate_execution_plan_digest()),
        counters,
    ))
}

#[cfg(test)]
fn injected_post_artifact_mutation_rollback(
    injection: WorthUiPlanSwapInjectionMode,
    active: &mut WorthUiActiveRuntimeState,
    payload: &WorthUiReadyActivationSwapPayload,
    gate_receipt: WorthUiActivationGateReceipt,
    prior: &WorthUiPriorValidPlan,
    prior_observation: WorthUiPriorValidPlanObservation,
    counters: &mut WorthUiAtomicPlanSwapCounters,
) -> Option<WorthUiPlanSwapRollback> {
    if injection != WorthUiPlanSwapInjectionMode::AfterArtifactMutation {
        return None;
    }
    active.replace_artifact_for_swap_injection_for_test(payload.active_artifact().clone());
    counters.record_active_state_mutation();
    counters.record_rollback_restore();
    *active = prior.restore_active_state();
    counters.record_denial();
    Some(rollback(
        WorthUiPlanSwapDenialReason::InjectedFailureAfterArtifactMutation,
        prior_observation,
        Some(payload.active_artifact().digest().raw()),
        Some(gate_receipt.candidate_execution_plan_digest()),
        *counters,
    ))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthUiPlanSwapInjectionMode {
    None,
    #[cfg(test)]
    BeforeCommit,
    #[cfg(test)]
    AfterArtifactMutation,
}

#[cfg(test)]
impl From<WorthUiPlanSwapFailureInjection> for WorthUiPlanSwapInjectionMode {
    fn from(injection: WorthUiPlanSwapFailureInjection) -> Self {
        match injection {
            WorthUiPlanSwapFailureInjection::BeforeCommit => Self::BeforeCommit,
            WorthUiPlanSwapFailureInjection::AfterArtifactMutation => Self::AfterArtifactMutation,
        }
    }
}

fn rollback(
    reason: WorthUiPlanSwapDenialReason,
    prior: crate::runtime::WorthUiPriorValidPlanObservation,
    attempted_next_artifact_digest: Option<u64>,
    attempted_next_plan_digest: Option<u64>,
    counters: WorthUiAtomicPlanSwapCounters,
) -> WorthUiPlanSwapRollback {
    WorthUiPlanSwapRollback::new(
        reason,
        prior,
        attempted_next_artifact_digest,
        attempted_next_plan_digest,
        counters,
    )
}
