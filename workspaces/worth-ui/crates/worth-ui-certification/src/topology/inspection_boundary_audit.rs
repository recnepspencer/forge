use super::dependency_audit::{
    collect_file_paths, manifest_dependency_crate_aliases, manifests_dependencies,
    normalize_manifest_alias_path, path_starts_with,
};
use super::workspace_source_inventory::WorkspaceSourceInventory;

const INSPECTION_OWNER_CRATES: [&str; 5] = [
    "worth-ui",
    "worth-ui-runtime",
    "worth-ui-inspection",
    "worth-ui-certification",
    "worth-ui-test-support",
];

const FORBIDDEN_INSPECTION_BYPASS_DEPS: [&str; 2] = ["worth-ui-runtime", "worth-ui-inspection"];

pub fn audit_consumers_route_inspection_through_worth_ui_facade(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let mut violations = Vec::new();

    for crate_root in workspace_crate_roots(inventory) {
        let crate_name = crate_root
            .file_name()
            .expect("crate roots should have final path component")
            .to_string_lossy()
            .into_owned();
        if INSPECTION_OWNER_CRATES.contains(&crate_name.as_str()) {
            continue;
        }

        let manifest = crate_root.join("Cargo.toml");
        if inventory.source(&manifest).is_some() {
            let dependencies = manifests_dependencies(inventory, &manifest);
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
        let source_relative = src_root
            .strip_prefix(inventory.root())
            .expect("source is in inventory");
        if !inventory.contains(source_relative) {
            continue;
        }

        let rust_files = inventory.rust_files_under(source_relative);
        let manifest_aliases = if inventory.source(&manifest).is_some() {
            manifest_dependency_crate_aliases(inventory, &manifest)
        } else {
            Default::default()
        };
        for file in rust_files {
            let file_text = file.text();
            for segments in collect_file_paths(inventory, file.absolute_path()) {
                let normalized_segments =
                    normalize_manifest_alias_path(&segments, &manifest_aliases);
                if path_starts_with(&normalized_segments, "worth_ui_runtime")
                    || path_starts_with(&normalized_segments, "worth_ui_inspection")
                {
                    violations.push(format!(
                        "{} reaches runtime-owned inspection surfaces directly; external consumers must enter through worth_ui::facade",
                        file.absolute_path().display()
                    ));
                }
            }
            for (crate_alias, package_name) in &manifest_aliases {
                if (package_name == "worth_ui_runtime" || package_name == "worth_ui_inspection")
                    && file_text.contains(&format!("{crate_alias}::"))
                {
                    violations.push(format!(
                        "{} reaches runtime-owned inspection surfaces directly; external consumers must enter through worth_ui::facade",
                        file.absolute_path().display()
                    ));
                }
            }
        }
    }

    violations.sort();
    violations.dedup();
    violations
}

fn workspace_crate_roots(inventory: &WorkspaceSourceInventory) -> Vec<std::path::PathBuf> {
    let mut roots = inventory
        .direct_entries_under("crates")
        .filter(|path| inventory.contains(path.join("Cargo.toml")))
        .map(|path| inventory.absolute_path(path))
        .collect::<Vec<_>>();
    roots.sort();
    roots
}
