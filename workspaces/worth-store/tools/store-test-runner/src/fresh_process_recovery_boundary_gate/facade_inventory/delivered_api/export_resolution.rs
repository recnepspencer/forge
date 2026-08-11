use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use syn::Item;

use super::{
    macro_public_enum, module_exports,
    module_paths::module_candidates,
    production_attrs, production_public,
    source_layout::{ModuleGraph, SourceModule},
    CanonicalType, ExportedSurface,
};

pub(super) struct ResolvedDeclaration {
    pub(super) path: PathBuf,
    pub(super) module: Vec<String>,
    pub(super) inline: Vec<(String, usize)>,
    pub(super) name: String,
    pub(super) is_type: bool,
    pub(super) is_module: bool,
}

pub(super) fn resolve_export(
    facade: &SourceModule,
    graph: &ModuleGraph,
    source_root: &Path,
    export: &ExportedSurface,
) -> Result<Vec<ResolvedDeclaration>, String> {
    if export.direct {
        return declarations_named_in_module(facade, graph, &export.source_name);
    }
    let mut visited = BTreeSet::new();
    resolve_use_target(
        facade,
        graph,
        source_root,
        &export.prefix,
        &export.source_name,
        &mut visited,
    )
}

pub(super) fn resolve_glob(
    facade: &SourceModule,
    graph: &ModuleGraph,
    source_root: &Path,
    export: &ExportedSurface,
) -> Result<Vec<(String, ResolvedDeclaration)>, String> {
    let mut visited = BTreeSet::new();
    let mut resolved = Vec::new();
    for components in module_candidates(facade, source_root, &export.prefix) {
        for module in graph.modules_at(&components) {
            resolved.extend(exported_declarations(
                module,
                graph,
                source_root,
                &mut visited,
            )?);
        }
    }
    if resolved.is_empty() {
        return Err(format!(
            "unsupported external or unresolved glob re-export {}; external module re-export aliases are not provable",
            export.prefix.join("::")
        ));
    }
    Ok(resolved)
}

pub(super) fn resolve_impl_type<'a>(
    source: &SourceModule,
    graph: &ModuleGraph,
    source_root: &Path,
    item: &syn::ItemImpl,
    exported: impl Iterator<Item = &'a CanonicalType>,
) -> Result<Vec<CanonicalType>, String> {
    let syn::Type::Path(owner) = item.self_ty.as_ref() else {
        return Ok(Vec::new());
    };
    let exported = exported.cloned().collect::<BTreeSet<_>>();
    let segments = owner
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    let Some((target, prefix)) = segments.split_last() else {
        return Ok(Vec::new());
    };
    let mut visited = BTreeSet::new();
    if !prefix.is_empty() {
        return Ok(
            resolve_use_target(source, graph, source_root, prefix, target, &mut visited)?
                .into_iter()
                .map(canonical_type)
                .filter(|identity| exported.contains(identity))
                .collect(),
        );
    }
    let direct = declarations_named_in_module(source, graph, target)?
        .into_iter()
        .map(canonical_type)
        .filter(|identity| exported.contains(identity))
        .collect::<Vec<_>>();
    if !direct.is_empty() {
        return Ok(direct);
    }
    resolve_imported_type(source, graph, source_root, target, &exported, &mut visited)
}

fn resolve_imported_type(
    source: &SourceModule,
    graph: &ModuleGraph,
    source_root: &Path,
    target: &str,
    exported: &BTreeSet<CanonicalType>,
    visited: &mut BTreeSet<(PathBuf, Vec<(String, usize)>, String)>,
) -> Result<Vec<CanonicalType>, String> {
    let mut identities = BTreeSet::new();
    for item in graph.items(source)? {
        let Item::Use(item) = item else {
            continue;
        };
        if !production_attrs(&item.attrs) {
            continue;
        }
        let mut imports = Vec::new();
        super::facade_exports::collect_use_exports(
            &item.tree,
            &mut Vec::new(),
            &mut imports,
            false,
        )?;
        for import in imports {
            let resolved = if import.glob {
                resolve_use_target(source, graph, source_root, &import.prefix, target, visited)?
            } else if import.export_name == target {
                resolve_use_target(
                    source,
                    graph,
                    source_root,
                    &import.prefix,
                    &import.source_name,
                    visited,
                )?
            } else {
                Vec::new()
            };
            for identity in resolved.into_iter().map(canonical_type) {
                if exported.contains(&identity) {
                    identities.insert(identity);
                }
            }
        }
    }
    Ok(identities.into_iter().collect())
}

fn exported_declarations(
    module: &SourceModule,
    graph: &ModuleGraph,
    source_root: &Path,
    visited: &mut BTreeSet<(PathBuf, Vec<(String, usize)>)>,
) -> Result<Vec<(String, ResolvedDeclaration)>, String> {
    if !visited.insert((module.path.clone(), module.inline.clone())) {
        return Ok(Vec::new());
    }
    let mut resolved = declarations_in_module(module, graph)?
        .into_iter()
        .map(|declaration| (declaration.name.clone(), declaration))
        .collect::<Vec<_>>();
    for export in module_exports(graph, module)? {
        if export.direct {
            continue;
        }
        if export.glob {
            let mut found = false;
            for components in module_candidates(module, source_root, &export.prefix) {
                for child in graph.modules_at(&components) {
                    resolved.extend(exported_declarations(child, graph, source_root, visited)?);
                    found = true;
                }
            }
            if !found {
                resolved.push((
                    "*".to_owned(),
                    unresolved_declaration(source_root, &export.prefix, "*"),
                ));
            }
            continue;
        }
        let mut use_visited = BTreeSet::new();
        let declarations = resolve_use_target(
            module,
            graph,
            source_root,
            &export.prefix,
            &export.source_name,
            &mut use_visited,
        )?;
        if declarations.is_empty() {
            resolved.push((
                export.export_name,
                unresolved_declaration(source_root, &export.prefix, &export.source_name),
            ));
        } else {
            resolved.extend(
                declarations
                    .into_iter()
                    .map(|declaration| (export.export_name.clone(), declaration)),
            );
        }
    }
    Ok(resolved)
}

fn resolve_use_target(
    from_module: &SourceModule,
    graph: &ModuleGraph,
    source_root: &Path,
    prefix: &[String],
    target: &str,
    visited: &mut BTreeSet<(PathBuf, Vec<(String, usize)>, String)>,
) -> Result<Vec<ResolvedDeclaration>, String> {
    let mut resolved = Vec::new();
    for components in module_candidates(from_module, source_root, prefix) {
        for module in graph.modules_at(&components) {
            resolved.extend(resolve_in_module(
                module,
                graph,
                source_root,
                target,
                visited,
            )?);
        }
    }
    Ok(resolved)
}

fn resolve_in_module(
    module: &SourceModule,
    graph: &ModuleGraph,
    source_root: &Path,
    target: &str,
    visited: &mut BTreeSet<(PathBuf, Vec<(String, usize)>, String)>,
) -> Result<Vec<ResolvedDeclaration>, String> {
    let key = (
        module.path.clone(),
        module.inline.clone(),
        target.to_owned(),
    );
    if !visited.insert(key.clone()) {
        return Ok(Vec::new());
    }
    let direct = declarations_named_in_module(module, graph, target)?;
    if !direct.is_empty() {
        return Ok(direct);
    }
    let mut resolved = Vec::new();
    for export in module_exports(graph, module)? {
        if export.direct || export.glob || export.export_name != target {
            continue;
        }
        resolved.extend(resolve_use_target(
            module,
            graph,
            source_root,
            &export.prefix,
            &export.source_name,
            visited,
        )?);
    }
    visited.remove(&key);
    Ok(resolved)
}

pub(super) fn declarations_named_in_module(
    module: &SourceModule,
    graph: &ModuleGraph,
    target: &str,
) -> Result<Vec<ResolvedDeclaration>, String> {
    Ok(declarations_in_module(module, graph)?
        .into_iter()
        .filter(|declaration| declaration.name == target)
        .collect())
}

fn declarations_in_module(
    module: &SourceModule,
    graph: &ModuleGraph,
) -> Result<Vec<ResolvedDeclaration>, String> {
    let mut declarations = Vec::new();
    for item in graph.items(module)? {
        let declaration = match item {
            Item::Const(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.ident.to_string(), false))
            }
            Item::Enum(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.ident.to_string(), true))
            }
            Item::Fn(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.sig.ident.to_string(), false))
            }
            Item::Static(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.ident.to_string(), false))
            }
            Item::Struct(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.ident.to_string(), true))
            }
            Item::Trait(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.ident.to_string(), true))
            }
            Item::Type(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.ident.to_string(), true))
            }
            Item::Union(item) if production_public(&item.vis, &item.attrs) => {
                Some((item.ident.to_string(), true))
            }
            Item::Macro(item) => macro_public_enum(&item).map(|name| (name, true)),
            Item::Mod(item) if production_public(&item.vis, &item.attrs) => {
                let mut logical = module.logical.clone();
                logical.push(item.ident.to_string());
                declarations.extend(
                    graph
                        .modules_at(&logical)
                        .iter()
                        .filter(|child| child.publicly_reachable)
                        .map(|child| ResolvedDeclaration {
                            path: child.path.clone(),
                            module: child.logical.clone(),
                            inline: child.inline.clone(),
                            name: item.ident.to_string(),
                            is_type: false,
                            is_module: true,
                        }),
                );
                None
            }
            _ => None,
        };
        if let Some((name, is_type)) = declaration {
            declarations.push(ResolvedDeclaration {
                path: module.path.clone(),
                module: module.logical.clone(),
                inline: module.inline.clone(),
                name,
                is_type,
                is_module: false,
            });
        }
    }
    Ok(declarations)
}

fn canonical_type(declaration: ResolvedDeclaration) -> CanonicalType {
    CanonicalType {
        path: declaration.path,
        module: declaration.module,
        inline: declaration.inline,
        name: declaration.name,
    }
}

fn unresolved_declaration(
    source_root: &Path,
    prefix: &[String],
    name: &str,
) -> ResolvedDeclaration {
    let mut path = source_root.join("__external__");
    for component in prefix {
        path.push(component);
    }
    path.set_extension("rs");
    ResolvedDeclaration {
        path,
        module: prefix.to_vec(),
        inline: Vec::new(),
        name: name.to_owned(),
        is_type: false,
        is_module: false,
    }
}
