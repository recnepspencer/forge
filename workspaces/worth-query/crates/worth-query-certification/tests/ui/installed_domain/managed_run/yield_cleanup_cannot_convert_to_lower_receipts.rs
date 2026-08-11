use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectExecutionAttemptReleaseReceipt, WorthQueryDirectYieldCleanupInspection,
    WorthQueryWorkflowExecutionAttemptReleaseReceipt, WorthQueryWorkflowYieldCleanupInspection,
    WorthQueryWorkflowYieldRecoveryCleanupInspection,
};
use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationReceipt;

fn deny<T, U>()
where
    T: Deref<Target = U> + AsRef<U> + Borrow<U> + Into<U>,
{
}

fn main() {
    deny::<WorthQueryDirectYieldCleanupInspection, BridgeExecutionBasisFinalizationReceipt>();
    deny::<WorthQueryDirectYieldCleanupInspection, RelationalExecutionBasisReleaseReceipt>();
    deny::<WorthQueryDirectYieldCleanupInspection, WorthQueryDirectExecutionAttemptReleaseReceipt>(
    );
    deny::<WorthQueryWorkflowYieldCleanupInspection, BridgeExecutionBasisFinalizationReceipt>();
    deny::<WorthQueryWorkflowYieldCleanupInspection, RelationalExecutionBasisReleaseReceipt>();
    deny::<
        WorthQueryWorkflowYieldCleanupInspection,
        WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    >();
    deny::<WorthQueryWorkflowYieldRecoveryCleanupInspection, BridgeExecutionBasisFinalizationReceipt>(
    );
    deny::<WorthQueryWorkflowYieldRecoveryCleanupInspection, RelationalExecutionBasisReleaseReceipt>(
    );
    deny::<
        WorthQueryWorkflowYieldRecoveryCleanupInspection,
        WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    >();
}
