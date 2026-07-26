use worth_runtime_bridge::facade::BridgeExecutionSafePointSignalState;

use super::{WorthQueryPausedWorkflowGraphExecution, WorthQueryWorkflowYieldDenialKind};

pub(super) fn classify_workflow_yield_denial(
    paused: &WorthQueryPausedWorkflowGraphExecution,
) -> Option<(WorthQueryWorkflowYieldDenialKind, &'static str)> {
    let running = &paused.active.running;
    if !running
        .resource_attempt
        .binding_authority()
        .is_current_installation_generation()
    {
        return Some((
            WorthQueryWorkflowYieldDenialKind::InstallationGenerationStale,
            "the installed workflow generation changed after the run was admitted",
        ));
    }
    let Some(contract) = running
        .resource_attempt
        .operation_resources()
        .envelope()
        .yield_contract()
    else {
        return Some((
            WorthQueryWorkflowYieldDenialKind::YieldNotInstalled,
            "the admitted resource envelope does not install provider checkpoint yield",
        ));
    };
    if !paused.safe_point.checkpoint_available() {
        return Some((
            WorthQueryWorkflowYieldDenialKind::CheckpointUnavailable,
            "the consumed provider safe point reported no checkpoint availability",
        ));
    }
    if paused.safe_point.observation().signal_state() != BridgeExecutionSafePointSignalState::Active
    {
        return Some((
            WorthQueryWorkflowYieldDenialKind::SignalAttemptNotActive,
            "the exact Signal request attempt is no longer active",
        ));
    }
    if paused.safe_point.observation().queue_depth() != 0 {
        return Some((
            WorthQueryWorkflowYieldDenialKind::QueueNotDrained,
            "yield requires every pending result chunk to be acknowledged",
        ));
    }
    if paused.active.execution.applied_effect_count() != 0 && !contract.partial_effects_may_remain()
    {
        return Some((
            WorthQueryWorkflowYieldDenialKind::PartialEffectPostureMismatch,
            "applied effects exceed the installed effect-free yield posture",
        ));
    }
    None
}

pub(super) fn classify_workflow_retained_bytes_denial(
    provider_retained_bytes: u64,
    artifact_retained_bytes: usize,
    retained_bytes_ceiling: u64,
) -> Option<(WorthQueryWorkflowYieldDenialKind, &'static str)> {
    let retained_total = provider_retained_bytes
        .saturating_add(u64::try_from(artifact_retained_bytes).unwrap_or(u64::MAX));
    if retained_total > retained_bytes_ceiling {
        return Some((
            WorthQueryWorkflowYieldDenialKind::RetainedBytesExceeded,
            "provider and workflow artifacts exceed the installed retained-byte ceiling",
        ));
    }
    None
}
