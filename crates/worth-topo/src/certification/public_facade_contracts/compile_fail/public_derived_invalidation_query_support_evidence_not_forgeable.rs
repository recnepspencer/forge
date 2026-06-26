use topology::derived_invalidation_selected_plan::DerivedInvalidationQuerySupportEvidence;

fn main() {
    let _ = DerivedInvalidationQuerySupportEvidence {
        projection_consumption_receipt_digest: None,
        native_read_receipt_digest: None,
        native_write_receipt_digest: None,
        support_digest: String::new(),
    };
}
