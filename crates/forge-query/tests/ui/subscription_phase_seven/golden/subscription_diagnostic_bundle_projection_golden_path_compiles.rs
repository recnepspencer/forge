use forge_query::facade::{
    DiagnosticAssemblyReceipt, QuerySubscriptionDiagnosticBundleWidth,
    QuerySubscriptionDiagnosticSemanticLabels,
};

fn bundle_width_projection_golden_path(width: &QuerySubscriptionDiagnosticBundleWidth) {
    let _ = width.bundle_width_projection().label();
}

fn assembly_receipt_projection_golden_path(receipt: &DiagnosticAssemblyReceipt) {
    let _ = receipt.assembly_receipt_projection().label();
}

fn semantic_labels_projection_golden_path(labels: &QuerySubscriptionDiagnosticSemanticLabels) {
    let _ = labels.labels_projection().label();
}

fn main() {}
