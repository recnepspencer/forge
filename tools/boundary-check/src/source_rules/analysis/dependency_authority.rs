//! Path-dependency authority identity for BC7001.
//!
//! Local sealing resolves imports and qualified bounds against the sealed trait
//! inventory of inspectable path dependencies. Inventory is fail-closed:
//! resolution errors propagate; opaque export-generation surfaces in a
//! dependency are errors (not silent omission). One cache is reused for the
//! full transitive traversal (perf: no nested rediscovery).

use super::authority_sealing_surface::FORBIDDEN_TRAITS;
use super::crate_modules::{parse_crate_modules, GovernedCrate, ModuleGraph};
use super::forbidden_aliases::{
    collect_forbidden_aliases_local, cumulative_aliases, AliasInventory,
};
use super::opaque_attributes::first_opaque_attribute;
use super::path_dependencies::path_dependency_roots;
use super::public_reachability::item_name;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use syn::{Attribute, Item};

/// Cached path-dependency graphs and their sealed-name inventories.
#[derive(Default)]
pub(super) struct DepAuthorityCache {
    /// dep root → sealed simple names (publicly projectable).
    sealed_by_root: BTreeMap<PathBuf, BTreeSet<String>>,
    /// dep root → full alias inventory (for nested resolution).
    aliases_by_root: BTreeMap<PathBuf, AliasInventory>,
    graphs_by_root: BTreeMap<PathBuf, ModuleGraph>,
    /// dep root → load error (sticky fail-closed; do not retry as empty).
    errors_by_root: BTreeMap<PathBuf, String>,
}

/// Map of Rust crate idents → sealed simple names + qualified spellings.
pub(super) type DepSealedIndex = BTreeMap<String, BTreeSet<String>>;

/// Build the sealed-export index for every path dependency of `crate_root`.
pub(super) fn dep_sealed_index(
    crate_root: &Path,
    cache: &mut DepAuthorityCache,
) -> Result<DepSealedIndex, String> {
    let path_deps = path_dependency_roots(crate_root)?;
    let mut index = DepSealedIndex::new();
    for (ident, dep_root) in &path_deps {
        let sealed = ensure_dep_sealed(dep_root, cache)?;
        let mut names = sealed.clone();
        for name in &sealed {
            names.insert(format!("{ident}::{name}"));
        }
        index.entry(ident.clone()).or_default().extend(names);
    }
    Ok(index)
}

fn ensure_dep_sealed(
    dep_root: &Path,
    cache: &mut DepAuthorityCache,
) -> Result<BTreeSet<String>, String> {
    if let Some(sealed) = cache.sealed_by_root.get(dep_root) {
        return Ok(sealed.clone());
    }
    if let Some(error) = cache.errors_by_root.get(dep_root) {
        return Err(error.clone());
    }

    let package = dep_root
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("dependency")
        .to_owned();
    let external = GovernedCrate {
        package: package.replace('_', "-"),
        crate_root: dep_root.to_path_buf(),
        relative_crate_root: dep_root.display().to_string(),
    };

    let result = (|| {
        let graph = parse_crate_modules(&external)?;
        // Fail closed: opaque macro/attr export generation cannot be sealed by
        // declaration-shaped alias collection alone.
        if let Some(reason) = opaque_export_generation_risk(&graph) {
            return Err(format!(
                "path dependency `{}` has opaque export-generation surface: {reason}",
                dep_root.display()
            ));
        }
        // Reuse the same cache for transitive path deps (no nested rediscovery).
        let aliases = collect_forbidden_aliases_local(&graph, dep_root, cache)?;
        let sealed = public_sealed_simple_names(&aliases);
        Ok((graph, aliases, sealed))
    })();

    match result {
        Ok((graph, aliases, sealed)) => {
            cache.graphs_by_root.insert(dep_root.to_path_buf(), graph);
            cache
                .aliases_by_root
                .insert(dep_root.to_path_buf(), aliases);
            cache
                .sealed_by_root
                .insert(dep_root.to_path_buf(), sealed.clone());
            Ok(sealed)
        }
        Err(error) => {
            cache
                .errors_by_root
                .insert(dep_root.to_path_buf(), error.clone());
            Err(error)
        }
    }
}

fn public_sealed_simple_names(aliases: &AliasInventory) -> BTreeSet<String> {
    let mut sealed = cumulative_aliases(aliases, &[]);
    for trait_name in FORBIDDEN_TRAITS {
        sealed.insert((*trait_name).to_owned());
    }
    sealed
}

/// Opaque macros/attrs at module scope can mint renamed public authority exports
/// the AST pass cannot seal — including when attached to *private* items or
/// modules (proc expansion is not visibility-preserving). Fail closed for
/// import-identity indexing on every such site.
fn opaque_export_generation_risk(graph: &ModuleGraph) -> Option<String> {
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            match item {
                Item::Macro(item_macro) => {
                    if item_macro.ident.is_none() {
                        // Item-position macro invocation: expansion invisible.
                        return Some(format!(
                            "item-position macro in module `{}`",
                            module_path_display(module_path)
                        ));
                    }
                    // #[macro_export] bodies that mint sealed renames.
                    if has_macro_export(&item_macro.attrs) {
                        let body = item_macro.mac.tokens.to_string();
                        if body_mentions_forbidden_authority(&body) {
                            return Some(format!(
                                "macro_export `{}` mentions sealed authority traits",
                                item_macro
                                    .ident
                                    .as_ref()
                                    .map(|i| i.to_string())
                                    .unwrap_or_else(|| "macro".to_owned())
                            ));
                        }
                    }
                    // Opaque attrs on macros themselves (any visibility).
                    if let Some(attr) = first_opaque_attribute(&item_macro.attrs) {
                        return Some(format!(
                            "opaque attribute `{attr}` on macro `{}` in `{}`",
                            item_macro
                                .ident
                                .as_ref()
                                .map(|i| i.to_string())
                                .unwrap_or_else(|| "macro".to_owned()),
                            module_path_display(module_path)
                        ));
                    }
                }
                other => {
                    // Private roots can still emit public siblings via proc expansion.
                    if let Some(attr) = first_opaque_attribute(item_attrs(other)) {
                        let visibility = if is_public_item(other) {
                            "public"
                        } else {
                            "private"
                        };
                        return Some(format!(
                            "opaque attribute `{attr}` on {visibility} item `{}` in `{}`",
                            item_name(other).unwrap_or_else(|| "item".to_owned()),
                            module_path_display(module_path)
                        ));
                    }
                }
            }
        }
    }
    None
}

fn body_mentions_forbidden_authority(text: &str) -> bool {
    for trait_name in FORBIDDEN_TRAITS {
        if contains_rust_ident(text, trait_name) {
            return true;
        }
    }
    false
}

fn contains_rust_ident(text: &str, ident: &str) -> bool {
    let bytes = text.as_bytes();
    let needle = ident.as_bytes();
    if needle.is_empty() || bytes.len() < needle.len() {
        return false;
    }
    for start in 0..=(bytes.len() - needle.len()) {
        if &bytes[start..start + needle.len()] != needle {
            continue;
        }
        let before_ok = start == 0 || !is_ident_byte(bytes[start - 1]);
        let after = start + needle.len();
        let after_ok = after >= bytes.len() || !is_ident_byte(bytes[after]);
        if before_ok && after_ok {
            return true;
        }
    }
    false
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn has_macro_export(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("macro_export"))
}

fn is_public_item(item: &Item) -> bool {
    use super::crate_modules::is_public_visibility;
    match item {
        Item::Const(i) => is_public_visibility(&i.vis),
        Item::Enum(i) => is_public_visibility(&i.vis),
        Item::Fn(i) => is_public_visibility(&i.vis),
        Item::Mod(i) => is_public_visibility(&i.vis),
        Item::Static(i) => is_public_visibility(&i.vis),
        Item::Struct(i) => is_public_visibility(&i.vis),
        Item::Trait(i) => is_public_visibility(&i.vis),
        Item::TraitAlias(i) => is_public_visibility(&i.vis),
        Item::Type(i) => is_public_visibility(&i.vis),
        Item::Union(i) => is_public_visibility(&i.vis),
        Item::Use(i) => is_public_visibility(&i.vis),
        _ => false,
    }
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::Fn(i) => &i.attrs,
        Item::Impl(i) => &i.attrs,
        Item::Mod(i) => &i.attrs,
        Item::Static(i) => &i.attrs,
        Item::Struct(i) => &i.attrs,
        Item::Trait(i) => &i.attrs,
        Item::TraitAlias(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        Item::Union(i) => &i.attrs,
        Item::Use(i) => &i.attrs,
        Item::Macro(i) => &i.attrs,
        Item::ExternCrate(i) => &i.attrs,
        Item::ForeignMod(i) => &i.attrs,
        _ => &[],
    }
}

fn module_path_display(path: &[String]) -> String {
    if path.is_empty() {
        "crate".to_owned()
    } else {
        path.join("::")
    }
}

/// Whether `source` under path-dep prefix `prefix` is a sealed authority export.
pub(super) fn dep_path_is_sealed(
    prefix: &[String],
    source: &str,
    dep_index: &DepSealedIndex,
) -> bool {
    if prefix.is_empty() {
        return false;
    }
    let crate_ident = &prefix[0];
    let Some(sealed) = dep_index.get(crate_ident) else {
        return false;
    };
    if prefix.len() == 1 {
        return sealed.contains(source) || sealed.contains(&format!("{crate_ident}::{source}"));
    }
    let mut full = prefix.join("::");
    full.push_str("::");
    full.push_str(source);
    sealed.contains(source)
        || sealed.contains(&full)
        || sealed.contains(&format!("{crate_ident}::{source}"))
}
