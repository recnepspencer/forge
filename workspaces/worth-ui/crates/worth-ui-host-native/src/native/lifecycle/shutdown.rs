#[path = "shutdown/host.rs"]
mod host;
#[path = "shutdown/orchestrator.rs"]
pub(super) mod orchestrator;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum UiNativeShutdownPhase {
    #[default]
    Open,
    SettlingExternalEffects,
    ReleasingDerivedState,
    ReleasingNativeResources,
    Closed,
}

pub(crate) fn progress_shutdown(
    state: &mut crate::native::UiNativeHostState,
) -> crate::native::UiNativeResourceCensus {
    let mut phase = state.lifecycle.shutdown_phase();
    let census =
        orchestrator::progress(&mut phase, &mut host::UiNativeHostShutdownPort::new(state));
    state.lifecycle.record_shutdown_phase(phase);
    census
}
