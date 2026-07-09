use worth_signal::facade::{
    CommittedResourceCompletionArtifact, ResourceLifecycleTransition, StagedResourceCompletionEffect,
};

fn WORTHd_staged_effect() -> StagedResourceCompletionEffect {
    loop {}
}

fn WORTHd_transition() -> ResourceLifecycleTransition {
    loop {}
}

fn main() {
    let _ = CommittedResourceCompletionArtifact {
        staged_effect: WORTHd_staged_effect(),
        lifecycle_transition: WORTHd_transition(),
    };
}
