use std::path::Path;

use syn::{Item, UseTree};

use super::{
    parse_source, production_public,
    source_layout::{ModuleGraph, SourceModule},
    ExportedSurface,
};

pub(super) fn facade_exports(path: &Path) -> Result<Vec<ExportedSurface>, String> {
    exports_from_items(parse_source(path)?.items, true)
}

pub(super) fn module_exports(
    graph: &ModuleGraph,
    module: &SourceModule,
) -> Result<Vec<ExportedSurface>, String> {
    exports_from_items(graph.items(module)?, module.publicly_reachable)
}

pub(super) fn validate_macro_exports(graph: &ModuleGraph) -> Result<(), String> {
    for module in graph.modules() {
        for item in graph.items(module)? {
            let Item::Macro(item) = item else {
                continue;
            };
            if super::production_attrs(&item.attrs) && exported_macro_definition(&item) {
                return Err(format!(
                    "unsupported exported macro definition in {}; public expansion is not provable",
                    module.path.display()
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn collect_use_exports(
    tree: &UseTree,
    prefix: &mut Vec<String>,
    exports: &mut Vec<ExportedSurface>,
    reject_module_aliases: bool,
) -> Result<(), String> {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_exports(&path.tree, prefix, exports, reject_module_aliases)?;
            prefix.pop();
        }
        UseTree::Name(name) => {
            let name = name.ident.to_string();
            push_named_export(prefix, name.clone(), name, exports, reject_module_aliases)?;
        }
        UseTree::Rename(rename) => push_named_export(
            prefix,
            rename.ident.to_string(),
            rename.rename.to_string(),
            exports,
            reject_module_aliases,
        )?,
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_use_exports(tree, prefix, exports, reject_module_aliases)?;
            }
        }
        UseTree::Glob(_) => push_export(
            prefix.clone(),
            "*".to_owned(),
            "*".to_owned(),
            true,
            exports,
        ),
    }
    Ok(())
}

fn push_named_export(
    prefix: &[String],
    source_name: String,
    export_name: String,
    exports: &mut Vec<ExportedSurface>,
    reject_module_aliases: bool,
) -> Result<(), String> {
    if reject_module_aliases && matches!(source_name.as_str(), "crate" | "self" | "super") {
        return Err(format!(
            "unsupported module re-export alias through reserved namespace {source_name} at {}; namespace projection is not provable",
            prefix.join("::")
        ));
    }
    if source_name == "self" {
        let mut module_prefix = prefix.to_vec();
        let module_name = module_prefix
            .pop()
            .ok_or_else(|| "unsupported unqualified self module import".to_owned())?;
        let export_name = if export_name == "self" {
            module_name.clone()
        } else {
            export_name
        };
        push_export(module_prefix, module_name, export_name, false, exports);
        return Ok(());
    }
    push_export(prefix.to_vec(), source_name, export_name, false, exports);
    Ok(())
}

fn exports_from_items(
    items: Vec<Item>,
    reject_unresolved_public_surfaces: bool,
) -> Result<Vec<ExportedSurface>, String> {
    let mut exports = Vec::new();
    for item in items {
        if let Item::Use(item) = &item {
            if production_public(&item.vis, &item.attrs) {
                collect_use_exports(&item.tree, &mut Vec::new(), &mut exports, true)?;
            }
            continue;
        }
        reject_unresolved_public_item(&item, reject_unresolved_public_surfaces)?;
        let Some(name) = direct_public_name(&item) else {
            continue;
        };
        push_export(Vec::new(), name.clone(), name, false, &mut exports);
        exports.last_mut().expect("direct export").direct = true;
    }
    Ok(exports)
}

fn direct_public_name(item: &Item) -> Option<String> {
    match item {
        Item::Const(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        Item::Enum(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        Item::Fn(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.sig.ident.to_string())
        }
        Item::Struct(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        Item::Static(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        Item::Trait(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        Item::TraitAlias(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        Item::Type(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        Item::Union(item) if production_public(&item.vis, &item.attrs) => {
            Some(item.ident.to_string())
        }
        _ => None,
    }
}

fn reject_unresolved_public_item(item: &Item, reject_reachable: bool) -> Result<(), String> {
    match item {
        Item::ExternCrate(item) if production_public(&item.vis, &item.attrs) => Err(format!(
            "unsupported public extern-crate export {}; provenance is not provable",
            item.rename
                .as_ref()
                .map(|(_, rename)| rename)
                .unwrap_or(&item.ident)
        )),
        Item::ForeignMod(item)
            if reject_reachable
                && super::production_attrs(&item.attrs)
                && foreign_module_can_export(item) =>
        {
            Err("unsupported public foreign-module surface; provenance is not provable".to_owned())
        }
        Item::Verbatim(tokens) if reject_reachable && !tokens.is_empty() => {
            Err("unsupported facade syntax; public surface is not provable".to_owned())
        }
        Item::Macro(item)
            if super::production_attrs(&item.attrs)
                && (exported_macro_definition(item)
                    || (reject_reachable && !private_macro_definition(item))) =>
        {
            let name = item
                .mac
                .path
                .segments
                .last()
                .map(|part| part.ident.to_string())
                .unwrap_or_else(|| "<macro>".to_owned());
            Err(format!(
                "unsupported facade macro invocation {name}; public expansion is not provable"
            ))
        }
        _ => Ok(()),
    }
}

fn foreign_module_can_export(item: &syn::ItemForeignMod) -> bool {
    item.items.iter().any(|item| match item {
        syn::ForeignItem::Fn(item) => production_public(&item.vis, &item.attrs),
        syn::ForeignItem::Static(item) => production_public(&item.vis, &item.attrs),
        syn::ForeignItem::Type(item) => production_public(&item.vis, &item.attrs),
        syn::ForeignItem::Macro(item) => super::production_attrs(&item.attrs),
        syn::ForeignItem::Verbatim(tokens) => !tokens.is_empty(),
        _ => true,
    })
}

fn private_macro_definition(item: &syn::ItemMacro) -> bool {
    item.mac.path.is_ident("macro_rules")
        && item.ident.is_some()
        && !exported_macro_definition(item)
}

fn exported_macro_definition(item: &syn::ItemMacro) -> bool {
    item.mac.path.is_ident("macro_rules")
        && item.ident.is_some()
        && item.attrs.iter().any(attribute_can_export_macro)
}

fn attribute_can_export_macro(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident("macro_export")
        || (attribute.path().is_ident("cfg_attr")
            && tokens_name_macro_export(match &attribute.meta {
                syn::Meta::List(list) => list.tokens.clone(),
                _ => return false,
            }))
}

fn tokens_name_macro_export(tokens: proc_macro2::TokenStream) -> bool {
    tokens.into_iter().any(|token| match token {
        proc_macro2::TokenTree::Ident(ident) => ident == "macro_export",
        proc_macro2::TokenTree::Group(group) => tokens_name_macro_export(group.stream()),
        _ => false,
    })
}

fn push_export(
    prefix: Vec<String>,
    source_name: String,
    export_name: String,
    glob: bool,
    exports: &mut Vec<ExportedSurface>,
) {
    exports.push(ExportedSurface {
        prefix,
        source_name,
        export_name,
        direct: false,
        glob,
    });
}
