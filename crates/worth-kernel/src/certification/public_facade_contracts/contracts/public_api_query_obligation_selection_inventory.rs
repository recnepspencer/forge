use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use worth_kernel::query_obligation_selection::boundary_inventory::{
    query_selection_boundary_inventory, validate_query_selection_boundary_inventory,
    QuerySelectionAuthorityPosture, QuerySelectionBoundaryInventory,
    QuerySelectionBoundaryInventoryRow, QuerySelectionDeletionAction,
    QuerySelectionInventoryFindingKind, QuerySelectionProofStrength,
    QuerySelectionSurfaceClassification, QuerySelectionSurfaceOwner,
};

#[test]
fn query_selection_inventory_covers_every_graph_obligation_surface() {
    let inventory = query_selection_boundary_inventory();
    let findings = validate_query_selection_boundary_inventory(&inventory);
    assert!(findings.is_empty(), "inventory findings: {findings:#?}");

    let duplicates = duplicated_surfaces(&inventory);
    assert!(
        duplicates.is_empty(),
        "inventory surfaces must be classified exactly once: {duplicates:#?}"
    );

    assert_scanned_source_counts_are_classified(&inventory);

    assert_required_surface_owner(
        &inventory,
        "graph_obligation_consumer_kit",
        QuerySelectionSurfaceOwner::ForgeQuery,
        QuerySelectionDeletionAction::KeepAsQueryOwnedSelection,
    );
    assert_deleted_surface(
        &inventory,
        "primitive_construction_graph_obligation_adoption_proof",
    );
    assert_required_surface(
        &inventory,
        "topology_operator_graph_obligation_adoption_proof",
        QuerySelectionProofStrength::InMemorySelection,
        QuerySelectionDeletionAction::MigrateToParallelSelectionSubstrate,
    );
    assert_required_surface(
        &inventory,
        "spatial_query_graph_obligation_adoption_proof_for_descriptor",
        QuerySelectionProofStrength::ExecutionBackedAdoption,
        QuerySelectionDeletionAction::KeepAsQueryOwnedSelection,
    );
}

#[test]
fn unclassified_selector_surface_fails_inventory() {
    let inventory =
        QuerySelectionBoundaryInventory::new(vec![QuerySelectionBoundaryInventoryRow::new(
            "",
            None,
            "unclassified_selector_surface",
            QuerySelectionSurfaceClassification::MigrationProjection,
            QuerySelectionAuthorityPosture::SelectorCoverageDeclaration,
            QuerySelectionProofStrength::RegistrationOnly,
            "",
            QuerySelectionDeletionAction::CertificationOnly,
            QuerySelectionSurfaceOwner::WorthKernel,
            None,
            None,
            None,
        )]);

    let finding_kinds = validate_query_selection_boundary_inventory(&inventory)
        .into_iter()
        .map(|finding| finding.kind())
        .collect::<BTreeSet<_>>();

    assert!(finding_kinds.contains(&QuerySelectionInventoryFindingKind::MissingSourcePath));
    assert!(finding_kinds.contains(&QuerySelectionInventoryFindingKind::MissingCurrentCaller));
    assert!(finding_kinds.contains(
        &QuerySelectionInventoryFindingKind::MigrationSurfaceWithoutExplicitDeletionAction
    ));
}

#[test]
fn inventory_rejects_in_memory_adoption_as_execution_proof() {
    let inventory =
        QuerySelectionBoundaryInventory::new(vec![QuerySelectionBoundaryInventoryRow::new(
            "crates/worth-kernel/src/query_obligation_selection/selection_substrate/selected_obligations.rs",
            None,
            "primitive_construction_graph_obligation_adoption_proof",
            QuerySelectionSurfaceClassification::MigrationProjection,
            QuerySelectionAuthorityPosture::ExecutionBackedSelectionAdoption,
            QuerySelectionProofStrength::InMemorySelection,
            "worth-kernel primitive construction graph-obligation adoption",
            QuerySelectionDeletionAction::MigrateToParallelSelectionSubstrate,
            QuerySelectionSurfaceOwner::WorthKernel,
            None,
            None,
            None,
        )]);

    assert_has_finding(
        &inventory,
        QuerySelectionInventoryFindingKind::InMemorySelectionPromotedAsExecutionAuthority,
    );
}

#[test]
fn support_pin_and_local_ceremony_are_not_selector_authority() {
    let inventory = QuerySelectionBoundaryInventory::new(vec![
        QuerySelectionBoundaryInventoryRow::new(
            "crates/worth-kernel/src/construction/graph_obligation_adoption/catalog.rs",
            None,
            "primitive_construction_graph_obligation_support_pin",
            QuerySelectionSurfaceClassification::QueryOwnedSelection,
            QuerySelectionAuthorityPosture::SupportPin,
            QuerySelectionProofStrength::SupportOnly,
            "worth-kernel primitive construction graph-obligation adoption",
            QuerySelectionDeletionAction::KeepAsQueryOwnedSelection,
            QuerySelectionSurfaceOwner::WorthKernel,
            None,
            None,
            None,
        ),
        QuerySelectionBoundaryInventoryRow::new(
            "crates/worth-kernel/src/construction/graph_obligation_adoption/residue.rs",
            Some("worth_kernel::query_obligation_selection"),
            "primitive_construction_graph_obligation_local_ceremony_audit",
            QuerySelectionSurfaceClassification::QueryOwnedSelection,
            QuerySelectionAuthorityPosture::LocalCeremonyAudit,
            QuerySelectionProofStrength::LocalCeremonyOnly,
            "worth-kernel primitive construction graph-obligation adoption",
            QuerySelectionDeletionAction::KeepAsQueryOwnedSelection,
            QuerySelectionSurfaceOwner::WorthKernel,
            None,
            None,
            None,
        ),
    ]);

    assert_has_finding(
        &inventory,
        QuerySelectionInventoryFindingKind::SupportSurfaceMarkedAsSelectedAuthority,
    );
    assert_has_finding(
        &inventory,
        QuerySelectionInventoryFindingKind::LocalCeremonyMarkedAsSelectedAuthority,
    );
    assert_has_finding(
        &inventory,
        QuerySelectionInventoryFindingKind::PublicLocalCeremonyExport,
    );
}

#[test]
fn selector_residue_rows_require_owner_cap_and_removal_trigger() {
    let inventory =
        QuerySelectionBoundaryInventory::new(vec![QuerySelectionBoundaryInventoryRow::new(
            "crates/worth-spatial/src/query_adoption/consumer_kit.rs",
            Some("worth_spatial::facade::query_adoption"),
            "spatial_query_graph_obligation_residue_manifest",
            QuerySelectionSurfaceClassification::CappedResidue,
            QuerySelectionAuthorityPosture::ResidueManifest,
            QuerySelectionProofStrength::ResidueOnly,
            "worth-spatial query adoption Consumer Kit",
            QuerySelectionDeletionAction::CappedResidue,
            QuerySelectionSurfaceOwner::WorthSpatial,
            None,
            None,
            None,
        )]);

    assert_has_finding(
        &inventory,
        QuerySelectionInventoryFindingKind::MissingCapForCappedResidue,
    );
    assert_has_finding(
        &inventory,
        QuerySelectionInventoryFindingKind::MissingBlockerForCappedResidue,
    );
    assert_has_finding(
        &inventory,
        QuerySelectionInventoryFindingKind::MissingRemovalTriggerForCappedResidue,
    );
}

fn assert_required_surface_owner(
    inventory: &QuerySelectionBoundaryInventory,
    surface: &str,
    owner: QuerySelectionSurfaceOwner,
    action: QuerySelectionDeletionAction,
) {
    let row = inventory
        .row_named(surface)
        .unwrap_or_else(|| panic!("missing required surface {surface}"));
    assert_eq!(row.owner(), owner);
    assert_eq!(row.deletion_action(), action);
}

fn assert_required_surface(
    inventory: &QuerySelectionBoundaryInventory,
    surface: &str,
    proof: QuerySelectionProofStrength,
    action: QuerySelectionDeletionAction,
) {
    let row = inventory
        .row_named(surface)
        .unwrap_or_else(|| panic!("missing required surface {surface}"));
    assert_eq!(row.proof_strength(), proof);
    assert_eq!(row.deletion_action(), action);
}

fn assert_deleted_surface(inventory: &QuerySelectionBoundaryInventory, surface: &str) {
    assert!(
        inventory.row_named(surface).is_none(),
        "deleted surface {surface} must not remain in the live query selection inventory"
    );
}

fn assert_has_finding(
    inventory: &QuerySelectionBoundaryInventory,
    expected: QuerySelectionInventoryFindingKind,
) {
    let findings = validate_query_selection_boundary_inventory(inventory);
    assert!(
        findings.iter().any(|finding| finding.kind() == expected),
        "expected {expected:?}, got {findings:#?}"
    );
}

fn duplicated_surfaces(inventory: &QuerySelectionBoundaryInventory) -> BTreeSet<&'static str> {
    let mut counts = BTreeMap::new();
    for row in inventory.rows() {
        *counts.entry(row.surface()).or_insert(0usize) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(surface, count)| (count > 1).then_some(surface))
        .collect()
}

fn assert_scanned_source_counts_are_classified(inventory: &QuerySelectionBoundaryInventory) {
    let scanned_counts = scanned_surface_counts_by_path();
    let inventory_counts = inventory_surface_counts_by_path(inventory);
    let missing_paths = scanned_counts
        .keys()
        .filter(|path| !inventory_counts.contains_key(*path))
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        missing_paths.is_empty(),
        "graph-obligation source paths missing inventory rows: {missing_paths:#?}"
    );

    let mismatched_counts = scanned_counts
        .iter()
        .filter_map(|(path, scanned_count)| {
            let inventory_count = inventory_counts.get(path).copied().unwrap_or_default();
            (inventory_count < *scanned_count).then_some((
                path.clone(),
                *scanned_count,
                inventory_count,
            ))
        })
        .collect::<Vec<_>>();
    assert!(
        mismatched_counts.is_empty(),
        "graph-obligation source counts must match inventory rows: {mismatched_counts:#?}"
    );
}

fn scanned_surface_counts_by_path() -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for source_file in scanned_rust_files() {
        let repo_path = repo_relative_path(&source_file);
        let contents = fs::read_to_string(&source_file)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", source_file.display()));
        let count = contents
            .lines()
            .filter(|line| line_declares_graph_obligation_inventory_surface(line))
            .count();
        if count > 0 {
            counts.insert(repo_path, count);
        }
    }
    counts
}

fn inventory_surface_counts_by_path(
    inventory: &QuerySelectionBoundaryInventory,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in inventory.rows() {
        *counts
            .entry(row.source_path().to_string())
            .or_insert(0usize) += 1;
    }
    counts
}

fn scanned_rust_files() -> Vec<PathBuf> {
    SCANNED_ROOTS
        .iter()
        .flat_map(|root| rust_files_under(&repo_path(root)))
        .filter(|path| !path_is_test_or_certification_support(path))
        .collect::<Vec<_>>()
}

fn rust_files_under(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = fs::read_dir(root)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", root.display()));
    for entry in entries {
        let path = entry.expect("directory entry should be readable").path();
        if path.is_dir() {
            files.extend(rust_files_under(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    files
}

fn line_declares_graph_obligation_inventory_surface(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("pub")
        && !trimmed.starts_with("pub use")
        && !trimmed.contains(" use ")
        && !trimmed.starts_with("use ")
        && GRAPH_OBLIGATION_SURFACE_TOKENS
            .iter()
            .any(|token| trimmed.contains(token))
}

fn path_is_test_or_certification_support(path: &Path) -> bool {
    let path_text = path.to_string_lossy();
    path_text.contains("_tests")
        || path_text.contains("\\tests\\")
        || path_text.contains("/tests/")
        || path_text.contains("certification")
}

fn repo_relative_path(path: &Path) -> String {
    path.strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn repo_path(relative: &str) -> PathBuf {
    repo_root().join(relative)
}

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-kernel manifest should live under crates/worth-kernel")
}

const SCANNED_ROOTS: &[&str] = &[
    "crates/worth-kernel/src/construction/graph_obligation_adoption",
    "crates/worth-kernel/src/construction/query_authority",
    "crates/worth-kernel/src/construction/result_surface",
    "crates/worth-topo/src/construction/query_native_boundary",
    "crates/worth-topo/src/topology_operators",
    "crates/worth-spatial/src/query_adoption",
    "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission",
];

const GRAPH_OBLIGATION_SURFACE_TOKENS: &[&str] = &[
    "graph_obligation",
    "GraphAuthority",
    "DeclaredTouchedBasis",
    "SpatialEvidenceQueryTouchDescriptor",
];
