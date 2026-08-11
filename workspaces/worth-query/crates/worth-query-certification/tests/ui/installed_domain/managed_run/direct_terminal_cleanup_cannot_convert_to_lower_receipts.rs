use std::borrow::Borrow;
use std::ops::Deref;

use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectExecutionAttemptReleaseReceipt, WorthQueryDirectRunCleanupInspection,
};
use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationReceipt;

fn require_deref<T: Deref<Target = U>, U>() {}
fn require_as_ref<T: AsRef<U>, U>() {}
fn require_borrow<T: Borrow<U>, U>() {}
fn require_into<T: Into<U>, U>() {}

fn main() {
    require_deref::<WorthQueryDirectRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>();
    require_as_ref::<WorthQueryDirectRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>();
    require_borrow::<WorthQueryDirectRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>();
    require_into::<WorthQueryDirectRunCleanupInspection, BridgeExecutionBasisFinalizationReceipt>();

    require_deref::<WorthQueryDirectRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>();
    require_as_ref::<WorthQueryDirectRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>();
    require_borrow::<WorthQueryDirectRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>();
    require_into::<WorthQueryDirectRunCleanupInspection, RelationalExecutionBasisReleaseReceipt>();

    require_deref::<WorthQueryDirectRunCleanupInspection, WorthQueryDirectExecutionAttemptReleaseReceipt>();
    require_as_ref::<WorthQueryDirectRunCleanupInspection, WorthQueryDirectExecutionAttemptReleaseReceipt>();
    require_borrow::<WorthQueryDirectRunCleanupInspection, WorthQueryDirectExecutionAttemptReleaseReceipt>();
    require_into::<WorthQueryDirectRunCleanupInspection, WorthQueryDirectExecutionAttemptReleaseReceipt>();
}
