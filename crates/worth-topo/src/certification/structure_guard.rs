use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::structure_guard_support::{
    domain_structure_closeout_violations, files_containing_any, production_files_containing_any,
    rust_files, src_relative_path,
};

#[test]
fn topology_crate_skeleton_keeps_the_domain_story() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let src = manifest_dir.join("src");

    let roots = directory_names(&src);
    assert_eq!(
        roots,
        BTreeSet::from([
            "brep".to_string(),
            "certification".to_string(),
            "construction".to_string(),
            "derived_topology".to_string(),
            "projection".to_string(),
            "test_support".to_string(),
            "topology_operators".to_string(),
            "validation".to_string(),
        ])
    );

    assert!(
        !manifest_dir.join("tests").exists(),
        "worth-topo public facade tests must live under certification/public_facade_contracts"
    );
}

#[test]
fn topology_crate_rejects_forbidden_permanent_folder_names() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = [
        "query",
        "edit",
        "fixtures",
        "helpers",
        "utils",
        "common",
        "query_native",
        "query_integration",
        "milestone_three",
        "milestone_two",
        "runtime_invariants",
        "validators",
    ];
    let mut violations = Vec::new();
    collect_forbidden_directory_names(&src, &forbidden, &mut violations);
    assert!(
        violations.is_empty(),
        "forbidden worth-topo skeleton folders remain: {violations:?}"
    );
}

#[test]
fn topology_crate_dependency_direction_stays_domain_shaped() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let violations = dependency_direction_violations(&src);
    assert!(
        violations.is_empty(),
        "worth-topo dependency direction violations remain: {violations:?}"
    );
}

#[test]
fn projection_read_views_remain_decode_and_present_only() {
    let read_views = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("projection")
        .join("read_views");
    let forbidden_meaning = [
        "validate_topology_view",
        "validate_interpreted_topology",
        "validate_materialized_topology",
        "TopologyMaterializer",
        "interpret_topology_view",
        "TopologyOperatorRunner",
        "certify_",
        "repair",
    ];
    let violations = files_containing_any(&read_views, &forbidden_meaning);
    assert!(
        violations.is_empty(),
        "projection/read_views must present decoded products, not infer legality, synthesize derived topology, certify, repair, or execute operators: {violations:?}"
    );
}

#[test]
fn validation_and_certification_keep_their_contract_boundary() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let validation = src.join("validation");
    let certification = src.join("certification");

    let validation_harness_terms = [
        "certify_",
        "closeout",
        "primitive_corpus",
        "hostile_",
        "CertificationSuite",
    ];
    let validation_violations =
        production_files_containing_any(&validation, &validation_harness_terms);
    assert!(
        validation_violations.is_empty(),
        "validation must answer invariant-family validity, not own certification harness logic: {validation_violations:?}"
    );

    let certification_validator_terms = ["pub fn validate_", "fn validate_"];
    let certification_violations =
        production_files_containing_any(&certification, &certification_validator_terms);
    assert!(
        certification_violations.is_empty(),
        "certification must orchestrate proof programs, not define invariant validators: {certification_violations:?}"
    );
}

#[test]
fn migration_map_carries_closeout_discipline() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("worth-topo crate lives under crates/");
    let map_path = workspace_root
        .join("_docs")
        .join("worth")
        .join("worth-topo-domain-structure-migration-map.md");
    let map = fs::read_to_string(&map_path).expect("migration map is readable");
    for required in [
        "## Closeout Status Discipline",
        "Allowed `Closeout status` values",
        "`landed_enforced`",
        "`landed_manual`",
        "`pending`",
        "`blocked_owner_decision`",
        "## Landed Enforcement Inventory",
        "`crates/worth-topo/src/certification/structure_guard.rs`",
    ] {
        assert!(
            map.contains(required),
            "migration map is missing closeout discipline marker `{required}`"
        );
    }
}

#[test]
fn domain_structure_closeout_stays_connected_to_spec_and_roadmap() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("worth-topo crate lives under crates/");
    let violations = domain_structure_closeout_violations(workspace_root);
    assert!(
        violations.is_empty(),
        "domain-structure closeout docs drifted from the closed gate: {violations:?}"
    );
}

#[test]
fn topology_crate_remains_geometry_dependency_pure() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(manifest_dir.join("Cargo.toml"))
        .expect("worth-topo Cargo.toml is readable");
    for forbidden_dependency in ["worth-geom", "worth-math", "forge-topo"] {
        assert!(
            !cargo_toml.contains(forbidden_dependency),
            "worth-topo must not depend on geometry or legacy topology crate `{forbidden_dependency}`"
        );
    }

    let src = manifest_dir.join("src");
    let source_violations =
        production_files_containing_any(&src, &["worth_geom::", "worth_math::", "forge_topo::"]);
    assert!(
        source_violations.is_empty(),
        "worth-topo source must not import geometry or legacy topology crates: {source_violations:?}"
    );
}

#[test]
fn broad_direct_file_clusters_stay_explicitly_reviewed() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed_dense_directories = [
        "certification",
        "certification/public_facade_contracts/compile_fail",
        "construction",
        "derived_topology/materialized_graph",
        "projection/diagnostic_surfaces/read_proof",
        "projection/runtime_boundary/declared_query_surfaces",
        "projection/runtime_boundary/query_runtime",
        "projection/runtime_boundary/query_runtime/tests",
        "validation/reference_integrity",
    ];
    let violations = dense_directory_violations(&src, 8, &allowed_dense_directories);
    assert!(
        violations.is_empty(),
        "new worth-topo flat file clusters need an explicit structural review before they pass: {violations:?}"
    );
}

fn directory_names(path: &Path) -> BTreeSet<String> {
    fs::read_dir(path)
        .expect("directory exists")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect()
}

fn dense_directory_violations(
    src: &Path,
    max_direct_rust_files: usize,
    allowed_relative_directories: &[&str],
) -> Vec<String> {
    let mut violations = Vec::new();
    collect_dense_directory_violations(
        src,
        src,
        max_direct_rust_files,
        allowed_relative_directories,
        &mut violations,
    );
    violations
}

fn collect_dense_directory_violations(
    src: &Path,
    path: &Path,
    max_direct_rust_files: usize,
    allowed_relative_directories: &[&str],
    violations: &mut Vec<String>,
) {
    let direct_rust_file_count = fs::read_dir(path)
        .expect("directory exists")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_ok_and(|kind| kind.is_file())
                && entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "rs")
        })
        .count();
    let relative = path
        .strip_prefix(src)
        .expect("directory lives under src")
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    if direct_rust_file_count > max_direct_rust_files
        && !allowed_relative_directories.contains(&relative.as_str())
    {
        violations.push(format!(
            "{relative}: {direct_rust_file_count} direct Rust files"
        ));
    }
    for entry in fs::read_dir(path).expect("directory exists") {
        let entry = entry.expect("directory entry is readable");
        if entry.file_type().expect("file type is readable").is_dir() {
            collect_dense_directory_violations(
                src,
                &entry.path(),
                max_direct_rust_files,
                allowed_relative_directories,
                violations,
            );
        }
    }
}

fn dependency_direction_violations(src: &Path) -> Vec<String> {
    let mut violations = Vec::new();
    collect_forbidden_imports(
        &src.join("brep"),
        &[
            "crate::certification",
            "crate::derived_topology",
            "crate::projection",
            "crate::test_support",
            "crate::topology_operators",
            "crate::validation",
        ],
        &[],
        &mut violations,
    );
    collect_forbidden_imports(
        &src.join("derived_topology"),
        &[
            "crate::certification",
            "crate::projection",
            "crate::topology_operators",
            "crate::validation",
        ],
        &[],
        &mut violations,
    );
    collect_forbidden_imports(
        &src.join("validation"),
        &[
            "crate::certification",
            "crate::projection",
            "crate::topology_operators",
        ],
        &[],
        &mut violations,
    );
    collect_forbidden_imports(
        &src.join("topology_operators"),
        &["crate::certification", "crate::projection"],
        &[
            "topology_operators/application/admission.rs",
            "topology_operators/application/bindings.rs",
            "topology_operators/application/existing_truth.rs",
            "topology_operators/application/mod.rs",
            "topology_operators/local_rewrites/boundary_wiring/adjacency_support.rs",
            "topology_operators/local_rewrites/boundary_wiring/composed_successor_program.rs",
            "topology_operators/local_rewrites/boundary_wiring/membership.rs",
            "topology_operators/local_rewrites/boundary_wiring/relation_update.rs",
            "topology_operators/local_rewrites/boundary_wiring/successor_admission.rs",
            "topology_operators/local_rewrites/boundary_wiring/successor_support.rs",
            "topology_operators/local_rewrites/entity_lifecycle/relation_create.rs",
            "topology_operators/local_rewrites/radial_cycles/splice_adjacency.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/membership_admission.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/face_inner_loop_program.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/mod.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/shared.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/shell_membership_program.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/wire_membership_program.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/shell_face_rehome.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/shell_face_rehome_support.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/shell_face_split.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/wire_rehome.rs",
            "topology_operators/local_rewrites/sheet_wire_laminar/wire_rehome_support.rs",
        ],
        &mut violations,
    );
    collect_forbidden_imports(
        &src.join("projection").join("read_views"),
        &[
            "crate::certification",
            "crate::derived_topology",
            "crate::topology_operators",
            "crate::validation",
        ],
        &[],
        &mut violations,
    );
    violations
}

fn collect_forbidden_imports(
    path: &Path,
    forbidden_imports: &[&str],
    allowed_relative_files: &[&str],
    violations: &mut Vec<String>,
) {
    for file in rust_files(path) {
        let relative = src_relative_path(&file);
        if allowed_relative_files.contains(&relative.as_str()) {
            continue;
        }
        let text = fs::read_to_string(&file).expect("rust source is readable");
        for forbidden in forbidden_imports {
            if text.contains(forbidden) {
                violations.push(format!("{relative} imports {forbidden}"));
            }
        }
    }
}

fn collect_forbidden_directory_names(
    path: &Path,
    forbidden: &[&str],
    violations: &mut Vec<String>,
) {
    for entry in fs::read_dir(path).expect("directory exists") {
        let entry = entry.expect("directory entry is readable");
        if !entry.file_type().expect("file type is readable").is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if forbidden.contains(&name.as_str()) {
            violations.push(entry.path().display().to_string());
        }
        collect_forbidden_directory_names(&entry.path(), forbidden, violations);
    }
}
