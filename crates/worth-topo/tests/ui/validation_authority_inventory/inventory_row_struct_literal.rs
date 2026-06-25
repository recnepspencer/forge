use topology::validation_authority_inventory::{
    WorthValidationAuthorityDisposition, WorthValidationAuthorityInventoryRow,
    WorthValidationAuthorityKind, WorthValidationAuthoritySource,
};

fn main() {
    let _row = WorthValidationAuthorityInventoryRow {
        source: WorthValidationAuthoritySource::TopologyValidatorDerivedReport,
        source_path: "fake.rs",
        source_symbol: "TopologyValidator::derived_validation_report",
        authority_kind: WorthValidationAuthorityKind::WholeViewValidatorEntry,
        owner: "fake",
        disposition: WorthValidationAuthorityDisposition::Migrate,
        removal_trigger: "fake",
        query_access_dependency: None,
        certification_only_comparison_allowed: true,
        note: "fake",
    };
}
