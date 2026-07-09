use worth_signal::facade::{AdmittedResourceCompletion, StagedResourceCompletionEffect};

fn WORTHd_admitted_completion() -> AdmittedResourceCompletion {
    loop {}
}

fn main() {
    let _ = StagedResourceCompletionEffect {
        admitted_completion: WORTHd_admitted_completion(),
    };
}
