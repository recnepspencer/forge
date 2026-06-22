use super::*;

#[test]
fn touched_graph_inventory_rejects_missing_static_invariant_category() {
    let touched_inventory = current_worth_touched_graph_authority_inventory()
        .into_iter()
        .filter(|row| {
            row.category() != WorthTouchedGraphAuthorityInventoryCategory::StaticInvariant
        })
        .collect::<Vec<_>>();

    let violation = certify_with_touched_inputs(
        touched_inventory,
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
    )
    .expect_err("static/global invariant coverage must be typed, not prose-only");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::MissingTouchedGraphInventoryCategory(
            WorthTouchedGraphAuthorityInventoryCategory::StaticInvariant
        )
    );
}

#[test]
fn touched_graph_inventory_rejects_unclassified_static_authority_entry() {
    let mut static_authority_entries =
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries();
    static_authority_entries.push(
        touched_graph_static_authority::WorthTouchedGraphStaticAuthorityEntry::new(
            "topology.validation.rule.unclassified",
            "crates/worth-topo/src/validation/unclassified",
            WorthTouchedGraphAuthorityInventoryCategory::StaticInvariant,
            "unclassified",
            "DERIVED_TOPOLOGY_RULE_SPECS",
        ),
    );

    let violation = certify_with_touched_inputs(
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        static_authority_entries,
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
    )
    .expect_err("a static authority entry without its own touched inventory row must fail");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::MissingTouchedGraphStaticAuthorityInventoryRow(
            "topology.validation.rule.unclassified"
        )
    );
}

#[test]
fn topology_rule_specs_have_static_authority_inventory_rows() {
    let registry_rule_names = topology_validation_rule_names_from_source();
    let static_authority_entries =
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries();
    let touched_inventory = current_worth_touched_graph_authority_inventory();

    for registry_rule_name in registry_rule_names {
        let source_id = format!("topology.validation.rule.{registry_rule_name}");
        assert!(
            static_authority_entries
                .iter()
                .any(|entry| entry.source_id() == source_id),
            "missing static authority manifest entry for {source_id}"
        );
        assert!(
            touched_inventory
                .iter()
                .any(|row| row.source_id() == source_id),
            "missing touched inventory row for {source_id}"
        );
    }
}

#[test]
fn touched_graph_deletion_rejects_self_declared_ordinary_facade_export() {
    let mut touched_deletion_ledger = current_worth_touched_graph_deletion_ledger();
    touched_deletion_ledger.push(WorthTouchedGraphAuthorityDeletionLedgerRow::new(
        "bad.public-collapse",
        "topology.validation.rule-registry",
        "crates/worth-topo/src/validation/rule_registry.rs",
        "worth-topo",
        WorthTouchedGraphAuthorityDisposition::Collapse,
        "DERIVED_TOPOLOGY_RULE_SPECS",
        "collapse row still exported through ordinary validation facade",
        "Phase 5 touched validator predicate buckets",
        "Phase 5 validator derivation lands.",
        "worth_topo::validation::validate_interpreted_topology",
        "negative test",
    ));

    let violation = certify_with_touched_inputs(
        current_worth_touched_graph_authority_inventory(),
        touched_deletion_ledger,
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
    )
    .expect_err("delete/collapse rows must not self-declare ordinary public facade exports");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::TouchedGraphDeletionStillOrdinaryPublicFacade(
            "bad.public-collapse"
        )
    );
}

#[test]
fn touched_graph_deletion_rejects_actual_facade_export_even_when_row_claims_sealed() {
    let mut touched_deletion_ledger = current_worth_touched_graph_deletion_ledger();
    touched_deletion_ledger.push(WorthTouchedGraphAuthorityDeletionLedgerRow::new(
        "bad.actual-public-collapse",
        "cross-crate.public-facades",
        "crates/worth-topo/src/validation/mod.rs",
        "worth-topo",
        WorthTouchedGraphAuthorityDisposition::Collapse,
        "validate_interpreted_topology",
        "collapse row falsely claims the public validation facade is sealed",
        "Phase 5 touched validator predicate buckets",
        "Phase 5 validator derivation lands.",
        touched_graph_inventory::SEALED_FROM_ORDINARY_FACADE,
        "negative test",
    ));

    let violation = certify_with_touched_inputs(
        current_worth_touched_graph_authority_inventory(),
        touched_deletion_ledger,
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
    )
    .expect_err("actual public facade exports must fail delete/collapse certification");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::TouchedGraphDeletionStillExportedByFacade(
            "bad.actual-public-collapse"
        )
    );
}

#[test]
fn touched_graph_residue_rejects_false_no_ordinary_public_facade_posture() {
    let touched_inventory = current_worth_touched_graph_authority_inventory()
        .into_iter()
        .map(|row| {
            if row.source_id() == "topology.operator-intent-blueprints" {
                WorthTouchedGraphAuthorityInventoryRow::new(
                    row.source_id(),
                    row.source_path(),
                    row.category(),
                    row.owner(),
                    row.current_authority_source(),
                    row.touched_graph_replacement(),
                    row.disposition(),
                    row.residue_cap(),
                    row.removal_trigger(),
                    touched_graph_inventory::NO_ORDINARY_PUBLIC_FACADE,
                    "negative test falsely hides public topology operator facade",
                )
            } else {
                row
            }
        })
        .collect::<Vec<_>>();

    let violation = certify_with_touched_inputs(
        touched_inventory,
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
    )
    .expect_err("public residue source must name capped public facade posture");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::TouchedGraphResiduePublicFacadePostureMismatch(
            "topology.operator-intent-blueprints"
        )
    );
}

#[test]
fn touched_graph_gate_files_satisfy_workspace_line_cap() {
    let workspace_root = workspace_root();
    let touched_files = phase_touched_rust_files(&workspace_root);

    assert!(
        !touched_files.is_empty(),
        "phase-touched Rust file set must be derived from git status"
    );
    assert!(
        touched_files.iter().any(|path| path
            == "crates/worth-spatial/src/workload_platform/planar_boolean_edge_splitting/edge_split_request/tests.rs"),
        "line-cap proof must include touched Rust files outside query_graph_authority_gate"
    );
    assert!(
        touched_files.iter().any(
            |path| path == "crates/worth-kernel/src/query_graph_authority_gate/closeout_doc.rs"
        ),
        "line-cap proof must include all Rust files from untracked gate directories"
    );

    for touched_file in touched_files {
        let contents = std::fs::read_to_string(workspace_root.join(&touched_file))
            .unwrap_or_else(|error| panic!("failed to read {touched_file}: {error}"));
        let line_count = contents.lines().count();
        assert!(
            line_count <= 400,
            "{touched_file} has {line_count} lines and exceeds the workspace cap"
        );
    }
}

fn certify_with_touched_inputs(
    touched_graph_inventory: Vec<WorthTouchedGraphAuthorityInventoryRow>,
    touched_graph_deletion_ledger: Vec<WorthTouchedGraphAuthorityDeletionLedgerRow>,
    static_authority_entries: Vec<
        touched_graph_static_authority::WorthTouchedGraphStaticAuthorityEntry,
    >,
    ordinary_public_facade_exports: Vec<
        touched_graph_facade_audit::WorthTouchedGraphOrdinaryPublicFacadeExport,
    >,
) -> Result<WorthGraphAuthorityGateReport, WorthGraphAuthorityGateViolation> {
    certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        current_worth_graph_authority_deletion_ledger(),
        touched_graph_inventory,
        touched_graph_deletion_ledger,
        static_authority_entries,
        ordinary_public_facade_exports,
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
}

fn phase_touched_rust_files(workspace_root: &std::path::Path) -> Vec<String> {
    let output = std::process::Command::new("git")
        .current_dir(workspace_root)
        .args(["status", "--short", "--"])
        .args(PHASE_SCOPE_PATHS)
        .output()
        .expect("git status must be available for touched-file line-cap proof");
    assert!(
        output.status.success(),
        "git status failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mut files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .flat_map(|line| phase_touched_rust_files_from_status_line(workspace_root, line))
        .collect::<Vec<_>>();
    files.sort();
    files.dedup();
    files
}

fn phase_touched_rust_files_from_status_line(
    workspace_root: &std::path::Path,
    line: &str,
) -> Vec<String> {
    let Some(path) = line.get(3..) else {
        return Vec::new();
    };
    let path = path.trim().trim_matches('"').replace('\\', "/");
    if path.is_empty() || line.starts_with(" D") || line.starts_with("D ") {
        return Vec::new();
    }
    let absolute_path = workspace_root.join(&path);
    if absolute_path.is_dir() {
        return rust_files_under_directory(workspace_root, &absolute_path);
    }
    if path.ends_with(".rs") && absolute_path.exists() {
        return vec![path];
    }
    Vec::new()
}

fn rust_files_under_directory(
    workspace_root: &std::path::Path,
    directory: &std::path::Path,
) -> Vec<String> {
    let mut pending = vec![directory.to_path_buf()];
    let mut files = Vec::new();
    while let Some(current) = pending.pop() {
        for entry in std::fs::read_dir(&current)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", current.display()))
        {
            let entry = entry.expect("directory entry should be readable");
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(relative_workspace_path(workspace_root, &path));
            }
        }
    }
    files
}

fn relative_workspace_path(workspace_root: &std::path::Path, path: &std::path::Path) -> String {
    path.strip_prefix(workspace_root)
        .expect("touched file should be under workspace root")
        .to_string_lossy()
        .replace('\\', "/")
}

fn topology_validation_rule_names_from_source() -> Vec<String> {
    let registry_path = workspace_root().join("crates/worth-topo/src/validation/rule_registry.rs");
    let registry_source = std::fs::read_to_string(&registry_path).unwrap_or_else(|error| {
        panic!(
            "failed to read topology validation rule registry {}: {error}",
            registry_path.display()
        )
    });
    let mut names = registry_source
        .lines()
        .filter_map(topology_validation_rule_name_from_line)
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn topology_validation_rule_name_from_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let raw_name = trimmed.strip_prefix("name: \"")?.strip_suffix("\",")?;
    Some(raw_name.to_string())
}

const PHASE_SCOPE_PATHS: &[&str] = &[
    "crates/worth-topo/src/topology_operators",
    "crates/worth-topo/src/validation",
    "crates/worth-topo/src/projection",
    "crates/worth-spatial/src/workload_platform",
    "crates/worth-kernel/src/query_graph_authority_gate",
    "crates/forge-query/src/runtime/mutation/graph_composition",
];

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-kernel lives two levels below workspace root")
        .to_path_buf()
}
