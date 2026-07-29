use worth_ui::facade::app::{WorthUiNativeApplicationShell, WorthUiNativeSourceRebindDenial};
use worth_ui::facade::rebind::{UiRebindOutcome, UiRebindReceipt, UiSourceRebindRequest};

pub(super) enum PlatformPulseRebindAction {
    Published(UiRebindReceipt),
    ObservedNoChange,
    SourceDenied(WorthUiNativeSourceRebindDenial),
    Denied(WorthUiNativeSourceRebindDenial),
    NonterminalDisposed,
}

pub(super) fn normalize_rebind(
    shell: &mut WorthUiNativeApplicationShell,
    snapshot: worth_ui::facade::source::WorthUiSettledSourceSnapshot,
    deadline_tick: u64,
    now_tick: u64,
) -> PlatformPulseRebindAction {
    let request = UiSourceRebindRequest::new(snapshot)
        .with_deadline(shell.rebind_deadline_at(deadline_tick))
        .observed_at_tick(now_tick);
    match shell.begin_source_rebind(request) {
        Ok(outcome) => normalize_rebind_outcome(outcome, now_tick),
        Err(denial) if denial.source_failure().is_some() => {
            PlatformPulseRebindAction::SourceDenied(denial)
        }
        Err(denial) => PlatformPulseRebindAction::Denied(denial),
    }
}

fn normalize_rebind_outcome(
    mut outcome: UiRebindOutcome<'_>,
    mut now_tick: u64,
) -> PlatformPulseRebindAction {
    loop {
        match outcome {
            UiRebindOutcome::Published(receipt) => {
                return PlatformPulseRebindAction::Published(receipt)
            }
            UiRebindOutcome::ObservedNoChange(_) => {
                return PlatformPulseRebindAction::ObservedNoChange
            }
            UiRebindOutcome::InFlight(handle) => {
                now_tick = now_tick.saturating_add(1);
                outcome = handle.complete(now_tick);
            }
            outcome => {
                drop(outcome);
                return PlatformPulseRebindAction::NonterminalDisposed;
            }
        }
    }
}
