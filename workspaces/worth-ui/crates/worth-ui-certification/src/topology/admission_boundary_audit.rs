use super::dependency_audit::{
    collect_file_paths, manifest_dependency_crate_aliases, manifests_dependencies,
    normalize_manifest_alias_path,
};
use super::workspace_source_inventory::WorkspaceSourceInventory;

const ADMISSION_OWNER_CRATES: [&str; 3] =
    ["worth-ui", "worth-ui-runtime", "worth-ui-certification"];
const FORBIDDEN_ADMISSION_BYPASS_DEP: &str = "worth-ui-runtime";

pub fn audit_consumers_route_admission_through_worth_ui_facade(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let mut violations = Vec::new();

    for crate_root in workspace_crate_roots(inventory) {
        let crate_name = crate_root
            .file_name()
            .expect("crate roots should have final path component")
            .to_string_lossy()
            .into_owned();
        if ADMISSION_OWNER_CRATES.contains(&crate_name.as_str()) {
            continue;
        }

        let manifest = crate_root.join("Cargo.toml");
        let manifest_aliases = if inventory.source(&manifest).is_some() {
            manifest_dependency_crate_aliases(inventory, &manifest)
        } else {
            Default::default()
        };
        let src_root = crate_root.join("src");
        let mut crate_uses_runtime_admission = false;

        if inventory.contains(
            src_root
                .strip_prefix(inventory.root())
                .expect("source is in inventory"),
        ) {
            let rust_files = inventory.rust_files_under(
                src_root
                    .strip_prefix(inventory.root())
                    .expect("source is in inventory"),
            );
            for file in rust_files {
                let file_text = file.text();
                let reaches_runtime_admission = collect_file_paths(inventory, file.absolute_path())
                    .into_iter()
                    .any(|segments| {
                        let normalized =
                            normalize_manifest_alias_path(&segments, &manifest_aliases);
                        normalized
                            .first()
                            .is_some_and(|segment| segment == "worth_ui_runtime")
                            && normalized.get(1).is_some_and(|segment| segment == "facade")
                            && normalized
                                .get(2)
                                .is_some_and(|segment| segment == "admission")
                    })
                    || manifest_aliases.iter().any(|(crate_alias, package_name)| {
                        package_name == "worth_ui_runtime"
                            && file_text.contains(&format!("{crate_alias}::facade::admission"))
                    });

                if reaches_runtime_admission {
                    crate_uses_runtime_admission = true;
                    violations.push(format!(
                        "{} bypasses the product admission facade and reaches `worth_ui_runtime::facade::admission`; external consumers must enter through `worth_ui::facade::admission`",
                        file.absolute_path().display()
                    ));
                }
            }
        }

        if crate_uses_runtime_admission && inventory.source(&manifest).is_some() {
            let dependencies = manifests_dependencies(inventory, &manifest);
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

fn workspace_crate_roots(inventory: &WorkspaceSourceInventory) -> Vec<std::path::PathBuf> {
    let mut roots = inventory
        .direct_entries_under("crates")
        .filter(|path| inventory.contains(path.join("Cargo.toml")))
        .map(|path| inventory.absolute_path(path))
        .collect::<Vec<_>>();
    roots.sort();
    roots
}
