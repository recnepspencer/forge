use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;

use syn::{Item, UseTree, Visibility};

pub fn lifecycle_propagation_fixture_paths() -> &'static [&'static str] {
    &[
        "tests/ui/lifecycle/external_runtime_bootstrap_fields_are_private.rs",
        "tests/ui/lifecycle/external_runtime_bootstrap_constructor_is_private.rs",
        "tests/ui/lifecycle/external_runtime_facade_inspection_inventory_freeze_is_not_public.rs",
        "tests/ui/lifecycle/external_runtime_root_lifecycle_factories_are_not_public.rs",
        "tests/ui/lifecycle/external_runtime_support_inventory_construction_is_private.rs",
        "tests/ui/lifecycle/external_inspection_scope_inventory_construction_is_private.rs",
        "tests/ui/lifecycle/external_inspection_subsystem_bootstrap_is_private.rs",
    ]
}

pub fn expected_phase3_lifecycle_subsystems() -> &'static [&'static str] {
    &[
        "dsl_package",
        "inspection",
        "query_binding",
        "host_contract",
    ]
}

pub fn audit_phase3_lifecycle_public_surface(workspace_root: &Path) -> Vec<String> {
    let file_expectations = HashMap::from([
        (
            "crates/worth-ui-runtime/src/facade/mod.rs",
            BTreeSet::from([
                "RUNTIME_SUPPORT_INVENTORY",
                "WorthUiRuntimeSupportInventory",
            ]),
        ),
        (
            "crates/worth-ui-inspection/src/lib.rs",
            BTreeSet::from([
                "RUNTIME_INSPECTION_SCOPE_INVENTORY",
                "UiInspectionScopeInventory",
            ]),
        ),
        (
            "crates/worth-ui-inspection/src/facade/mod.rs",
            BTreeSet::from([
                "RUNTIME_INSPECTION_SCOPE_INVENTORY",
                "UiInspectionScopeInventory",
            ]),
        ),
    ]);
    let mut violations = Vec::new();

    for (relative_path, expected_names) in file_expectations {
        let path = workspace_root.join(relative_path);
        let actual_names = collect_phase3_public_surface_names(&path);

        if actual_names != expected_names {
            violations.push(format!(
                "{} exposes lifecycle public names {:?}; expected {:?}",
                path.display(),
                actual_names,
                expected_names
            ));
        }
    }

    violations.sort();
    violations
}

fn collect_phase3_public_surface_names(path: &Path) -> BTreeSet<&'static str> {
    let text = fs::read_to_string(path).unwrap_or_else(|error| {
        panic!(
            "{} should decode as UTF-8 Rust source: {error}",
            path.display()
        );
    });
    let parsed = syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    });
    let mut names = BTreeSet::new();

    for item in parsed.items {
        match item {
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                collect_public_use_names(&item_use.tree, &mut names);
            }
            Item::Const(item_const)
                if matches!(item_const.vis, Visibility::Public(_))
                    && is_phase3_lifecycle_surface_name(&item_const.ident.to_string()) =>
            {
                names.insert(leak_name(item_const.ident.to_string()));
            }
            Item::Fn(item_fn)
                if matches!(item_fn.vis, Visibility::Public(_))
                    && is_phase3_lifecycle_surface_name(&item_fn.sig.ident.to_string()) =>
            {
                names.insert(leak_name(item_fn.sig.ident.to_string()));
            }
            Item::Struct(item_struct)
                if matches!(item_struct.vis, Visibility::Public(_))
                    && is_phase3_lifecycle_surface_name(&item_struct.ident.to_string()) =>
            {
                names.insert(leak_name(item_struct.ident.to_string()));
            }
            _ => {}
        }
    }

    names
}

fn collect_public_use_names(tree: &UseTree, output: &mut BTreeSet<&'static str>) {
    match tree {
        UseTree::Name(name) => {
            let name = name.ident.to_string();
            if is_phase3_lifecycle_surface_name(&name) {
                output.insert(leak_name(name));
            }
        }
        UseTree::Rename(rename) => {
            let name = rename.rename.to_string();
            if is_phase3_lifecycle_surface_name(&name) {
                output.insert(leak_name(name));
            }
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_public_use_names(item, output);
            }
        }
        UseTree::Path(path) => collect_public_use_names(&path.tree, output),
        UseTree::Glob(_) => {}
    }
}

fn is_phase3_lifecycle_surface_name(name: &str) -> bool {
    name.starts_with("phase3_")
        || name.starts_with("PHASE3_")
        || name.contains("SCOPE_INVENTORY")
        || name.contains("SUPPORT_INVENTORY")
        || name.contains("Bootstrap")
        || name.contains("SupportInventory")
        || name.contains("ScopeInventory")
}

fn leak_name(name: String) -> &'static str {
    Box::leak(name.into_boxed_str())
}
