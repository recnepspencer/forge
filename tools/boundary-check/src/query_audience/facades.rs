//! Leaf-facade contract for configured Query audience crates.
//!
//! Validates that each matrix audience package under `crates/<package>` is a
//! zero-behavior re-export leaf over the configured engine: one direct engine
//! dependency, facade-only lib surface, and re-export-only facade module.

use crate::config::QueryAudienceContract;
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use std::fs;
use std::path::Path;
use syn::{Item, ItemMod, UseTree, Visibility};

pub(crate) fn validate_query_audience_facades(
    root: &Path,
    contract: &QueryAudienceContract,
) -> Result<Vec<Diagnostic>, String> {
    let mut diagnostics = Vec::new();
    let audience_packages: Vec<&str> = contract
        .audiences
        .iter()
        .map(|audience| audience.package.as_str())
        .collect();

    for audience in &contract.audiences {
        let relative = format!("crates/{}", audience.package);
        let crate_root = root.join(&relative);
        if !crate_root.is_dir() {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                relative,
                format!(
                    "configured Query audience facade `{}` is missing under crates/",
                    audience.package
                ),
            ));
            continue;
        }

        diagnostics.extend(validate_engine_only_dependency(
            &crate_root,
            &relative,
            &contract.engine_package,
            &audience_packages,
        )?);
        diagnostics.extend(validate_facade_only_lib(
            &crate_root.join("src/lib.rs"),
            &relative,
        )?);
        diagnostics.extend(validate_reexport_only_facade(
            &crate_root.join("src/facade.rs"),
            &relative,
            &contract.engine_package,
            &audience_packages,
        )?);
    }

    Ok(diagnostics)
}

fn validate_engine_only_dependency(
    crate_root: &Path,
    relative: &str,
    engine_package: &str,
    audience_packages: &[&str],
) -> Result<Vec<Diagnostic>, String> {
    let manifest_path = crate_root.join("Cargo.toml");
    let text = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("read {}: {e}", manifest_path.display()))?;
    let value: toml::Value =
        toml::from_str(&text).map_err(|e| format!("parse {}: {e}", manifest_path.display()))?;
    let deps = direct_dependency_package_names(&value);
    let mut diagnostics = Vec::new();

    if deps.len() != 1 || deps[0] != engine_package {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
            format!("{relative}/Cargo.toml"),
            format!(
                "Query audience facade must depend only on engine package `{engine_package}`; found dependencies: {}",
                if deps.is_empty() {
                    "none".to_owned()
                } else {
                    deps.join(", ")
                }
            ),
        ));
    }

    for dep in &deps {
        if audience_packages.iter().any(|package| package == dep) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                format!("{relative}/Cargo.toml"),
                format!("Query audience facade must not depend on another audience facade `{dep}`"),
            ));
        }
    }

    Ok(diagnostics)
}

fn direct_dependency_package_names(manifest: &toml::Value) -> Vec<String> {
    let Some(deps) = manifest
        .get("dependencies")
        .and_then(|value| value.as_table())
    else {
        return Vec::new();
    };
    let mut names = deps
        .iter()
        .map(|(key, spec)| {
            if let Some(table) = spec.as_table() {
                table
                    .get("package")
                    .and_then(|package| package.as_str())
                    .unwrap_or(key)
                    .to_owned()
            } else {
                key.clone()
            }
        })
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();
    names
}

fn validate_facade_only_lib(path: &Path, relative: &str) -> Result<Vec<Diagnostic>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let syntax = syn::parse_file(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let mut diagnostics = Vec::new();
    let mut facade_exports = 0usize;

    for item in syntax.items {
        match item {
            Item::Mod(ItemMod {
                vis: Visibility::Public(_),
                ident,
                content: None,
                ..
            }) if ident == "facade" => {
                facade_exports += 1;
            }
            Item::Mod(ItemMod {
                vis: Visibility::Inherited,
                content: None,
                ..
            }) => {}
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                    format!("{relative}/src/lib.rs"),
                    "Query audience lib.rs must export only the facade module".to_owned(),
                ));
            }
            _ => {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                    format!("{relative}/src/lib.rs"),
                    "Query audience lib.rs must remain facade-only wiring (no behavior items)"
                        .to_owned(),
                ));
            }
        }
    }

    if facade_exports != 1 {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
            format!("{relative}/src/lib.rs"),
            "Query audience lib.rs must declare exactly one public facade module".to_owned(),
        ));
    }

    Ok(diagnostics)
}

fn validate_reexport_only_facade(
    path: &Path,
    relative: &str,
    engine_package: &str,
    audience_packages: &[&str],
) -> Result<Vec<Diagnostic>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let syntax = syn::parse_file(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let mut diagnostics = Vec::new();
    let engine_crate = engine_package.replace('-', "_");

    if syntax.items.is_empty() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
            format!("{relative}/src/facade.rs"),
            "Query audience facade.rs must not be empty".to_owned(),
        ));
        return Ok(diagnostics);
    }

    for item in syntax.items {
        match item {
            Item::Use(item_use) if matches!(item_use.vis, Visibility::Public(_)) => {
                if use_tree_is_glob(&item_use.tree) {
                    diagnostics.push(Diagnostic::new(
                        DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                        format!("{relative}/src/facade.rs"),
                        "Query audience facade.rs must not use glob re-exports".to_owned(),
                    ));
                }
                if let Some(root) = use_tree_root_ident(&item_use.tree) {
                    if root != engine_crate {
                        diagnostics.push(Diagnostic::new(
                            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                            format!("{relative}/src/facade.rs"),
                            format!(
                                "Query audience facade.rs may re-export only from engine crate `{engine_crate}`; found `{root}`"
                            ),
                        ));
                    }
                    let root_package = root.replace('_', "-");
                    if audience_packages.contains(&root_package.as_str()) {
                        diagnostics.push(Diagnostic::new(
                            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                            format!("{relative}/src/facade.rs"),
                            format!(
                                "Query audience facade must not re-export another audience facade `{root_package}`"
                            ),
                        ));
                    }
                }
            }
            _ => {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                    format!("{relative}/src/facade.rs"),
                    "Query audience facade.rs must contain only public re-exports (no wrappers, functions, or types)"
                        .to_owned(),
                ));
            }
        }
    }

    Ok(diagnostics)
}

fn use_tree_is_glob(tree: &UseTree) -> bool {
    match tree {
        UseTree::Glob(_) => true,
        UseTree::Path(path) => use_tree_is_glob(&path.tree),
        UseTree::Group(group) => group.items.iter().any(use_tree_is_glob),
        UseTree::Name(_) | UseTree::Rename(_) => false,
    }
}

fn use_tree_root_ident(tree: &UseTree) -> Option<String> {
    match tree {
        UseTree::Path(path) => Some(path.ident.to_string()),
        UseTree::Name(name) => Some(name.ident.to_string()),
        UseTree::Rename(rename) => Some(rename.ident.to_string()),
        UseTree::Group(group) => group.items.first().and_then(use_tree_root_ident),
        UseTree::Glob(_) => None,
    }
}
