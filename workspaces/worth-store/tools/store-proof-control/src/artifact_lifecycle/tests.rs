use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::*;

#[test]
fn cleanup_executes_exactly_the_dry_run_and_preserves_evidence() {
    let fixture = ArtifactLifecycleFixture::create();
    let inventory = fixture.inventory();
    let plan = BuildArtifactCleanupPlan::lower(
        &inventory,
        BuildArtifactRetentionPolicy::bounded_local().unwrap(),
    )
    .unwrap();
    let planned: BTreeSet<_> = plan
        .targets()
        .iter()
        .map(|target| target.relative_path().to_owned())
        .collect();
    assert!(planned.contains("debug/incremental"));
    assert!(planned.contains("debug/incremental/unit/cache.bin"));
    assert!(planned.contains("debug/deps/stale-deadbeef.rlib"));
    assert!(!planned.contains("debug/deps/current-cafebabe.rlib"));
    assert!(!planned.contains("evidence/ci/bundle.json"));

    let receipt = BuildArtifactCleanupReceipt::execute(&plan).unwrap();
    assert_eq!(receipt.outcome(), &BuildArtifactCleanupOutcome::Completed);
    let deleted: BTreeSet<_> = receipt
        .deleted_targets()
        .iter()
        .map(|target| target.relative_path().to_owned())
        .collect();
    assert_eq!(deleted, planned);
    assert!(receipt.remaining_targets().is_empty());
    assert!(fixture.current_rlib.exists());
    assert!(fixture.evidence_bundle.exists());
    assert!(fixture.ui_expectation.exists());
    fixture.remove();
}

#[test]
fn planning_denies_path_traversal_and_inventory_denies_symlink_escape() {
    let fixture = ArtifactLifecycleFixture::create();
    let outside = fixture.workspace.join("outside");
    mark_disposable_artifact_root(&outside).unwrap();
    let traversal = fixture.target.join("..").join("outside");
    let denial = BuildArtifactInventory::inspect(&fixture.workspace, &traversal, None).unwrap_err();
    assert!(denial.contains("path traversal"));

    let outside_file = outside.join("outside.bin");
    std::fs::write(&outside_file, "outside").unwrap();
    let link = fixture.target.join("escape-link");
    if create_file_symlink(&outside_file, &link).is_ok() {
        let denial =
            BuildArtifactInventory::inspect(&fixture.workspace, &fixture.target, None).unwrap_err();
        assert!(denial.contains("symlink or junction"));
        std::fs::remove_file(link).unwrap();
    }
    fixture.remove();
}

#[test]
fn cleanup_denies_filesystem_drift_before_mutating_any_planned_target() {
    let fixture = ArtifactLifecycleFixture::create();
    let plan = BuildArtifactCleanupPlan::lower(
        &fixture.inventory(),
        BuildArtifactRetentionPolicy::bounded_local().unwrap(),
    )
    .unwrap();
    let planned_file = fixture.target.join("debug/deps/stale-deadbeef.rlib");
    std::fs::write(fixture.target.join("unplanned.bin"), "drift").unwrap();
    let denial = BuildArtifactCleanupReceipt::execute(&plan).unwrap_err();
    assert!(denial.contains("changed after planning"));
    assert!(planned_file.exists());
    fixture.remove();
}

#[test]
fn repeated_warm_edit_residue_stays_bounded_by_the_retention_contract() {
    let fixture = ArtifactLifecycleFixture::create();
    for cycle in 0..3 {
        let cycle_root = fixture
            .target
            .join(format!("debug/incremental/cycle-{cycle}"));
        std::fs::create_dir_all(&cycle_root).unwrap();
        std::fs::write(cycle_root.join("query-cache.bin"), [cycle as u8]).unwrap();
        let plan = BuildArtifactCleanupPlan::lower(
            &fixture.inventory(),
            BuildArtifactRetentionPolicy::bounded_local().unwrap(),
        )
        .unwrap();
        let receipt = BuildArtifactCleanupReceipt::execute(&plan).unwrap();
        assert_eq!(receipt.outcome(), &BuildArtifactCleanupOutcome::Completed);
        assert!(!cycle_root.exists());
    }
    let final_inventory = fixture.inventory();
    assert_eq!(
        final_inventory
            .artifacts()
            .iter()
            .filter(|artifact| artifact.class() == BuildArtifactClass::IncrementalState)
            .count(),
        0
    );
    fixture.remove();
}

struct ArtifactLifecycleFixture {
    workspace: PathBuf,
    target: PathBuf,
    current_rlib: PathBuf,
    evidence_bundle: PathBuf,
    ui_expectation: PathBuf,
}

impl ArtifactLifecycleFixture {
    fn create() -> Self {
        let workspace = std::env::temp_dir().join(format!(
            "store-artifact-lifecycle-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&workspace).unwrap();
        let target = workspace.join("artifact-fixture-target");
        mark_disposable_artifact_root(&target).unwrap();
        let deps = target.join("debug/deps");
        let incremental = target.join("debug/incremental/unit");
        let evidence = target.join("evidence/ci");
        let ui = target.join("evidence/ui");
        for directory in [&deps, &incremental, &evidence, &ui] {
            std::fs::create_dir_all(directory).unwrap();
        }
        let current_rlib = deps.join("current-cafebabe.rlib");
        std::fs::write(&current_rlib, "current").unwrap();
        std::fs::write(deps.join("stale-deadbeef.rlib"), "stale").unwrap();
        std::fs::write(incremental.join("cache.bin"), "incremental").unwrap();
        let evidence_bundle = evidence.join("bundle.json");
        std::fs::write(&evidence_bundle, "{}").unwrap();
        let ui_expectation = ui.join("expected.stderr");
        std::fs::write(&ui_expectation, "denied").unwrap();
        Self {
            workspace,
            target,
            current_rlib,
            evidence_bundle,
            ui_expectation,
        }
    }

    fn inventory(&self) -> BuildArtifactInventory {
        BuildArtifactInventory::inspect_with_test_reuse_paths(
            &self.workspace,
            &self.target,
            std::slice::from_ref(&self.current_rlib),
        )
        .unwrap()
    }

    fn remove(self) {
        std::fs::remove_dir_all(self.workspace).unwrap();
    }
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
