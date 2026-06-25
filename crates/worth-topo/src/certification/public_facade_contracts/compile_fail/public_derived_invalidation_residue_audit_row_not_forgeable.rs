use topology::derived_invalidation_authority_inventory::{
    DerivedInvalidationAuthorityOwner, DerivedInvalidationOldAuthorityKind,
    DerivedInvalidationProductCategory,
};
use topology::derived_invalidation_deletion_closeout::DerivedInvalidationResidueAuditRow;

fn main() {
    let _ = DerivedInvalidationResidueAuditRow {
        source_path: String::new(),
        surface: String::new(),
        product_category: DerivedInvalidationProductCategory::CertificationBootstrap,
        authority_kind: DerivedInvalidationOldAuthorityKind::CertificationBootstrapMaterialization,
        owner: DerivedInvalidationAuthorityOwner::WorthTopoCertification,
        capped_count: 1,
        blocker: String::new(),
        removal_trigger: String::new(),
        certification_or_bootstrap_only: true,
        ordinary_invalidation_admissible: false,
        inventory_row_digest: String::new(),
        row_digest: String::new(),
    };
}
