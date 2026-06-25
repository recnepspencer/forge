use super::row;
use crate::validation_authority_inventory::authority_kind::WorthValidationAuthorityKind;
use crate::validation_authority_inventory::disposition::WorthValidationAuthorityDisposition;
use crate::validation_authority_inventory::inventory_row::{
    WorthValidationAuthorityInventoryRow, WorthValidationAuthorityInventoryRowInput,
};
use crate::validation_authority_inventory::source_authority::WorthValidationAuthoritySource;

pub(super) fn push_operator_closeout_rows(rows: &mut Vec<WorthValidationAuthorityInventoryRow>) {
    rows.push(row(WorthValidationAuthorityInventoryRowInput {
        source: WorthValidationAuthoritySource::OperatorCloseoutValidatorFamilyCoverage,
        source_path: "crates/worth-topo/src/certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs",
        source_symbol: "build_validator_family_coverage_rows",
        authority_kind: WorthValidationAuthorityKind::OperatorCloseoutValidationProof,
        owner: "worth-topo.certification.topology_operator_closeout",
        disposition: WorthValidationAuthorityDisposition::Cap,
        removal_trigger: "Phase 7 replaces validator-family coverage rows with selected obligation proof rows.",
        query_access_dependency: Some("Milestone 9 selected obligation closeout"),
        certification_only_comparison_allowed: true,
        note: "Validator-family rows prove old closeout breadth but cannot become selected obligation proof.",
    }));
    rows.push(row(WorthValidationAuthorityInventoryRowInput {
        source: WorthValidationAuthoritySource::OperatorCloseoutValidationBreadth,
        source_path: "crates/worth-topo/src/certification/topology_operator_closeout/validation_breadth_row.rs",
        source_symbol: "MilestoneThreeValidationBreadthRow",
        authority_kind: WorthValidationAuthorityKind::CertificationComparisonReport,
        owner: "worth-topo.certification.topology_operator_closeout",
        disposition: WorthValidationAuthorityDisposition::Cap,
        removal_trigger: "Phase 7 replaces breadth rows with Query-selected obligation counters.",
        query_access_dependency: Some("Milestone 9 selected obligation counters"),
        certification_only_comparison_allowed: true,
        note: "Breadth rows remain comparison evidence only during migration.",
    }));
    rows.push(row(WorthValidationAuthorityInventoryRowInput {
        source: WorthValidationAuthoritySource::OperatorCloseoutDerivedValidationInspection,
        source_path: "crates/worth-topo/src/certification/topology_operator_closeout/report.rs",
        source_symbol: "MilestoneThreeValidatorFamily::DerivedValidationInspection",
        authority_kind: WorthValidationAuthorityKind::OperatorCloseoutValidationProof,
        owner: "worth-topo.certification.topology_operator_closeout",
        disposition: WorthValidationAuthorityDisposition::Cap,
        removal_trigger:
            "Phase 7 removes derived-validation inspection as ordinary closeout authority.",
        query_access_dependency: Some("Milestone 9 enforcement receipts"),
        certification_only_comparison_allowed: true,
        note: "Derived validation inspection is old proof surface, not a selected obligation.",
    }));
}
