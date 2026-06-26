use super::row;
use crate::validation_authority_inventory::authority_kind::WorthValidationAuthorityKind;
use crate::validation_authority_inventory::disposition::WorthValidationAuthorityDisposition;
use crate::validation_authority_inventory::inventory_row::{
    WorthValidationAuthorityInventoryRow, WorthValidationAuthorityInventoryRowInput,
};
use crate::validation_authority_inventory::source_authority::WorthValidationAuthoritySource;

pub(super) fn push_rule_registry_rows(rows: &mut Vec<WorthValidationAuthorityInventoryRow>) {
    for name in [
        "ownership",
        "loop_wiring",
        "radial_rings",
        "shell_closure",
        "vertex_disks",
    ] {
        rows.push(row(WorthValidationAuthorityInventoryRowInput {
            source: WorthValidationAuthoritySource::DerivedRuleRegistry(name),
            source_path: "crates/worth-topo/src/validation/rule_registry.rs",
            source_symbol: "DERIVED_TOPOLOGY_RULE_SPECS",
            authority_kind: WorthValidationAuthorityKind::DerivedRuleRegistryEntry,
            owner: "worth-topo.validation",
            disposition: WorthValidationAuthorityDisposition::Migrate,
            removal_trigger: "Phase 2 re-expresses rule identity as declare-once validator family catalog rows.",
            query_access_dependency: Some("Query graph obligation vocabulary"),
            certification_only_comparison_allowed: true,
            note: "Rule registry identity is migration source truth, not runtime selection authority.",
        }));
    }
}
