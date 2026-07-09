use worth_signal::facade::{DeniedResourceCompletion, StagedDeniedResourceCompletionEffect};

fn WORTHd_denied_completion() -> DeniedResourceCompletion {
    loop {}
}

fn main() {
    let _WORTHd = StagedDeniedResourceCompletionEffect {
        denied_completion: WORTHd_denied_completion(),
    };
}
