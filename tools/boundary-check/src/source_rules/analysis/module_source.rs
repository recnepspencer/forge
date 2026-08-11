//! Rustc-compatible module source selection for the sealing inventory.
//!
//! `#[path]` is resolved from the containing source file's physical directory;
//! inline module nesting appends its virtual components to that base.
//! Every selectable `path` / `cfg_attr(..., path = ...)` branch is collected
//! fail-closed so conditional path selection cannot hide a hostile body.

use std::path::{Path, PathBuf};
use syn::{Attribute, Lit, Meta};

/// Virtual directory of a module for resolving child `#[path]` and out-of-line files.
pub(super) fn child_module_dir(parent_dir: &Path, child_name: &str) -> PathBuf {
    parent_dir.join(child_name)
}

/// Directory used for nested resolution after loading a source file.
pub(super) fn directory_after_loading_file(source_path: &Path) -> PathBuf {
    if source_path.file_name().and_then(|s| s.to_str()) == Some("mod.rs") {
        source_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    } else {
        // `foo.rs` → virtual directory `foo/`
        source_path.with_extension("")
    }
}

/// Base directory rustc uses for `#[path]` inside the current source module.
///
/// Conventional `foo.rs` children live under `foo/`, but a path attribute in
/// `foo.rs` is relative to the physical directory containing `foo.rs`. Inline
/// module nesting is retained on top of that physical directory.
pub(super) fn path_attribute_dir(parent_source: &Path, module_dir: &Path) -> PathBuf {
    let conventional_base = directory_after_loading_file(parent_source);
    let Some(physical_parent) = parent_source.parent() else {
        return module_dir.to_path_buf();
    };
    match module_dir.strip_prefix(&conventional_base) {
        Ok(inline_suffix) => physical_parent.join(inline_suffix),
        Err(_) => module_dir.to_path_buf(),
    }
}

/// All module path selectors on a `mod` item: direct `#[path]` and nested
/// `cfg_attr(..., path = "...")` branches (fail-closed inventory of every branch).
pub(super) fn all_path_selectors(attrs: &[Attribute]) -> Vec<String> {
    let mut out = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("path") {
            if let Some(value) = path_attr_string(attr) {
                out.push(value);
            }
            continue;
        }
        if attr.path().is_ident("cfg_attr") {
            collect_cfg_attr_paths(attr, &mut out);
        }
    }
    out.sort();
    out.dedup();
    out
}

fn path_attr_string(attr: &Attribute) -> Option<String> {
    match &attr.meta {
        Meta::NameValue(nv) => {
            if let syn::Expr::Lit(expr_lit) = &nv.value {
                if let Lit::Str(s) = &expr_lit.lit {
                    return Some(s.value());
                }
            }
            None
        }
        Meta::List(list) => syn::parse2::<syn::LitStr>(list.tokens.clone())
            .ok()
            .map(|s| s.value()),
        _ => None,
    }
}

fn collect_cfg_attr_paths(attr: &Attribute, out: &mut Vec<String>) {
    let Meta::List(list) = &attr.meta else {
        return;
    };
    let groups = split_top_level_comma_groups(list.tokens.clone());
    // Skip predicate (first group); remaining groups are nested attributes.
    for nested in groups.into_iter().skip(1) {
        collect_path_from_nested_tokens(nested, out);
    }
}

fn collect_path_from_nested_tokens(tokens: proc_macro2::TokenStream, out: &mut Vec<String>) {
    let Ok(meta) = syn::parse2::<Meta>(tokens) else {
        return;
    };
    match &meta {
        Meta::Path(path) if path.is_ident("path") => {}
        Meta::NameValue(nv) if nv.path.is_ident("path") => {
            if let syn::Expr::Lit(expr_lit) = &nv.value {
                if let Lit::Str(s) = &expr_lit.lit {
                    out.push(s.value());
                }
            }
        }
        Meta::List(list) if list.path.is_ident("path") => {
            if let Ok(s) = syn::parse2::<syn::LitStr>(list.tokens.clone()) {
                out.push(s.value());
            }
        }
        Meta::List(list) if list.path.is_ident("cfg_attr") => {
            let groups = split_top_level_comma_groups(list.tokens.clone());
            for nested in groups.into_iter().skip(1) {
                collect_path_from_nested_tokens(nested, out);
            }
        }
        _ => {}
    }
}

fn split_top_level_comma_groups(tokens: proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    use proc_macro2::TokenTree;
    let mut groups = Vec::new();
    let mut current = Vec::new();
    for tree in tokens {
        match &tree {
            TokenTree::Punct(punct) if punct.as_char() == ',' => {
                groups.push(current.into_iter().collect());
                current = Vec::new();
            }
            _ => current.push(tree),
        }
    }
    if !current.is_empty() {
        groups.push(current.into_iter().collect());
    }
    groups
}

/// Resolve out-of-line module sources for a child `mod` under its two rustc
/// bases: the conventional module directory and the path-attribute directory.
///
/// Returns every concrete file that may be compiled: all path-attribute targets
/// plus conventional candidates when present (covers conditional `cfg_attr(path)`).
pub(super) fn resolve_child_sources(
    conventional_parent_dir: &Path,
    path_attribute_parent_dir: &Path,
    child_name: &str,
    attrs: &[Attribute],
) -> Result<Vec<PathBuf>, String> {
    let mut sources = Vec::new();
    let selectors = all_path_selectors(attrs);
    for rel in &selectors {
        let resolved = path_attribute_parent_dir.join(rel);
        if resolved.is_file() {
            sources.push(resolved);
        } else {
            return Err(format!(
                "unresolved #[path = \"{rel}\"] for module `{child_name}` under {}",
                path_attribute_parent_dir.display()
            ));
        }
    }

    // Conventional locations always inventoried when present so a cfg_attr(path)
    // that is inactive still cannot hide behind a decoy-only conventional scan,
    // and so path-less modules resolve normally.
    let conventional = [
        conventional_parent_dir.join(child_name).join("mod.rs"),
        conventional_parent_dir.join(format!("{child_name}.rs")),
    ];
    for candidate in conventional {
        if candidate.is_file() && !sources.iter().any(|s| s == &candidate) {
            sources.push(candidate);
        }
    }

    if sources.is_empty() {
        return Err(format!(
            "unresolved module `{child_name}` under {}; expected path attribute or conventional file",
            conventional_parent_dir.display()
        ));
    }
    Ok(sources)
}

/// Resolve the single source rustc selects after a Cargo world projected cfg/cfg_attr.
pub(super) fn resolve_selected_child_source(
    conventional_parent_dir: &Path,
    path_attribute_parent_dir: &Path,
    child_name: &str,
    attrs: &[Attribute],
) -> Result<(PathBuf, bool), String> {
    let selectors = all_path_selectors(attrs);
    if selectors.len() > 1 {
        return Err(format!(
            "module `{child_name}` has multiple active path selectors: {}",
            selectors.join(", ")
        ));
    }
    if let Some(selector) = selectors.first() {
        let selected = path_attribute_parent_dir.join(selector);
        if selected.is_file() {
            return Ok((selected, true));
        }
        return Err(format!(
            "unresolved active #[path = \"{selector}\"] for module `{child_name}` under {}",
            path_attribute_parent_dir.display()
        ));
    }
    let candidates = [
        conventional_parent_dir.join(child_name).join("mod.rs"),
        conventional_parent_dir.join(format!("{child_name}.rs")),
    ]
    .into_iter()
    .filter(|candidate| candidate.is_file())
    .collect::<Vec<_>>();
    match candidates.as_slice() {
        [source] => Ok((source.clone(), false)),
        [] => Err(format!(
            "unresolved module `{child_name}` under {}; expected one conventional file",
            conventional_parent_dir.display()
        )),
        _ => Err(format!(
            "module `{child_name}` has both conventional source files under {}",
            conventional_parent_dir.display()
        )),
    }
}
