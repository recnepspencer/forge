use std::fs;
use std::path::{Path, PathBuf};

use super::dependency_audit::{
    collect_file_paths, collect_rust_files, manifest_dependency_crate_aliases,
    manifests_dependencies, normalize_manifest_alias_path,
};

const ADMISSION_OWNER_CRATES: [&str; 3] =
    ["worth-ui", "worth-ui-runtime", "worth-ui-certification"];
const FORBIDDEN_ADMISSION_BYPASS_DEP: &str = "worth-ui-runtime";

pub fn audit_consumers_route_admission_through_worth_ui_facade(
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
        if ADMISSION_OWNER_CRATES.contains(&crate_name.as_str()) {
            continue;
        }

        let manifest = crate_root.join("Cargo.toml");
        let manifest_aliases = if manifest.exists() {
            manifest_dependency_crate_aliases(&manifest)
        } else {
            Default::default()
        };
        let src_root = crate_root.join("src");
        let mut crate_uses_runtime_admission = false;

        if src_root.exists() {
            let mut rust_files = Vec::new();
            collect_rust_files(&src_root, &mut rust_files);
            for file in rust_files {
                let file_text =
                    fs::read_to_string(&file).expect("consumer source file should decode");
                let reaches_runtime_admission =
                    collect_file_paths(&file).into_iter().any(|segments| {
                        let normalized =
                            normalize_manifest_alias_path(&segments, &manifest_aliases);
                        normalized
                            .first()
                            .is_some_and(|segment| segment == "worth_ui_runtime")
                            && normalized.get(1).is_some_and(|segment| segment == "facade")
                            && normalized
                                .get(2)
                                .is_some_and(|segment| segment == "admission")
                    }) || manifest_aliases.iter().any(|(crate_alias, package_name)| {
                        package_name == "worth_ui_runtime"
                            && file_text.contains(&format!("{crate_alias}::facade::admission"))
                    });

                if reaches_runtime_admission {
                    crate_uses_runtime_admission = true;
                    violations.push(format!(
                        "{} bypasses the product admission facade and reaches `worth_ui_runtime::facade::admission`; external consumers must enter through `worth_ui::facade::admission`",
                        file.display()
                    ));
                }
            }
        }

        if crate_uses_runtime_admission && manifest.exists() {
            let dependencies = manifests_dependencies(&manifest);
            if dependencies
                .iter()
                .any(|dependency| dependency.package == FORBIDDEN_ADMISSION_BYPASS_DEP)
            {
                violations.push(format!(
                    "{} depends on `{FORBIDDEN_ADMISSION_BYPASS_DEP}` directly; external admission consumers must route through the worth-ui facade",
                    manifest.display()
                ));
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
