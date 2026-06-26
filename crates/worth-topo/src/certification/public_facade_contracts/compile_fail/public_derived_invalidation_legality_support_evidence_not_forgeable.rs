use topology::derived_invalidation_selected_plan::DerivedInvalidationLegalitySupportEvidence;

fn main() {
    let _ = DerivedInvalidationLegalitySupportEvidence {
        selected_legality_plan_digest: None,
        selected_validator_receipt_digest: None,
        support_digest: String::new(),
    };
}
