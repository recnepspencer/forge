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
    if running
        .resource_attempt
        .resources()
        .envelope()
        .yield_contract()
        .is_none()
    {
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
    None
}
