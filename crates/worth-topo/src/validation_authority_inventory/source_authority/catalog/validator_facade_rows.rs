use super::row;
use crate::validation_authority_inventory::authority_kind::WorthValidationAuthorityKind;
use crate::validation_authority_inventory::disposition::WorthValidationAuthorityDisposition;
use crate::validation_authority_inventory::inventory_row::{
    WorthValidationAuthorityInventoryRow, WorthValidationAuthorityInventoryRowInput,
};
use crate::validation_authority_inventory::source_authority::WorthValidationAuthoritySource;

pub(super) fn push_validator_facade_rows(rows: &mut Vec<WorthValidationAuthorityInventoryRow>) {
    rows.push(row(WorthValidationAuthorityInventoryRowInput {
        source: WorthValidationAuthoritySource::TopologyValidatorMaterializedReport,
        source_path: "crates/worth-topo/src/validation/facade.rs",
        source_symbol: "TopologyValidator::materialized_validation_report",
        authority_kind: WorthValidationAuthorityKind::WholeViewValidatorEntry,
        owner: "worth-topo.validation",
        disposition: WorthValidationAuthorityDisposition::Migrate,
        removal_trigger: "Phase 4 migrates materialized validator families into Query-selected obligation execution.",
        query_access_dependency: Some("Milestone 8 access receipts for materialized topology facts"),
        certification_only_comparison_allowed: true,
        note: "Old whole-view validator entry; comparison oracle only after catalog routing exists.",
    }));
    rows.push(row(WorthValidationAuthorityInventoryRowInput {
        source: WorthValidationAuthoritySource::TopologyValidatorDerivedReport,
        source_path: "crates/worth-topo/src/validation/facade.rs",
        source_symbol: "TopologyValidator::derived_validation_report",
        authority_kind: WorthValidationAuthorityKind::WholeViewValidatorEntry,
        owner: "worth-topo.validation",
        disposition: WorthValidationAuthorityDisposition::Migrate,
        removal_trigger:
            "Phase 4/6 replace derived report execution with selected obligation receipts.",
        query_access_dependency: Some("Milestone 8 access receipts for interpreted topology facts"),
        certification_only_comparison_allowed: true,
        note: "Old broad derived validation report; cannot satisfy ordinary operator closeout.",
    }));
    rows.push(row(WorthValidationAuthorityInventoryRowInput {
        source: WorthValidationAuthoritySource::ValidateInterpretedTopologyFacade,
        source_path: "crates/worth-topo/src/validation/facade.rs",
        source_symbol: "validate_interpreted_topology",
        authority_kind: WorthValidationAuthorityKind::WholeViewValidatorEntry,
        owner: "worth-topo.validation",
        disposition: WorthValidationAuthorityDisposition::Migrate,
        removal_trigger:
            "Phase 7 cuts public validation facade consumers to selected legality proof.",
        query_access_dependency: Some("Query-selected obligation receipts"),
        certification_only_comparison_allowed: true,
        note: "Facade wrapper around derived validation; old authority until rerouted.",
    }));
    rows.push(row(WorthValidationAuthorityInventoryRowInput {
        source: WorthValidationAuthoritySource::ValidateNamedTopologyTruthFacade,
        source_path: "crates/worth-topo/src/validation/facade.rs",
        source_symbol: "validate_named_topology_truth",
        authority_kind: WorthValidationAuthorityKind::WholeViewValidatorEntry,
        owner: "worth-topo.validation",
        disposition: WorthValidationAuthorityDisposition::QueryAccessGap,
        removal_trigger: "Later Query truth-validator posture admits named truth validation as graph obligation.",
        query_access_dependency: Some("Query truth-read support posture"),
        certification_only_comparison_allowed: true,
        note: "Truth naming validation remains visible but cannot be promoted to selected topology legality.",
    }));
}
