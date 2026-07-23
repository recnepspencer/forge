use std::fs;
use std::path::Path;

use syn::{File, Item, ItemUse, UseTree, Visibility};

use super::public_surface_audit::collect_public_names;

const RUNTIME_FACADE_ROOT: &str = "crates/worth-ui-runtime/src/facade/mod.rs";
const RUNTIME_ADMISSION_FACADE: &str = "crates/worth-ui-runtime/src/facade/admission.rs";
const PRODUCT_FACADE_ROOT: &str = "crates/worth-ui/src/facade/mod.rs";
const PRODUCT_ADMISSION_FACADE: &str = "crates/worth-ui/src/facade/admission.rs";
const CURATED_ADMISSION_PUBLIC_NAMES: &[&str] = &[
    "UiAdmissionAggregation",
    "UiAdmissionBoundary",
    "UiAdmissionDecision",
    "UiAdmissionFamily",
    "UiAdmissionHostCapability",
    "UiAdmissionOutcome",
    "UiAdmissionQueryBasis",
    "UiAdmissionReport",
    "UiAdmissionSelectionBudget",
    "UiAdmissionStaleEvidence",
    "UiAdmissionTarget",
    "UiAdmissionWorld",
    "UiLegalityDecision",
    "UiLegalityPosture",
    "UiLegalityReason",
    "UiSupportPosture",
    "UiSupportReason",
    "UiSupportSnapshot",
];

pub fn audit_runtime_admission_surface_routes_through_curated_submodule(
    workspace_root: &Path,
) -> Vec<String> {
    let root_path = workspace_root.join(RUNTIME_FACADE_ROOT);
    let root_public_names = collect_public_names(&root_path);
    let mut violations = Vec::new();

    for name in root_public_names {
        if looks_like_admission_surface(&name) {
            violations.push(format!(
                "{} publicly exposes `{name}` from the runtime facade root instead of routing admission authority through `facade::admission`",
                root_path.display()
            ));
        }
    }

    if !parse_rust_file(&root_path).items.iter().any(|item| {
        matches!(
            item,
            Item::Mod(item_mod)
                if matches!(item_mod.vis, Visibility::Public(_)) && item_mod.ident == "admission"
        )
    }) {
        violations.push(format!(
            "{} must publish one dedicated `admission` facade submodule",
            root_path.display()
        ));
    }

    violations.sort();
    violations.dedup();
    violations
}

pub fn audit_admission_facades_are_curated_and_glob_free(workspace_root: &Path) -> Vec<String> {
    let product_root_path = workspace_root.join(PRODUCT_FACADE_ROOT);
    let runtime_path = workspace_root.join(RUNTIME_ADMISSION_FACADE);
    let product_path = workspace_root.join(PRODUCT_ADMISSION_FACADE);
    let runtime_names = collect_public_names(&runtime_path);
    let product_names = collect_public_names(&product_path);
    let mut violations = Vec::new();

    if product_root_contains_flat_surface(&parse_rust_file(&product_root_path)) {
        violations.push(format!(
            "{} must not declare a compatibility facade or flat root re-exports; product callers must enter through named facade modules",
            product_root_path.display()
        ));
    }

    if runtime_names != product_names {
        violations.push(format!(
            "{} must mirror the curated runtime admission facade exactly; product names: {:?}, runtime names: {:?}",
            product_path.display(),
            product_names,
            runtime_names
        ));
    }

    let curated_names = curated_name_set();
    if runtime_names != curated_names {
        violations.push(format!(
            "{} must expose exactly the curated admission capability set; observed: {:?}, expected: {:?}",
            runtime_path.display(),
            runtime_names,
            curated_names
        ));
    }

    if let Some(reason) = first_invalid_public_use(&runtime_path, &[&["crate", "admission"]]) {
        violations.push(format!("{} {reason}", runtime_path.display()));
    }
    if let Some(reason) = first_invalid_public_use(
        &product_path,
        &[&["worth_ui_runtime", "facade", "admission"]],
    ) {
        violations.push(format!("{} {reason}", product_path.display()));
    }

    violations.sort();
    violations.dedup();
    violations
}

fn product_root_contains_flat_surface(parsed: &File) -> bool {
    parsed.items.iter().any(|item| match item {
        Item::Mod(item_mod) => item_mod.ident == "compat",
        Item::Use(item_use) => matches!(item_use.vis, Visibility::Public(_)),
        _ => false,
    })
}

fn first_invalid_public_use(path: &Path, allowed_prefixes: &[&[&str]]) -> Option<String> {
    let parsed = parse_rust_file(path);

    for item in parsed.items {
        match item {
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                if contains_glob_use(&item_use.tree) {
                    return Some(
                        "must enumerate curated admission exports explicitly instead of using glob re-exports"
                            .to_string(),
                    );
                }

                let prefixes = public_use_prefixes(&item_use);
                if prefixes.iter().any(|prefix| {
                    !allowed_prefixes.iter().any(|expected_prefix| {
                        prefix.len() >= expected_prefix.len()
                            && expected_prefix
                                .iter()
                                .zip(prefix.iter())
                                .all(|(expected, actual)| expected == actual)
                    })
                }) {
                    return Some(format!(
                        "must route public admission exports only through one of: {}",
                        allowed_prefixes
                            .iter()
                            .map(|prefix| prefix.join("::"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
            Item::Mod(item_mod) if matches!(item_mod.vis, Visibility::Public(_)) => {
                return Some(format!(
                    "must not publish nested public modules such as `{}` from the admission facade",
                    item_mod.ident
                ));
            }
            _ => {}
        }
    }

    None
}

fn public_use_prefixes(item_use: &ItemUse) -> Vec<Vec<String>> {
    let mut prefixes = Vec::new();
    collect_use_prefixes(&item_use.tree, Vec::new(), &mut prefixes);
    prefixes
}

fn collect_use_prefixes(tree: &UseTree, prefix: Vec<String>, output: &mut Vec<Vec<String>>) {
    match tree {
        UseTree::Path(path) => {
            let mut next = prefix;
            next.push(path.ident.to_string());
            collect_use_prefixes(&path.tree, next, output);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                collect_use_prefixes(item, prefix.clone(), output);
            }
        }
        UseTree::Name(_) | UseTree::Rename(_) => output.push(prefix),
        UseTree::Glob(_) => output.push(prefix),
    }
}

fn contains_glob_use(tree: &UseTree) -> bool {
    match tree {
        UseTree::Glob(_) => true,
        UseTree::Path(path) => contains_glob_use(&path.tree),
        UseTree::Group(group) => group.items.iter().any(contains_glob_use),
        UseTree::Name(_) | UseTree::Rename(_) => false,
    }
}

fn looks_like_admission_surface(name: &str) -> bool {
    name.starts_with("UiAdmission")
        || name.starts_with("UiLegality")
        || name.starts_with("UiSupport")
}

fn parse_rust_file(path: &Path) -> File {
    let text = fs::read_to_string(path).expect("source file should decode");
    syn::parse_file(&text).unwrap_or_else(|error| {
        panic!("{} should parse as Rust source: {error}", path.display());
    })
}

fn curated_name_set() -> std::collections::BTreeSet<String> {
    CURATED_ADMISSION_PUBLIC_NAMES
        .iter()
        .map(|name| (*name).to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::product_root_contains_flat_surface;

    #[test]
    fn compatibility_surface_audit_rejects_private_module_glob_reexport_loophole() {
        let parsed = syn::parse_file("mod compat; pub use compat::*;")
            .expect("hostile compatibility facade fixture parses");
        assert!(product_root_contains_flat_surface(&parsed));
    }

    #[test]
    fn compatibility_surface_audit_rejects_non_compat_flat_reexports() {
        let parsed = syn::parse_file("pub use crate::support::SupportSnapshot;")
            .expect("flat facade export fixture parses");
        assert!(product_root_contains_flat_surface(&parsed));
    }

    #[test]
    fn named_facade_modules_do_not_trigger_compatibility_surface_audit() {
        let parsed =
            syn::parse_file("pub mod app; pub mod runtime;").expect("named facade fixture parses");
        assert!(!product_root_contains_flat_surface(&parsed));
    }
}
