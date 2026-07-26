use std::fs;
use std::path::{Path, PathBuf};

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
const CURATED_ADMISSION_AUDIENCE_NAMES: &[&str] = &["WorthUiAdmissionExt"];

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
    let paths = AdmissionFacadePaths::new(workspace_root);
    let names = AdmissionSurfaceNames::capture(&paths);
    let mut violations = Vec::new();

    audit_product_root(&paths, &mut violations);
    audit_product_mirror(&paths, &names, &mut violations);
    audit_runtime_capabilities(&paths, &names, &mut violations);
    audit_runtime_audiences(&paths, &names, &mut violations);
    audit_public_routes(&paths, &mut violations);

    violations.sort();
    violations.dedup();
    violations
}

struct AdmissionFacadePaths {
    product_root: PathBuf,
    runtime: PathBuf,
    product: PathBuf,
}

impl AdmissionFacadePaths {
    fn new(workspace_root: &Path) -> Self {
        Self {
            product_root: workspace_root.join(PRODUCT_FACADE_ROOT),
            runtime: workspace_root.join(RUNTIME_ADMISSION_FACADE),
            product: workspace_root.join(PRODUCT_ADMISSION_FACADE),
        }
    }
}

struct AdmissionSurfaceNames {
    runtime: std::collections::BTreeSet<String>,
    product: std::collections::BTreeSet<String>,
    runtime_audiences: std::collections::BTreeSet<String>,
}

impl AdmissionSurfaceNames {
    fn capture(paths: &AdmissionFacadePaths) -> Self {
        Self {
            runtime: collect_public_names(&paths.runtime),
            product: collect_public_names(&paths.product),
            runtime_audiences: collect_public_trait_names(&parse_rust_file(&paths.runtime)),
        }
    }
}

fn audit_product_root(paths: &AdmissionFacadePaths, violations: &mut Vec<String>) {
    if product_root_contains_flat_surface(&parse_rust_file(&paths.product_root)) {
        violations.push(format!(
            "{} must not declare a compatibility facade or flat root re-exports; product callers must enter through named facade modules",
            paths.product_root.display()
        ));
    }
}

fn audit_product_mirror(
    paths: &AdmissionFacadePaths,
    names: &AdmissionSurfaceNames,
    violations: &mut Vec<String>,
) {
    let mut expected = names.runtime.clone();
    expected.extend(names.runtime_audiences.iter().cloned());
    if names.product != expected {
        violations.push(format!(
            "{} must mirror the curated runtime admission capabilities and named audiences exactly; product names: {:?}, expected names: {:?}",
            paths.product.display(),
            names.product,
            expected
        ));
    }
}

fn audit_runtime_capabilities(
    paths: &AdmissionFacadePaths,
    names: &AdmissionSurfaceNames,
    violations: &mut Vec<String>,
) {
    let expected = curated_name_set();
    if names.runtime != expected {
        violations.push(format!(
            "{} must expose exactly the curated admission capability set; observed: {:?}, expected: {:?}",
            paths.runtime.display(),
            names.runtime,
            expected
        ));
    }
}

fn audit_runtime_audiences(
    paths: &AdmissionFacadePaths,
    names: &AdmissionSurfaceNames,
    violations: &mut Vec<String>,
) {
    let expected = curated_audience_name_set();
    if names.runtime_audiences != expected {
        violations.push(format!(
            "{} must declare exactly the curated admission audience traits; observed: {:?}, expected: {:?}",
            paths.runtime.display(),
            names.runtime_audiences,
            expected
        ));
    }
}

fn audit_public_routes(paths: &AdmissionFacadePaths, violations: &mut Vec<String>) {
    if let Some(reason) = first_invalid_public_use(&paths.runtime, &[&["crate", "admission"]]) {
        violations.push(format!("{} {reason}", paths.runtime.display()));
    }
    if let Some(reason) = first_invalid_public_use(
        &paths.product,
        &[&["worth_ui_runtime", "facade", "admission"]],
    ) {
        violations.push(format!("{} {reason}", paths.product.display()));
    }
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

fn collect_public_trait_names(parsed: &File) -> std::collections::BTreeSet<String> {
    parsed
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Trait(item_trait) if matches!(item_trait.vis, Visibility::Public(_)) => {
                Some(item_trait.ident.to_string())
            }
            _ => None,
        })
        .collect()
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

fn curated_audience_name_set() -> std::collections::BTreeSet<String> {
    CURATED_ADMISSION_AUDIENCE_NAMES
        .iter()
        .map(|name| (*name).to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{collect_public_trait_names, product_root_contains_flat_surface};

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

    #[test]
    fn admission_audience_inventory_rejects_unlisted_public_traits() {
        let parsed = syn::parse_file(
            "pub trait WorthUiAdmissionExt {} pub trait WorthUiAdmissionBypassExt {}",
        )
        .expect("hostile audience fixture parses");
        let names = collect_public_trait_names(&parsed);
        assert_eq!(
            names,
            ["WorthUiAdmissionBypassExt", "WorthUiAdmissionExt"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        );
    }
}
