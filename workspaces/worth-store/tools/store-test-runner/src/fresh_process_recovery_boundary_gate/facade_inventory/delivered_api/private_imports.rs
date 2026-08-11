use syn::Item;

use super::{
    facade_exports::collect_use_exports,
    production_attrs, production_public,
    source_layout::{ModuleGraph, SourceModule},
    ExportedSurface,
};

pub(super) fn reject_private_import_alias(
    source: &SourceModule,
    graph: &ModuleGraph,
    export: &ExportedSurface,
) -> Result<(), String> {
    let mut scope = source.logical.clone();
    let mut components = export.prefix.as_slice();
    let explicitly_scoped = components
        .first()
        .is_some_and(|part| matches!(part.as_str(), "crate" | "self" | "super"));
    if components.first().is_some_and(|part| part == "crate") {
        scope.clear();
        components = &components[1..];
    } else if components.first().is_some_and(|part| part == "self") {
        components = &components[1..];
    } else {
        while components.first().is_some_and(|part| part == "super") {
            scope.pop();
            components = &components[1..];
        }
    }
    if !explicitly_scoped {
        let first_binding = components.first().unwrap_or(&export.source_name);
        reject_binding_in_lexical_scopes(graph, &scope, first_binding)?;
    }
    for component in components {
        reject_binding_in_scope(graph, &scope, component)?;
        scope.push(component.clone());
    }
    reject_binding_in_scope(graph, &scope, &export.source_name)
}

fn reject_binding_in_lexical_scopes(
    graph: &ModuleGraph,
    scope: &[String],
    binding: &str,
) -> Result<(), String> {
    let mut candidate = scope.to_vec();
    loop {
        reject_binding_in_scope(graph, &candidate, binding)?;
        if candidate.pop().is_none() {
            return Ok(());
        }
    }
}

fn reject_binding_in_scope(
    graph: &ModuleGraph,
    scope: &[String],
    binding: &str,
) -> Result<(), String> {
    for source in graph.modules_at(scope) {
        if source_has_private_binding(graph, source, binding)? {
            return Err(format!(
                "unsupported public re-export through private import alias {binding}; canonical namespace provenance is not provable"
            ));
        }
    }
    Ok(())
}

fn source_has_private_binding(
    graph: &ModuleGraph,
    source: &SourceModule,
    binding: &str,
) -> Result<bool, String> {
    for item in graph.items(source)? {
        match item {
            Item::Use(item)
                if production_attrs(&item.attrs) && !production_public(&item.vis, &item.attrs) =>
            {
                let mut imports = Vec::new();
                collect_use_exports(&item.tree, &mut Vec::new(), &mut imports, false)?;
                if imports
                    .iter()
                    .any(|import| import.glob || import.export_name == binding)
                {
                    return Ok(true);
                }
            }
            Item::ExternCrate(item)
                if production_attrs(&item.attrs) && !production_public(&item.vis, &item.attrs) =>
            {
                let extern_binding = item
                    .rename
                    .as_ref()
                    .map(|(_, rename)| rename)
                    .unwrap_or(&item.ident)
                    .to_string();
                if extern_binding == binding {
                    return Ok(true);
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
