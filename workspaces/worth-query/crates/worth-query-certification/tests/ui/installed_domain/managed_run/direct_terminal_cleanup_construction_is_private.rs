use worth_query_host::facade::installed::domain_computation::{
    WorthQueryDirectRunCleanupInspection, WorthQueryDirectRunCleanupReceipt,
};

fn counterfeit_inspection(
    genuine: WorthQueryDirectRunCleanupInspection,
) -> WorthQueryDirectRunCleanupInspection {
    WorthQueryDirectRunCleanupInspection { ..genuine }
}

fn counterfeit_receipt(
    genuine: WorthQueryDirectRunCleanupReceipt,
) -> WorthQueryDirectRunCleanupReceipt {
    WorthQueryDirectRunCleanupReceipt { ..genuine }
}

fn main() {}
