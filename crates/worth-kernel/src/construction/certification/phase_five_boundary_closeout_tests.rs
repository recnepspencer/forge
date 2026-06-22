use std::path::Path;

use forge_query::facade::consumer_kit::hard_prohibition_boundary_audit;
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use crate::construction::authoring::require_default_primitive_construction_query_authority;
use crate::construction::query_enforcement_adoption::worth_kernel_query_boundary_sources;

const TOPOLOGY_CARGO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/Cargo.toml"
));
const SPATIAL_CARGO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/Cargo.toml"
));
const SPATIAL_LIB: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/lib.rs"
));
const SPATIAL_STRUCTURE_GUARD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/structure_guard.rs"
));
const TOPOLOGY_STRUCTURE_GUARD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/src/certification/structure_guard.rs"
));
const TOPOLOGY_PUBLIC_API: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/src/certification/public_facade_contracts/contracts/public_api.rs"
));
const TOPOLOGY_BOUNDARY_TESTS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/src/construction/boundary_tests.rs"
));
const KERNEL_ADMITTED_SCAFFOLD_ROOT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/mod.rs"
));
const KERNEL_FAMILY_BIRTH_INPUT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/family_birth_input/mod.rs"
));
const KERNEL_REQUEST_GEOMETRY_DISPATCH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/family_birth_input/request_geometry_dispatch.rs"
));
const KERNEL_BIRTH_SCAFFOLD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/family_birth_input/birth_scaffold.rs"
));
const KERNEL_TOPOLOGY_READY_BIRTH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/phase_chain/admitted_scaffold/topology_ready_birth.rs"
));
const KERNEL_LIB_ROOT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
const KERNEL_PUBLIC_API_CONSTRUCTION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/certification/public_facade_contracts/contracts/public_api_construction.rs"
));
const KERNEL_AUTHORING: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/authoring.rs"
));
const KERNEL_QUERY_SUPPORT_PINS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/query_support_pins.rs"
));
const KERNEL_QUERY_SUPPORT_PINS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/query_support_pins.json"
));
const KERNEL_RUNTIME_PROOF_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/runtime_proof"
);
#[test]
fn phase_five_boundary_closeout_report_proves_query_native_construction_boundary() {
    assert!(query_runtime_boundary_audit_clean());
    assert!(topology_rejects_spatial_dependency());
    assert!(spatial_rejects_kernel_dependency());
    assert!(synopsis_owned_admitted_handoff_precedent());
    assert!(kernel_consumes_synopsis_owned_admitted_handoff());
    assert!(public_queryless_happy_path_quarantined());
    assert!(query_runtime_authoring_honesty());
    assert!(family_birth_input_boundary_localized());
    assert!(topology_ready_birth_boundary_localized());
    assert!(dead_runtime_proof_subtree_deleted());
}

fn topology_rejects_spatial_dependency() -> bool {
    !TOPOLOGY_CARGO.contains("worth-spatial.workspace = true")
        && !TOPOLOGY_CARGO.contains("worth-geom.workspace = true")
        && TOPOLOGY_STRUCTURE_GUARD.contains("\"worth-spatial\"")
        && TOPOLOGY_STRUCTURE_GUARD.contains("\"worth-geom\"")
}

fn spatial_rejects_kernel_dependency() -> bool {
    let production_cargo = production_dependency_section(SPATIAL_CARGO);
    !production_cargo.contains("worth-kernel")
        && !production_cargo.contains("worth_kernel")
        && SPATIAL_LIB.contains("mod structure_guard;")
        && SPATIAL_STRUCTURE_GUARD.contains("worth-kernel")
        && SPATIAL_STRUCTURE_GUARD.contains("worth_kernel::")
}

fn production_dependency_section(cargo_toml: &str) -> &str {
    cargo_toml
        .split("[dev-dependencies]")
        .next()
        .expect("split always returns a first section")
}

fn synopsis_owned_admitted_handoff_precedent() -> bool {
    TOPOLOGY_PUBLIC_API
        .contains("prepare_primitive_construction_query_admitted_handoff_from_synopsis")
        && TOPOLOGY_BOUNDARY_TESTS
            .contains("prepare_primitive_construction_query_admitted_handoff_from_synopsis")
}

fn kernel_consumes_synopsis_owned_admitted_handoff() -> bool {
    KERNEL_ADMITTED_SCAFFOLD_ROOT.contains("prepare_primitive_construction_topology_ready_birth(")
        && KERNEL_TOPOLOGY_READY_BIRTH
            .contains("prepare_primitive_construction_query_admitted_handoff_from_synopsis(")
        && !KERNEL_TOPOLOGY_READY_BIRTH.contains("prepare_primitive_construction_query_handoff(")
}

fn public_queryless_happy_path_quarantined() -> bool {
    !KERNEL_LIB_ROOT.contains("pub mod facade;")
        && !KERNEL_PUBLIC_API_CONSTRUCTION.contains("prepare_primitive_construction_result")
        && !KERNEL_PUBLIC_API_CONSTRUCTION.contains("prepare_primitive_construction_outcome")
        && !KERNEL_PUBLIC_API_CONSTRUCTION.contains("pub mod prelude;")
}

fn query_runtime_authoring_honesty() -> bool {
    query_authority_typed_evidence_is_satisfied()
        && KERNEL_AUTHORING.contains("require_default_primitive_construction_query_authority(")
        && KERNEL_QUERY_SUPPORT_PINS.contains("load_support_pin_contract_document(")
        && KERNEL_QUERY_SUPPORT_PINS
            .contains("ForgeQuerySupportPinContractSchemaVersion::current()")
        && KERNEL_QUERY_SUPPORT_PINS_JSON.contains("\"consumer_name\": \"worth-kernel\"")
        && KERNEL_QUERY_SUPPORT_PINS_JSON.contains("\"pinned_vocabulary_identity\"")
        && !KERNEL_AUTHORING.contains("REQUIRED_QUERY_FAMILIES")
        && !KERNEL_AUTHORING.contains("REPORTED_QUERY_FAMILIES")
        && !KERNEL_AUTHORING.contains("PrimitiveConstructionQueryGapRow")
        && !KERNEL_AUTHORING.contains("support_pinning_contract(\"worth-kernel\")")
        && !KERNEL_AUTHORING.contains("project_workspace_support_snapshot(")
        && !KERNEL_AUTHORING.contains("require_primitive_construction_query_entry(")
        && KERNEL_AUTHORING.contains("ForgeQueryWorkspace")
        && !KERNEL_AUTHORING.contains("author_primitive_construction_declaration(")
        && !KERNEL_AUTHORING.contains("PrimitiveConstructionAuthoringEntry")
}

fn query_authority_typed_evidence_is_satisfied() -> bool {
    let runtime = match milestone_one_runtime_builder() {
        Ok(builder) => builder.build(),
        Err(_) => return false,
    };
    let workspace = match topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-five-closeout-query-authority".to_string(),
    ) {
        Ok(workspace) => workspace,
        Err(_) => return false,
    };
    let receipt = match require_default_primitive_construction_query_authority(&workspace) {
        Ok(receipt) => receipt,
        Err(_) => return false,
    };

    receipt.support_pins_satisfied()
        && receipt.support_pin_finding_count() == 0
        && receipt.support_pin_blocking_finding_count() == 0
        && receipt.evaluated_support_source_matrix_digest()
            == workspace
                .public_support_matrix()
                .matrix_digest()
                .terminal_projection_for_reporting()
}

fn family_birth_input_boundary_localized() -> bool {
    KERNEL_ADMITTED_SCAFFOLD_ROOT.contains("family_birth_input::build_family_birth_input(")
        && KERNEL_FAMILY_BIRTH_INPUT
            .contains("request_geometry_dispatch::build_request_geometry_birth_input(")
        && KERNEL_REQUEST_GEOMETRY_DISPATCH.contains("match request.geometry()")
        && KERNEL_BIRTH_SCAFFOLD.contains("PrimitiveConstructionBirthScaffoldPlan")
}

fn topology_ready_birth_boundary_localized() -> bool {
    KERNEL_ADMITTED_SCAFFOLD_ROOT.contains("prepare_primitive_construction_topology_ready_birth(")
        && !KERNEL_ADMITTED_SCAFFOLD_ROOT.contains("plan_primitive_construction_birth(")
        && !KERNEL_ADMITTED_SCAFFOLD_ROOT
            .contains("TopologyPrimitiveConstructionQueryBirthSynopsis::new(")
        && !KERNEL_ADMITTED_SCAFFOLD_ROOT.contains("topology_family_from_spatial_family(")
        && KERNEL_TOPOLOGY_READY_BIRTH
            .contains("TopologyPrimitiveConstructionQueryBirthSynopsis::new(")
        && KERNEL_TOPOLOGY_READY_BIRTH.contains("topology_family_from_spatial_family(")
}

fn dead_runtime_proof_subtree_deleted() -> bool {
    !Path::new(KERNEL_RUNTIME_PROOF_DIR).exists()
}

fn query_runtime_boundary_audit_clean() -> bool {
    hard_prohibition_boundary_audit()
        .covering_sources(worth_kernel_query_boundary_sources())
        .try_assert_clean()
        .is_ok()
}
