use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};
use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationReceipt;

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

deny_direct_target!(direct_bridge, BridgeExecutionBasisFinalizationReceipt);
deny_direct_target!(direct_relational, RelationalExecutionBasisLease);
deny_workflow_target!(workflow_bridge, BridgeExecutionBasisFinalizationReceipt);
deny_workflow_target!(workflow_relational, RelationalExecutionBasisLease);

fn main() {}
