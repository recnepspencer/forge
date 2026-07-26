use worth_runtime_bridge::facade::BridgeExecutionSafePointSignalState::Active;

use super::{WorthQueryDirectYieldDenialKind, WorthQueryPausedDirectGraphExecution};

pub(super) fn classify_direct_yield_denial(
    paused: &WorthQueryPausedDirectGraphExecution,
) -> Option<(WorthQueryDirectYieldDenialKind, &'static str)> {
    let running = &paused.active.running;
    if !running
        .resource_attempt
        .binding_authority()
        .is_current_installation_generation()
    {
        return Some((
            WorthQueryDirectYieldDenialKind::InstallationGenerationStale,
            "the installed operation generation changed after the run was admitted",
        ));
    }
    let contract = running
        .resource_attempt
        .resources()
        .envelope()
        .yield_contract();
    if contract.is_none() {
        return Some((
            WorthQueryDirectYieldDenialKind::YieldNotInstalled,
            "the admitted resource envelope does not install provider checkpoint yield",
        ));
    }
    if !paused.safe_point.checkpoint_available() {
        return Some((
            WorthQueryDirectYieldDenialKind::CheckpointUnavailable,
            "the consumed provider safe point reported no checkpoint availability",
        ));
    }
    if paused.safe_point.observation().signal_state() != Active {
        return Some((
            WorthQueryDirectYieldDenialKind::SignalAttemptNotActive,
            "the exact Signal request attempt is no longer active",
        ));
    }
    if paused.safe_point.observation().queue_depth() != 0 {
        return Some((
            WorthQueryDirectYieldDenialKind::QueueNotDrained,
            "yield requires every pending result chunk to be acknowledged",
        ));
    }
    if paused.active.execution.applied_effect_count() != 0
        && !contract
            .expect("yield contract checked above")
            .partial_effects_may_remain()
    {
        return Some((
            WorthQueryDirectYieldDenialKind::PartialEffectPostureMismatch,
            "applied effects exceed the installed effect-free yield posture",
        ));
    }
    None
}
