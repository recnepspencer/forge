use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectExecutionAttemptReleaseReceipt,
    WorthQueryDirectReadmissionCleanupInspection,
    WorthQueryDirectReadmissionCleanupPending,
    WorthQueryDirectReadmissionCleanupPendingInspection,
    WorthQueryDirectReadmissionCleanupReceipt,
    WorthQueryProviderCheckpointReleaseEvidence, WorthQueryProviderExecutionReleaseEvidence,
    WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    WorthQueryWorkflowReadmissionCleanupInspection,
    WorthQueryWorkflowReadmissionCleanupPending,
    WorthQueryWorkflowReadmissionCleanupPendingInspection,
    WorthQueryWorkflowReadmissionCleanupReceipt,
};
use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationReceipt,
    BridgeExecutionBasisReadmissionRecoveryRequired,
};

fn deny<T, U>()
where
    T: Deref<Target = U> + AsRef<U> + Borrow<U> + Into<U>,
{
}

fn main() {
    deny::<WorthQueryDirectReadmissionCleanupReceipt, WorthQueryDirectReadmissionCleanupInspection>();
    deny::<WorthQueryDirectReadmissionCleanupReceipt, BridgeExecutionBasisFinalizationReceipt>();
    deny::<WorthQueryDirectReadmissionCleanupInspection, RelationalExecutionBasisReleaseReceipt>();
    deny::<
        WorthQueryDirectReadmissionCleanupInspection,
        WorthQueryDirectExecutionAttemptReleaseReceipt,
    >();
    deny::<WorthQueryDirectReadmissionCleanupInspection, WorthQueryProviderCheckpointReleaseEvidence>();
    deny::<WorthQueryDirectReadmissionCleanupInspection, WorthQueryProviderExecutionReleaseEvidence>();
    deny::<WorthQueryDirectReadmissionCleanupPending, BridgeExecutionBasisReadmissionRecoveryRequired>();
    deny::<
        WorthQueryDirectReadmissionCleanupPending,
        WorthQueryDirectReadmissionCleanupPendingInspection,
    >();
    deny::<WorthQueryDirectReadmissionCleanupPendingInspection, BridgeExecutionBasisReadmissionRecoveryRequired>();
    deny::<WorthQueryWorkflowReadmissionCleanupReceipt, WorthQueryWorkflowReadmissionCleanupInspection>();
    deny::<WorthQueryWorkflowReadmissionCleanupReceipt, BridgeExecutionBasisFinalizationReceipt>();
    deny::<WorthQueryWorkflowReadmissionCleanupInspection, RelationalExecutionBasisReleaseReceipt>();
    deny::<
        WorthQueryWorkflowReadmissionCleanupInspection,
        WorthQueryWorkflowExecutionAttemptReleaseReceipt,
    >();
    deny::<WorthQueryWorkflowReadmissionCleanupInspection, WorthQueryProviderCheckpointReleaseEvidence>();
    deny::<WorthQueryWorkflowReadmissionCleanupInspection, WorthQueryProviderExecutionReleaseEvidence>();
    deny::<WorthQueryWorkflowReadmissionCleanupPending, BridgeExecutionBasisReadmissionRecoveryRequired>();
    deny::<
        WorthQueryWorkflowReadmissionCleanupPending,
        WorthQueryWorkflowReadmissionCleanupPendingInspection,
    >();
    deny::<WorthQueryWorkflowReadmissionCleanupPendingInspection, BridgeExecutionBasisReadmissionRecoveryRequired>();
}
