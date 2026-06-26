use topology::derived_invalidation_family_catalog::{
    DerivedTopologyProductFamilyIdentity, DerivedTopologyQueryReceiptPosture,
};
use topology::derived_invalidation_selected_plan::{
    DerivedInvalidationDenialKind, DerivedInvalidationDenialRow,
};

fn main() {
    let _ = DerivedInvalidationDenialRow {
        kind: DerivedInvalidationDenialKind::MissingQuerySupport,
        family_identity: DerivedTopologyProductFamilyIdentity::LoopCycles,
        family_digest: String::new(),
        required_query_posture: Some(DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired),
        required_legality_posture: None,
        denial_digest: String::new(),
    };
}
