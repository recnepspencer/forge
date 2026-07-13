//! Canonical compiled-library observation shared by constitutional source rules.

use super::crate_modules::{parse_crate_modules, GovernedCrate};
use crate::cargo_graph::package_name_from_manifest;
use std::path::{Path, PathBuf};
use syn::Item;

pub(crate) struct CompiledLibrarySurface {
    pub(crate) library_source: PathBuf,
    pub(crate) exported_macro_sources: Vec<String>,
}

pub(crate) fn observe_compiled_library_surface(
    manifest: &Path,
) -> Result<CompiledLibrarySurface, String> {
    let crate_root = manifest
        .parent()
        .ok_or_else(|| format!("{} has no crate directory", manifest.display()))?
        .to_path_buf();
    let governed = GovernedCrate {
        package: package_name_from_manifest(manifest)?,
        crate_root: crate_root.clone(),
        relative_crate_root: crate_root.display().to_string(),
    };
    let library_source = super::crate_modules::resolve_lib_source_path(&crate_root)?;
    let graph = parse_crate_modules(&governed)?;
    let exported_macro_sources = graph
        .modules
        .values()
        .filter(|node| node.items.iter().any(is_exported_macro))
        .map(|node| node.relative_source.clone())
        .collect();
    Ok(CompiledLibrarySurface {
        library_source,
        exported_macro_sources,
    })
}

fn is_exported_macro(item: &Item) -> bool {
    let Item::Macro(item_macro) = item else {
        return false;
    };
    item_macro
        .attrs
        .iter()
        .any(|attribute| attribute.path().is_ident("macro_export"))
}
