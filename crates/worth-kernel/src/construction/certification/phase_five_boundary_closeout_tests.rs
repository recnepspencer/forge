use std::path::Path;

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
const KERNEL_RUNTIME_PROOF_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/runtime_proof"
);
const QUERY_RUNTIME_AUDITED_FILES: [(&str, &str); 10] = [
    (
        "worth-kernel.authoring",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/authoring.rs"
        )),
    ),
    (
        "worth-kernel.corpus-execution-proof-ingredients",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/tests/support/branch_basis_digest.rs"
        )),
    ),
    (
        "worth-kernel.query-projection-consumption-support",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/tests/support/projection_consumption.rs"
        )),
    ),
    (
        "worth-kernel.outcome",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/result_surface/outcome.rs"
        )),
    ),
    (
        "worth-spatial.primitive-birth",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-spatial/src/bindings/primitive_birth.rs"
        )),
    ),
    (
        "worth-geom.realization-conditioning",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-geom/src/primitives/shape_realization/conditioning.rs"
        )),
    ),
    (
        "worth-geom.realization-support",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-geom/src/primitives/shape_realization/support.rs"
        )),
    ),
    (
        "worth-topo.construction-boundary-mod",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/mod.rs"
        )),
    ),
    (
        "worth-topo.query-native-boundary",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/query_native_boundary.rs"
        )),
    ),
    (
        "worth-topo.boundary-tests",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/boundary_tests.rs"
        )),
    ),
];
const FORBIDDEN_RUNTIME_PATTERNS: [&str; 9] = [
    ".batch(",
    ".write(",
    "bind_existing_entity(",
    "bind_existing_relation(",
    "update_existing(",
    "verify_existing(",
    "update_existing_verified(",
    "delete_existing(",
    "probe_existing(",
];

#[test]
fn phase_five_boundary_closeout_report_proves_query_native_construction_boundary() {
    assert_eq!(query_runtime_violation_count(), 0);
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
    KERNEL_AUTHORING.contains("REQUIRED_QUERY_FAMILIES")
        && KERNEL_AUTHORING.contains("require_primitive_construction_query_entry(")
        && KERNEL_AUTHORING.contains("ForgeQueryWorkspace")
        && !KERNEL_AUTHORING.contains("author_primitive_construction_declaration(")
        && !KERNEL_AUTHORING.contains("PrimitiveConstructionAuthoringEntry")
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

fn query_runtime_violation_count() -> usize {
    QUERY_RUNTIME_AUDITED_FILES
        .iter()
        .flat_map(|(_, source)| {
            FORBIDDEN_RUNTIME_PATTERNS
                .iter()
                .map(|pattern| source.contains(pattern))
        })
        .filter(|found| *found)
        .count()
}
