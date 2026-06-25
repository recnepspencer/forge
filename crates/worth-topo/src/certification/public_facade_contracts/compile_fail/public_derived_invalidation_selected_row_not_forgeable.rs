use topology::derived_invalidation_family_catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyQueryReceiptPosture,
};
use topology::derived_invalidation_selected_plan::{
    DerivedInvalidationPlannedDisposition, DerivedInvalidationSelectedRow,
};

fn main() {
    let _ = DerivedInvalidationSelectedRow {
        family_identity: DerivedTopologyProductFamilyIdentity::LoopCycles,
        family_digest: String::new(),
        query_posture: DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        query_receipt_digest: None,
        legality_posture: DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
        legality_receipt_digest: None,
        planned_disposition: DerivedInvalidationPlannedDisposition::IncrementalUpdate,
        row_digest: String::new(),
    };
}
