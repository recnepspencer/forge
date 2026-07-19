//! Leaf-facade contract for configured Query audience crates.
//!
//! Validates that each matrix audience package under the configured Query
//! workspace is a zero-behavior re-export leaf over the configured engine: one
//! direct engine dependency, facade-only lib surface, and re-export-only facade
//! module. The optional certification package is a cold leaf over the engine.

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
    let query_crates_root = root.join(&contract.workspace).join("crates");

    for audience in &contract.audiences {
        let crate_root = query_crates_root.join(&audience.package);
        let relative = relative_to_root(root, &crate_root);
        if !crate_root.is_dir() {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                relative,
                format!(
                    "configured Query audience facade `{}` is missing under `{}/crates`",
                    audience.package, contract.workspace
                ),
            ));
            continue;
        }

        let authority_packages = if audience.authority_packages.is_empty() {
            vec![contract.engine_package.clone()]
        } else {
            audience.authority_packages.clone()
        };
        diagnostics.extend(validate_authority_dependencies(
            &crate_root,
            &relative,
            &authority_packages,
            &audience_packages,
        )?);
        diagnostics.extend(validate_facade_only_lib(
            &crate_root.join("src/lib.rs"),
            &relative,
        )?);
        diagnostics.extend(validate_reexport_only_facade(
            &crate_root.join("src/facade.rs"),
            &relative,
            &authority_packages,
            &audience_packages,
        )?);
    }

    if let Some(certification_package) = &contract.certification_package {
        diagnostics.extend(validate_cold_certification_leaf(
            root,
            &query_crates_root,
            certification_package,
            contract,
        )?);
    }

    Ok(diagnostics)
}

fn validate_cold_certification_leaf(
    root: &Path,
    query_crates_root: &Path,
    certification_package: &str,
    contract: &QueryAudienceContract,
) -> Result<Vec<Diagnostic>, String> {
    let certification_root = query_crates_root.join(certification_package);
    let certification_relative = relative_to_root(root, &certification_root);
    let mut diagnostics = Vec::new();

    if !certification_root.is_dir() {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
            certification_relative,
            format!(
                "configured Query certification package `{certification_package}` is missing from the Query workspace"
            ),
        ));
        return Ok(diagnostics);
    }

    let certification_manifest = certification_root.join("Cargo.toml");
    let certification_text = fs::read_to_string(&certification_manifest)
        .map_err(|e| format!("read {}: {e}", certification_manifest.display()))?;
    let certification_value: toml::Value = toml::from_str(&certification_text)
        .map_err(|e| format!("parse {}: {e}", certification_manifest.display()))?;
    let certification_dependencies = direct_dependency_package_names(&certification_value);
    if certification_dependencies != [contract.engine_package.clone()] {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
            format!("{certification_relative}/Cargo.toml"),
            format!(
                "Query certification must be a cold leaf over engine package `{}`; found normal dependencies: {}",
                contract.engine_package,
                if certification_dependencies.is_empty() {
                    "none".to_owned()
                } else {
                    certification_dependencies.join(", ")
                }
            ),
        ));
    }

    let mut ordinary_packages = vec![contract.engine_package.as_str()];
    ordinary_packages.extend(
        contract
            .audiences
            .iter()
            .filter(|audience| audience.label != "replay")
            .map(|audience| audience.package.as_str()),
    );
    for ordinary_package in ordinary_packages {
        let manifest_path = query_crates_root.join(ordinary_package).join("Cargo.toml");
        let text = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("read {}: {e}", manifest_path.display()))?;
        let value: toml::Value =
            toml::from_str(&text).map_err(|e| format!("parse {}: {e}", manifest_path.display()))?;
        if all_direct_dependency_package_names(&value)
            .iter()
            .any(|dependency| dependency == certification_package)
        {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                relative_to_root(root, &manifest_path),
                format!(
                    "ordinary Query package `{ordinary_package}` must not depend on cold certification `{certification_package}`"
                ),
            ));
        }
    }

    Ok(diagnostics)
}

fn validate_authority_dependencies(
    crate_root: &Path,
    relative: &str,
    authority_packages: &[String],
    audience_packages: &[&str],
) -> Result<Vec<Diagnostic>, String> {
    let manifest_path = crate_root.join("Cargo.toml");
    let text = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("read {}: {e}", manifest_path.display()))?;
    let value: toml::Value =
        toml::from_str(&text).map_err(|e| format!("parse {}: {e}", manifest_path.display()))?;
    let deps = direct_dependency_package_names(&value);
    let mut diagnostics = Vec::new();

    let mut expected = authority_packages.to_vec();
    expected.sort();
    if deps != expected {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
            format!("{relative}/Cargo.toml"),
            format!(
                "Query audience facade must depend on exactly {}; found dependencies: {}",
                expected.join(", "),
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

fn all_direct_dependency_package_names(manifest: &toml::Value) -> Vec<String> {
    let mut names = Vec::new();
    for table_name in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(dependencies) = manifest.get(table_name).and_then(toml::Value::as_table) else {
            continue;
        };
        names.extend(dependencies.iter().map(|(key, spec)| {
            spec.as_table()
                .and_then(|table| table.get("package"))
                .and_then(toml::Value::as_str)
                .unwrap_or(key)
                .to_owned()
        }));
    }
    names.sort();
    names.dedup();
    names
}

fn relative_to_root(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
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
    authority_packages: &[String],
    audience_packages: &[&str],
) -> Result<Vec<Diagnostic>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let syntax = syn::parse_file(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
    let mut diagnostics = Vec::new();
    let authority_crates = authority_packages
        .iter()
        .map(|package| package.replace('-', "_"))
        .collect::<Vec<_>>();

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
                    if !authority_crates.iter().any(|allowed| allowed == &root) {
                        diagnostics.push(Diagnostic::new(
                            DiagnosticCode::Bc3003QueryAudienceFacadeContract,
                            format!("{relative}/src/facade.rs"),
                            format!(
                                "Query audience facade.rs may re-export only from configured authority crates {}; found `{root}`",
                                authority_crates.join(", ")
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
