use std::fs;
use std::path::{Path, PathBuf};

use super::dependency_audit::{
    collect_file_paths, collect_rust_files, manifest_dependency_crate_aliases,
    manifests_dependencies, normalize_manifest_alias_path, path_starts_with,
};

const INSPECTION_OWNER_CRATES: [&str; 5] = [
    "worth-ui",
    "worth-ui-runtime",
    "worth-ui-inspection",
    "worth-ui-certification",
    "worth-ui-test-support",
];

const FORBIDDEN_INSPECTION_BYPASS_DEPS: [&str; 2] = ["worth-ui-runtime", "worth-ui-inspection"];

pub fn audit_consumers_route_inspection_through_worth_ui_facade(
    workspace_root: &Path,
) -> Vec<String> {
    let crates_root = workspace_root.join("crates");
    let mut violations = Vec::new();

    for crate_root in workspace_crate_roots(&crates_root) {
        let crate_name = crate_root
            .file_name()
            .expect("crate roots should have final path component")
            .to_string_lossy()
            .into_owned();
        if INSPECTION_OWNER_CRATES.contains(&crate_name.as_str()) {
            continue;
        }

        let manifest = crate_root.join("Cargo.toml");
        if manifest.exists() {
            let dependencies = manifests_dependencies(&manifest);
            for forbidden_dep in FORBIDDEN_INSPECTION_BYPASS_DEPS {
                if dependencies
                    .iter()
                    .any(|dependency| dependency.package == forbidden_dep)
                {
                    violations.push(format!(
                        "{} depends on `{forbidden_dep}` directly; external consumers must route inspection through the worth-ui facade",
                        manifest.display()
                    ));
                }
            }
        }

        let src_root = crate_root.join("src");
        if !src_root.exists() {
            continue;
        }

        let mut rust_files = Vec::new();
        collect_rust_files(&src_root, &mut rust_files);
        let manifest_aliases = if manifest.exists() {
            manifest_dependency_crate_aliases(&manifest)
        } else {
            Default::default()
        };
        for file in rust_files {
            let file_text = fs::read_to_string(&file).expect("consumer source file should decode");
            for segments in collect_file_paths(&file) {
                let normalized_segments =
                    normalize_manifest_alias_path(&segments, &manifest_aliases);
                if path_starts_with(&normalized_segments, "worth_ui_runtime")
                    || path_starts_with(&normalized_segments, "worth_ui_inspection")
                {
                    violations.push(format!(
                        "{} reaches runtime-owned inspection surfaces directly; external consumers must enter through worth_ui::facade",
                        file.display()
                    ));
                }
            }
            for (crate_alias, package_name) in &manifest_aliases {
                if (package_name == "worth_ui_runtime" || package_name == "worth_ui_inspection")
                    && file_text.contains(&format!("{crate_alias}::"))
                {
                    violations.push(format!(
                        "{} reaches runtime-owned inspection surfaces directly; external consumers must enter through worth_ui::facade",
                        file.display()
                    ));
                }
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn workspace_crate_roots(crates_root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();

    for entry in fs::read_dir(crates_root).expect("workspace crates directory should read") {
        let entry = entry.expect("workspace crate directory entry should read");
        let path = entry.path();
        if path.is_dir() {
            roots.push(path);
        }
    }

    roots.sort();
    roots
}
