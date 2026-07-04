use super::row;
use crate::validation_authority_inventory::authority_kind::WorthValidationAuthorityKind;
use crate::validation_authority_inventory::disposition::WorthValidationAuthorityDisposition;
use crate::validation_authority_inventory::inventory_row::{
    WorthValidationAuthorityInventoryRow, WorthValidationAuthorityInventoryRowInput,
};
use crate::validation_authority_inventory::source_authority::WorthValidationAuthoritySource;

pub(super) fn push_old_authority_region_rows(rows: &mut Vec<WorthValidationAuthorityInventoryRow>) {
    for old_authority_region in old_authority_regions() {
        rows.push(row(WorthValidationAuthorityInventoryRowInput {
            source: WorthValidationAuthoritySource::OldAuthorityUseRegion(
                old_authority_region.region,
            ),
            source_path: old_authority_region.source_path,
            source_symbol: old_authority_region.symbol,
            authority_kind: old_authority_region.authority_kind,
            owner: old_authority_region.owner,
            disposition: WorthValidationAuthorityDisposition::Cap,
            removal_trigger: "Phase 5/7 removes this old authority use after Query-registered obligations own closeout.",
            query_access_dependency: Some(old_authority_region.dependency),
            certification_only_comparison_allowed: true,
            note: "Region-scoped old authority residue; comparison only and not ordinary legality.",
        }));
    }
}

fn old_authority_regions() -> &'static [OldAuthorityRegion] {
    &OLD_AUTHORITY_REGIONS
}

const OLD_AUTHORITY_REGIONS: [OldAuthorityRegion; 15] = [
    invariant_runtime_region(
        "topology_operator_closeout_tests",
        "crates/worth-topo/src/certification/topology_operator_closeout/tests",
        "worth-topo.certification.topology_operator_closeout",
    ),
    invariant_runtime_region(
        "topology_operator_closeout_acceptance_rows",
        "crates/worth-topo/src/certification/topology_operator_closeout/acceptance_rows",
        "worth-topo.certification.topology_operator_closeout",
    ),
    invariant_runtime_region(
        "topology_operator_closeout_operator_family_proof",
        "crates/worth-topo/src/certification/topology_operator_closeout/operator_family_proof",
        "worth-topo.certification.topology_operator_closeout",
    ),
    invariant_runtime_region(
        "topology_operator_closeout_scale_pressure",
        "crates/worth-topo/src/certification/topology_operator_closeout/scale_pressure_proof",
        "worth-topo.certification.topology_operator_closeout",
    ),
    invariant_runtime_region(
        "projection_closeout_tests",
        "crates/worth-topo/src/certification/projection_closeout/tests",
        "worth-topo.certification.projection_closeout",
    ),
    invariant_runtime_region(
        "query_runtime_tests",
        "crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests",
        "worth-topo.projection.runtime_boundary.query_runtime",
    ),
    invariant_runtime_region(
        "certification_support_declaration_runtime",
        "crates/worth-topo/src/certification/support/declaration_runtime.rs",
        "worth-topo.certification.support",
    ),
    invariant_runtime_region(
        "topology_operators_adoption_tests",
        "crates/worth-topo/src/topology_operators/adoption_tests",
        "worth-topo.topology_operators.adoption_tests",
    ),
    invariant_runtime_region(
        "public_facade_contracts_public_api",
        "crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api.rs",
        "worth-topo.certification.public_facade_contracts",
    ),
    invariant_runtime_region(
        "runtime_boundary_bridge_tests",
        "crates/worth-topo/src/projection/runtime_boundary/bridge/tests.rs",
        "worth-topo.projection.runtime_boundary.bridge",
    ),
    whole_view_validation_region(
        "workload_topology_seed_validation",
        "crates/worth-topo/src/workload_platform/topology_seed/seed_recipe.rs",
        "TopologyValidator::derived_validation_report",
        "worth-topo.workload_platform.topology_seed",
    ),
    whole_view_validation_region(
        "workload_nmt_construction_validation",
        "crates/worth-topo/src/workload_platform/nmt_topology_construction/construction.rs",
        "TopologyValidator::derived_validation_report",
        "worth-topo.workload_platform.nmt_topology_construction",
    ),
    whole_view_validation_region(
        "diagnostic_surface_validation",
        "crates/worth-topo/src/projection/planner_owned_routing/diagnostic_projection_input/source.rs",
        "validate_interpreted_topology",
        "worth-topo.projection.planner_owned_routing.diagnostic_projection_input",
    ),
    whole_view_validation_region(
        "certification_support_parity_validation",
        "crates/worth-topo/src/certification/support/parity/tests.rs",
        "validate_interpreted_topology",
        "worth-topo.certification.support.parity",
    ),
    OldAuthorityRegion {
        region: "validation_reference_integrity_tests",
        source_path: "crates/worth-topo/src/validation/reference_integrity/tests",
        symbol: "milestone_one_invariant_registrations",
        owner: "worth-topo.validation.reference_integrity",
        dependency: "Query invariant registration facade",
        authority_kind: WorthValidationAuthorityKind::RuntimeInvariantRegistrationPack,
    },
];

const fn invariant_runtime_region(
    region: &'static str,
    source_path: &'static str,
    owner: &'static str,
) -> OldAuthorityRegion {
    OldAuthorityRegion {
        region,
        source_path,
        symbol: "build_milestone_one_runtime",
        owner,
        dependency: "Query invariant registration facade",
        authority_kind: WorthValidationAuthorityKind::RuntimeInvariantRegistrationPack,
    }
}

const fn whole_view_validation_region(
    region: &'static str,
    source_path: &'static str,
    symbol: &'static str,
    owner: &'static str,
) -> OldAuthorityRegion {
    OldAuthorityRegion {
        region,
        source_path,
        symbol,
        owner,
        dependency: "Milestone 9 selected obligation receipts",
        authority_kind: WorthValidationAuthorityKind::WholeViewValidatorEntry,
    }
}

struct OldAuthorityRegion {
    region: &'static str,
    source_path: &'static str,
    symbol: &'static str,
    owner: &'static str,
    dependency: &'static str,
    authority_kind: WorthValidationAuthorityKind,
}
