use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryProviderCheckpointEvidence,
    WorthQueryWorkflowExecutionResourceAttempt, WorthQueryYieldedDirectRun,
    WorthQueryYieldedWorkflowRun,
};

macro_rules! deny_direct_target {
    ($name:ident, $target:ty) => {
        fn $name(run: WorthQueryYieldedDirectRun) {
            fn require_deref<T: Deref<Target = $target>>(_: &T) {}
            fn require_as_ref<T: AsRef<$target>>(_: &T) {}
            fn require_borrow<T: Borrow<$target>>(_: &T) {}
            require_deref(&run);
            require_as_ref(&run);
            require_borrow(&run);
            let _: $target = run.into();
        }
    };
}

macro_rules! deny_workflow_target {
    ($name:ident, $target:ty) => {
        fn $name(run: WorthQueryYieldedWorkflowRun) {
            fn require_deref<T: Deref<Target = $target>>(_: &T) {}
            fn require_as_ref<T: AsRef<$target>>(_: &T) {}
            fn require_borrow<T: Borrow<$target>>(_: &T) {}
            require_deref(&run);
            require_as_ref(&run);
            require_borrow(&run);
            let _: $target = run.into();
        }
    };
}

deny_direct_target!(direct_attempt, WorthQueryDirectExecutionResourceAttempt);
deny_direct_target!(direct_checkpoint, WorthQueryProviderCheckpointEvidence);
deny_workflow_target!(workflow_attempt, WorthQueryWorkflowExecutionResourceAttempt);
deny_workflow_target!(workflow_checkpoint, WorthQueryProviderCheckpointEvidence);

fn main() {}
