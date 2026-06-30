use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::catalog::current_compiled_product_reuse_inventory;
use super::classification::{
    CompiledProductReuseDisposition, CompiledProductReuseSemanticCategory,
    CompiledProductReuseSemanticDistinction,
};
use super::closeout::CompiledProductReuseInventoryCloseout;
use super::error::CompiledProductReuseInventoryError;
use super::report::CompiledProductReuseInventoryReport;
use super::row::CompiledProductReuseSurfaceIdentity as Surface;
use super::source_scan::CompiledProductReuseScanScopeReport;

const SCAN_ROOT_BASELINES: [(&str, &str); 10] = [
    (
        "crates/worth-kernel/src/workload_composition/public_closeout/baseline.rs",
        "pub fn public_closeout_baseline() {}\n",
    ),
    (
        "crates/worth-kernel/src/workload_composition/worth_workload/baseline.rs",
        "pub fn worth_workload_baseline() {}\n",
    ),
    (
        "crates/worth-kernel/src/replay_undo_consumer_cutover/public_closeout/baseline.rs",
        "pub fn replay_undo_public_closeout_baseline() {}\n",
    ),
    (
        "crates/worth-topo/src/derived_topology/invalidation_plan/catalog/baseline.rs",
        "pub fn invalidation_catalog_baseline() {}\n",
    ),
    (
        "crates/worth-topo/src/derived_topology/invalidation_plan/selection/baseline.rs",
        "pub fn invalidation_selection_baseline() {}\n",
    ),
    (
        "crates/worth-topo/src/projection/diagnostic_surfaces/baseline.rs",
        "pub fn diagnostic_surface_baseline() {}\n",
    ),
    (
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/baseline.rs",
        "pub fn read_execution_baseline() {}\n",
    ),
    (
        "crates/worth-spatial/src/workload_platform/evidence_lookup_index_product/baseline.rs",
        "pub fn evidence_lookup_index_baseline() {}\n",
    ),
    (
        "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/baseline.rs",
        "pub fn evidence_lookup_public_closeout_baseline() {}\n",
    ),
    (
        "crates/worth-spatial/src/workload_platform/retained_replay_workload/baseline.rs",
        "pub fn retained_replay_baseline() {}\n",
    ),
];

const EXCLUDED_SCOPE_FIXTURES: [(&str, &str); 3] = [
    (
        "crates/worth-kernel/src/workload_composition/source_firewall/ignored.rs",
        "pub fn ignored_source_firewall() {}\n",
    ),
    (
        "crates/worth-kernel/src/workload_composition/compiled_product_reuse_inventory/ignored.rs",
        "pub fn ignored_inventory_lane() {}\n",
    ),
    (
        "crates/worth-spatial/src/workload_platform/retained_replay_workload/tests/ignored.rs",
        "pub fn ignored_test_dir() {}\n",
    ),
];

const EXPECTED_SURFACES: [Surface; 23] = [
    Surface::BuildDerivedEquivalenceContract,
    Surface::BuildDerivedEquivalenceContractReport,
    Surface::CompareDerivedEquivalenceContracts,
    Surface::DerivedInvalidationPlannedDispositionFromUpdatePosture,
    Surface::HistoricalPathReuseDescriptorRetainedReuse,
    Surface::HistoricalCapabilityDescriptorRetainedReuse,
    Surface::ReuseEvidenceLookupIndexProduct,
    Surface::IndexProductDigest,
    Surface::ReplayParityReportFromRetainedProjectionMatch,
    Surface::ReplayParityReportRowCount,
    Surface::RetainedArtifactCaptureReceiptFromArtifacts,
    Surface::ReplayWorkloadWithCapturedRetainedWorkload,
    Surface::ReplayCaptureReceipt,
    Surface::LookupConsumedWorkloadCompositionAdmit,
    Surface::WorthWorkloadAdmitLookupConsumedWorkload,
    Surface::WorthWorkloadAdmitLookupConsumedBatchExecutionCluster,
    Surface::CurrentEvidenceLookupPublicCloseout,
    Surface::CurrentEvidenceLookupPublicCloseoutAssemblyInput,
    Surface::CurrentWorthWorkloadOrdinaryConsumerCutover,
    Surface::CurrentWorthTouchedGraphConflictPublicCloseout,
    Surface::CurrentWorthTouchedGraphConflictMilestoneFourteenSeed,
    Surface::ReplayUndoPublicCloseoutReadModelProjection,
    Surface::KernelConflictPublicCloseoutBoundaryTraceability,
];

const HOSTILE_CASES: [FixtureCase; 5] = [
    FixtureCase::new(
        "ordinary cache key",
        "crates/worth-spatial/src/workload_platform/evidence_lookup_public_closeout/uncovered_cache_key.rs",
        "pub fn public_closeout_cache_key() -> &'static str { \"cache\" }\n",
        "reuse helper identifier",
    ),
    FixtureCase::new(
        "row-count shortcut",
        "crates/worth-spatial/src/workload_platform/retained_replay_workload/uncovered_row_count.rs",
        "pub fn same_row_count(left_row_count: usize, right_row_count: usize) -> bool { left_row_count == right_row_count }\n",
        "row-count shortcut line",
    ),
    FixtureCase::new(
        "rendered-shape equality",
        "crates/worth-topo/src/projection/diagnostic_surfaces/uncovered_shape.rs",
        "pub fn compare_rendered_shape() -> bool { rendered_shape == expected_shape }\n",
        "rendered-shape equality line",
    ),
    FixtureCase::new(
        "pointer identity",
        "crates/worth-kernel/src/workload_composition/worth_workload/uncovered_pointer_identity.rs",
        "pub fn same_pointer(left: &Arc<String>, right: &Arc<String>) -> bool { Arc::ptr_eq(left, right) }\n",
        "pointer identity line",
    ),
    FixtureCase::new(
        "retained folklore helper",
        "crates/worth-spatial/src/workload_platform/retained_replay_workload/uncovered_retained_helper.rs",
        "pub fn retained_capture_helper() -> &'static str { \"retained\" }\n",
        "retained folklore identifier",
    ),
];

#[test]
fn current_inventory_explicitly_covers_phase_one_surface() {
    let inventory = current_compiled_product_reuse_inventory().expect("inventory builds");
    let closeout = CompiledProductReuseInventoryCloseout::close(inventory.clone())
        .expect("inventory closeout");

    assert_eq!(closeout.source_scan().uncovered_pattern_count(), 0);
    assert_eq!(inventory.rows().len(), EXPECTED_SURFACES.len());
    assert_surface_coverage(&inventory);
    assert_category_coverage(&inventory);
    assert_distinction_coverage(&inventory);
    assert_disposition_coverage(&inventory);
}

#[test]
fn source_scan_scope_tracks_real_roots_and_exclusions() {
    let fixture = create_fixture_workspace(None);
    let scope_report = CompiledProductReuseScanScopeReport::from_workspace_root(&fixture.root)
        .expect("scope report");

    assert_eq!(scope_report.scanned_file_count(), SCAN_ROOT_BASELINES.len());
    for expected_path in fixture.baseline_paths() {
        assert!(
            scope_report
                .scanned_relative_paths()
                .contains(expected_path),
            "missing scoped baseline path `{expected_path}`"
        );
    }
    for excluded_path in fixture.excluded_paths() {
        assert!(
            !scope_report
                .scanned_relative_paths()
                .contains(excluded_path),
            "excluded path should not be scanned: `{excluded_path}`"
        );
    }

    fixture.remove();
}

#[test]
fn ordinary_rows_cannot_cap() {
    let inventory = current_compiled_product_reuse_inventory().expect("inventory builds");
    let mut rows = inventory.rows().to_vec();
    let index = rows
        .iter()
        .position(|row| row.ordinary_path())
        .expect("ordinary row");
    let row = rows[index].clone();
    rows[index] = super::row::CompiledProductReuseInventoryRow::new(
        row.surface_identity(),
        row.source_path(),
        row.surface_name(),
        row.authority_surface(),
        row.semantic_category(),
        row.semantic_distinction(),
        row.old_authority_kind(),
        CompiledProductReuseDisposition::Cap,
        row.owner(),
        row.replacement_phase(),
        row.blocker(),
        row.removal_trigger(),
        true,
        false,
        Some(1),
        row.scan_pattern(),
        row.secondary_scan_pattern(),
    );

    let error = CompiledProductReuseInventoryCloseout::close(
        CompiledProductReuseInventoryReport::new(rows),
    )
    .expect_err("ordinary cap must fail");
    assert!(matches!(
        error,
        CompiledProductReuseInventoryError::InvalidOrdinaryDisposition { .. }
    ));
}

#[test]
fn closeout_rejects_uncovered_folklore_growth_in_mixed_scope_tree() {
    let inventory = current_compiled_product_reuse_inventory().expect("inventory builds");
    for case in HOSTILE_CASES {
        let fixture = create_fixture_workspace(Some(case));
        let error = CompiledProductReuseInventoryCloseout::close_with_workspace_root(
            inventory.clone(),
            fixture.root(),
        )
        .expect_err(case.name);
        assert_uncovered_pattern(error, case.relative_path, case.expected_pattern);
        fixture.remove();
    }
}

#[test]
fn closeout_ignores_cardinality_completeness_checks() {
    let inventory = current_compiled_product_reuse_inventory().expect("inventory builds");
    let fixture = create_fixture_workspace(Some(FixtureCase::new(
        "cardinality completeness check",
        "crates/worth-kernel/src/replay_undo_consumer_cutover/public_closeout/closeout.rs",
        "pub fn require_complete_classification(expected_row_count: usize, inventory_rows: &[usize]) -> bool { inventory_rows.len() == expected_row_count }\n",
        "row-count shortcut line",
    )));

    CompiledProductReuseInventoryCloseout::close_with_workspace_root(inventory, fixture.root())
        .expect("cardinality completeness checks are not pseudo-reuse shortcuts");

    fixture.remove();
}

#[derive(Clone, Copy)]
struct FixtureCase {
    name: &'static str,
    relative_path: &'static str,
    contents: &'static str,
    expected_pattern: &'static str,
}

impl FixtureCase {
    const fn new(
        name: &'static str,
        relative_path: &'static str,
        contents: &'static str,
        expected_pattern: &'static str,
    ) -> Self {
        Self {
            name,
            relative_path,
            contents,
            expected_pattern,
        }
    }
}

struct FixtureWorkspace {
    root: PathBuf,
    baseline_paths: Vec<String>,
    excluded_paths: Vec<String>,
}

impl FixtureWorkspace {
    fn root(&self) -> &Path {
        &self.root
    }

    fn baseline_paths(&self) -> &[String] {
        &self.baseline_paths
    }

    fn excluded_paths(&self) -> &[String] {
        &self.excluded_paths
    }

    fn remove(self) {
        fs::remove_dir_all(self.root).expect("fixture workspace removed");
    }
}

fn assert_surface_coverage(inventory: &CompiledProductReuseInventoryReport) {
    let observed = inventory
        .rows()
        .iter()
        .map(|row| row.surface_identity())
        .collect::<BTreeSet<_>>();
    for surface in EXPECTED_SURFACES {
        assert!(observed.contains(&surface), "missing surface {:?}", surface);
    }
}

fn assert_category_coverage(inventory: &CompiledProductReuseInventoryReport) {
    for category in CompiledProductReuseSemanticCategory::REQUIRED_COVERED {
        assert!(
            inventory
                .rows()
                .iter()
                .any(|row| row.semantic_category() == category),
            "missing category `{}`",
            category.as_str()
        );
    }
}

fn assert_distinction_coverage(inventory: &CompiledProductReuseInventoryReport) {
    for distinction in [
        CompiledProductReuseSemanticDistinction::CompiledProductIdentity,
        CompiledProductReuseSemanticDistinction::Equivalence,
        CompiledProductReuseSemanticDistinction::Compatibility,
        CompiledProductReuseSemanticDistinction::AuthorityTruth,
    ] {
        assert!(
            inventory
                .rows()
                .iter()
                .any(|row| row.semantic_distinction() == distinction),
            "missing distinction `{}`",
            distinction.as_str()
        );
    }
}

fn assert_disposition_coverage(inventory: &CompiledProductReuseInventoryReport) {
    for disposition in [
        CompiledProductReuseDisposition::Migrate,
        CompiledProductReuseDisposition::Delete,
        CompiledProductReuseDisposition::Cap,
        CompiledProductReuseDisposition::CertificationOnly,
        CompiledProductReuseDisposition::QueryGap,
    ] {
        assert!(
            inventory
                .rows()
                .iter()
                .any(|row| row.disposition() == disposition),
            "missing disposition `{}`",
            disposition.as_str()
        );
    }
}

fn assert_uncovered_pattern(
    error: CompiledProductReuseInventoryError,
    expected_path: &str,
    expected_pattern: &str,
) {
    match error {
        CompiledProductReuseInventoryError::UncoveredSourcePattern(message) => {
            assert!(message.contains(expected_path), "{message}");
            assert!(message.contains(expected_pattern), "{message}");
        }
        other => panic!("expected uncovered source pattern, got {other:?}"),
    }
}

fn create_fixture_workspace(injected_case: Option<FixtureCase>) -> FixtureWorkspace {
    let root = unique_fixture_root();
    let mut baseline_paths = Vec::new();
    let mut excluded_paths = Vec::new();

    for (relative_path, contents) in SCAN_ROOT_BASELINES {
        write_fixture_file(&root, relative_path, contents);
        baseline_paths.push(relative_path.replace('\\', "/"));
    }
    for (relative_path, contents) in EXCLUDED_SCOPE_FIXTURES {
        write_fixture_file(&root, relative_path, contents);
        excluded_paths.push(relative_path.replace('\\', "/"));
    }
    if let Some(case) = injected_case {
        write_fixture_file(&root, case.relative_path, case.contents);
    }

    FixtureWorkspace {
        root,
        baseline_paths,
        excluded_paths,
    }
}

fn write_fixture_file(workspace_root: &Path, relative_path: &str, contents: &str) {
    let file_path = workspace_root.join(relative_path);
    create_parent_directory(&file_path);
    fs::write(&file_path, contents).expect("fixture file written");
}

fn create_parent_directory(file_path: &Path) {
    let parent = file_path.parent().expect("fixture parent");
    fs::create_dir_all(parent).expect("fixture parent created");
}

fn unique_fixture_root() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("worth_compiled_product_reuse_inventory_{nanos}"));
    fs::create_dir_all(&root).expect("fixture root created");
    root
}
