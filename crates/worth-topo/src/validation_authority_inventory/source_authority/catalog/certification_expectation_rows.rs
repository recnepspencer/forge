use super::row;
use crate::validation_authority_inventory::authority_kind::WorthValidationAuthorityKind;
use crate::validation_authority_inventory::disposition::WorthValidationAuthorityDisposition;
use crate::validation_authority_inventory::inventory_row::{
    WorthValidationAuthorityInventoryRow, WorthValidationAuthorityInventoryRowInput,
};
use crate::validation_authority_inventory::source_authority::WorthValidationAuthoritySource;

pub(super) fn push_certification_expectation_rows(
    rows: &mut Vec<WorthValidationAuthorityInventoryRow>,
) {
    for suite in ["milestone_one", "milestone_two", "milestone_three"] {
        rows.push(row(WorthValidationAuthorityInventoryRowInput {
            source: WorthValidationAuthoritySource::CertificationValidatorExpectations(suite),
            source_path: "crates/worth-topo/src/certification/requirements.rs",
            source_symbol: "CertificationSuiteRequirements::validator_expectations",
            authority_kind: WorthValidationAuthorityKind::CertificationExpectationArray,
            owner: "worth-topo.certification",
            disposition: WorthValidationAuthorityDisposition::Cap,
            removal_trigger: "Phase 7 replaces expectation-array closeout with selected obligation closeout.",
            query_access_dependency: Some("Milestone 9 selected obligation closeout proof"),
            certification_only_comparison_allowed: true,
            note: "Expectation arrays are certification scaffolding only; no ordinary operator authority.",
        }));
    }
}
