use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn cleaned_phase_boundaries_stay_under_declared_file_count_caps() {
    let repo_root = repo_root();
    for boundary in [
        DirectoryBoundary::new(
            "workspaces/forge-store/crates/forge-store-blob-chunks/src",
            3,
        ),
        DirectoryBoundary::new(
            "workspaces/forge-store/crates/forge-store-physical-format/src",
            1,
        ),
        DirectoryBoundary::new(
            "workspaces/forge-store/crates/forge-store-recovery-physics/src/integrity_handoff",
            7,
        ),
        DirectoryBoundary::new(
            "workspaces/forge-store/crates/forge-store-test-support/src",
            10,
        ),
    ] {
        boundary.assert_root_rs_file_count(&repo_root);
        boundary.assert_recursive_line_cap(&repo_root, 400);
    }
}

#[test]
fn aggregation_files_remain_aggregation_only() {
    let repo_root = repo_root();
    for relative_path in [
        "workspaces/forge-store/crates/forge-store-blob-chunks/src/lib.rs",
        "workspaces/forge-store/crates/forge-store-blob-chunks/src/exports/mod.rs",
        "workspaces/forge-store/crates/forge-store-physical-format/src/lib.rs",
        "workspaces/forge-store/crates/forge-store-recovery-physics/src/integrity_handoff/mod.rs",
        "workspaces/forge-store/crates/forge-store-test-support/src/lib.rs",
    ] {
        assert_aggregation_only(&repo_root.join(relative_path));
    }
}

#[test]
fn facade_visibility_stays_narrow_at_closeout() {
    let repo_root = repo_root();
    assert_public_mods(
        &repo_root.join("workspaces/forge-store/crates/forge-store-blob-chunks/src/lib.rs"),
        &[
            "blob_capsule_readiness_compile_fail",
            "blob_chunk_integrity_compile_fail",
            "blob_chunk_root_compile_fail",
            "blob_corruption_compile_fail",
            "blob_export_bundle_compile_fail",
            "blob_generation_registry_compile_fail",
            "blob_import_readmission_compile_fail",
            "blob_placement_movement_compile_fail",
            "blob_publication_commit_compile_fail",
            "blob_reachability_compile_fail",
            "blob_recovery_records_compile_fail",
            "blob_retention_reclaim_compile_fail",
            "blob_streaming_read_compile_fail",
            "security_metadata_compile_fail",
        ],
    );
    assert_public_mods(
        &repo_root.join("workspaces/forge-store/crates/forge-store-physical-format/src/lib.rs"),
        &["access", "physical_format_compile_fail"],
    );
    assert_public_mods(
        &repo_root.join("workspaces/forge-store/crates/forge-store-test-support/src/lib.rs"),
        &["harness"],
    );
}

struct DirectoryBoundary {
    relative_path: &'static str,
    max_root_rs_files: usize,
}

impl DirectoryBoundary {
    const fn new(relative_path: &'static str, max_root_rs_files: usize) -> Self {
        Self {
            relative_path,
            max_root_rs_files,
        }
    }

    fn assert_root_rs_file_count(&self, repo_root: &Path) {
        let directory = repo_root.join(self.relative_path);
        let root_rs_files = fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
            .count();
        assert!(
            root_rs_files <= self.max_root_rs_files,
            "{} has {} root .rs files; closeout cap is {}",
            directory.display(),
            root_rs_files,
            self.max_root_rs_files
        );
    }

    fn assert_recursive_line_cap(&self, repo_root: &Path, cap: usize) {
        let directory = repo_root.join(self.relative_path);
        for rust_file in collect_rust_files(&directory) {
            let line_count = fs::read_to_string(&rust_file)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", rust_file.display()))
                .lines()
                .count();
            assert!(
                line_count <= cap,
                "{} is {} lines; closeout cap is {}",
                rust_file.display(),
                line_count,
                cap
            );
        }
    }
}

fn assert_aggregation_only(path: &Path) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let mut inside_pub_use_block = false;
    let mut inside_attribute_block = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if inside_attribute_block {
            if trimmed.ends_with(']') {
                inside_attribute_block = false;
            }
            continue;
        }
        if trimmed.is_empty()
            || trimmed.starts_with("//")
            || trimmed.starts_with("#![")
            || trimmed == "}"
        {
            continue;
        }
        if trimmed.starts_with("#[") {
            if !trimmed.ends_with(']') {
                inside_attribute_block = true;
            }
            continue;
        }
        if inside_pub_use_block {
            if trimmed.ends_with(';') {
                inside_pub_use_block = false;
            }
            continue;
        }
        if (trimmed.starts_with("pub use ") || trimmed.starts_with("pub(crate) use "))
            && !trimmed.ends_with(';')
        {
            inside_pub_use_block = true;
            continue;
        }
        let is_aggregation_line = trimmed.starts_with("mod ")
            || trimmed.starts_with("pub use ")
            || trimmed.starts_with("pub(crate) use ")
            || trimmed.starts_with("pub mod ");
        assert!(
            is_aggregation_line,
            "{} contains non-aggregation line: {}",
            path.display(),
            trimmed
        );
    }
}

fn assert_public_mods(path: &Path, allowed_public_mods: &[&str]) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let mut actual = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(module_name) = trimmed
            .strip_prefix("pub mod ")
            .map(|value| value.trim_end_matches(';'))
        {
            actual.push(module_name.to_string());
        }
    }
    let expected: Vec<String> = allowed_public_mods
        .iter()
        .map(|value| (*value).to_string())
        .collect();
    assert_eq!(
        actual,
        expected,
        "{} exposes unexpected public modules",
        path.display()
    );
}

fn collect_rust_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_rust_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    files.sort();
    files
}

fn repo_root() -> PathBuf {
    let mut current = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if current.join("AGENTS.md").exists() {
            return current;
        }
        assert!(
            current.pop(),
            "failed to locate repository root from {}",
            env!("CARGO_MANIFEST_DIR")
        );
    }
}
