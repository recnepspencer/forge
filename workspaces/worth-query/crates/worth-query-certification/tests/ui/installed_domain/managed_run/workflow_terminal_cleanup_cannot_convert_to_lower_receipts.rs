use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryWorkflowExecutionAttemptReleaseReceipt, WorthQueryWorkflowRunCleanupInspection,
};
use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationReceipt;

fn require_deref<T: Deref<Target = U>, U>() {}
fn require_as_ref<T: AsRef<U>, U>() {}
fn require_borrow<T: Borrow<U>, U>() {}
fn require_into<T: Into<U>, U>() {}

fn main() {
    require_deref::<WorthQueryWorkflowRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>(
    );
    require_as_ref::<WorthQueryWorkflowRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>(
    );
    require_borrow::<WorthQueryWorkflowRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>(
    );
    require_into::<WorthQueryWorkflowRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>(
    );

    require_deref::<WorthQueryWorkflowRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>(
    );
    require_as_ref::<WorthQueryWorkflowRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>(
    );
    require_borrow::<WorthQueryWorkflowRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>(
    );
    require_into::<WorthQueryWorkflowRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>(
    );

    require_deref::<
        WorthQueryWorkflowRunCleanupInspection,
        WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    >();
    require_as_ref::<
        WorthQueryWorkflowRunCleanupInspection,
        WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    >();
    require_borrow::<
        WorthQueryWorkflowRunCleanupInspection,
        WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    >();
    require_into::<
        WorthQueryWorkflowRunCleanupInspection,
        WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    >();
}
