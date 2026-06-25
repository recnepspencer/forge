use topology::derived_invalidation_authority_inventory::{
    DerivedInvalidationAuthorityOwner, DerivedInvalidationOldAuthorityKind,
    DerivedInvalidationProductCategory,
};
use topology::derived_invalidation_deletion_closeout::{
    DerivedInvalidationDeletionDisposition, DerivedInvalidationDeletionRow,
};

fn main() {
    let _ = DerivedInvalidationDeletionRow {
        source_path: String::new(),
        surface: String::new(),
        product_category: DerivedInvalidationProductCategory::LoopCycles,
        authority_kind: DerivedInvalidationOldAuthorityKind::WholeViewMaterialization,
        owner: DerivedInvalidationAuthorityOwner::WorthTopoDerivedTopology,
        disposition: DerivedInvalidationDeletionDisposition::MigratedAuthorityDeleted,
        inventory_row_digest: String::new(),
        row_digest: String::new(),
    };
}
