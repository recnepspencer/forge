use worth_ui_host_contract::UiHostSurfaceRegistrationOutcome;

fn observed_known_empty(outcome: UiHostSurfaceRegistrationOutcome) -> bool {
    matches!(outcome, UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty)
}

fn main() {
    let _ = observed_known_empty;
}
