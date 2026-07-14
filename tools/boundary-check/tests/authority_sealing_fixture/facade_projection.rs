//! Projects fixture source declarations onto a Phase 6-compatible facade.

use quote::ToTokens;
use std::collections::BTreeSet;

pub(super) fn facade_reexports(source: &str) -> String {
    facade_reexports_with_dependency(source, None)
}

pub(super) fn facade_reexports_with_external(
    source: &str,
    package: &str,
    dependency_source: &str,
) -> String {
    facade_reexports_with_dependency(source, Some((package, dependency_source)))
}

fn facade_reexports_with_dependency(source: &str, external: Option<(&str, &str)>) -> String {
    let syntax = syn::parse_file(source).expect("parse fixture surface");
    let local_modules = syntax
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(value) => Some(value.ident.to_string()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let public_uses = syntax.items.iter().filter_map(|item| match item {
        syn::Item::Use(value) if public(&value.vis) => {
            let rendered = value.to_token_stream().to_string();
            let root = use_root(&value.tree);
            let external_glob = matches!(
                &value.tree,
                syn::UseTree::Path(path) if matches!(&*path.tree, syn::UseTree::Glob(_))
            );
            if external_glob
                && external
                    .is_some_and(|(package, _)| root.as_deref() == Some(&package.replace('-', "_")))
            {
                let (package, dependency_source) = external.expect("checked external");
                let dependency =
                    syn::parse_file(dependency_source).expect("parse fixture dependency");
                return Some(
                    dependency_export_names(&dependency.items)
                        .into_iter()
                        .map(|name| format!("pub use {}::{name};\n", package.replace('-', "_")))
                        .collect::<String>(),
                );
            }
            if root
                .as_ref()
                .is_some_and(|name| local_modules.contains(name))
                || matches!(root.as_deref(), Some("self" | "super" | "crate"))
            {
                Some(format!(
                    "pub use crate::test_surface::{};\n",
                    value.tree.to_token_stream()
                ))
            } else {
                Some(format!("{rendered}\n"))
            }
        }
        _ => None,
    });
    let names = syntax.items.iter().flat_map(public_item_names);
    public_uses
        .chain(names.map(|name| format!("pub use crate::test_surface::{name};\n")))
        .collect()
}

fn dependency_export_names(items: &[syn::Item]) -> BTreeSet<String> {
    let mut names = items
        .iter()
        .flat_map(public_item_names)
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    for item_use in items.iter().filter_map(|item| match item {
        syn::Item::Use(value) if public(&value.vis) => Some(value),
        _ => None,
    }) {
        collect_dependency_use_exports(&item_use.tree, items, &mut names);
    }
    names
}

fn collect_dependency_use_exports(
    tree: &syn::UseTree,
    items: &[syn::Item],
    names: &mut BTreeSet<String>,
) {
    match tree {
        syn::UseTree::Name(value) => {
            names.insert(value.ident.to_string());
        }
        syn::UseTree::Rename(value) => {
            names.insert(value.rename.to_string());
        }
        syn::UseTree::Group(group) => {
            for child in &group.items {
                collect_dependency_use_exports(child, items, names);
            }
        }
        syn::UseTree::Path(path) if matches!(&*path.tree, syn::UseTree::Glob(_)) => {
            if let Some(syn::Item::Mod(module)) = items
                .iter()
                .find(|item| matches!(item, syn::Item::Mod(module) if module.ident == path.ident))
            {
                if let Some((_, nested)) = &module.content {
                    names.extend(dependency_export_names(nested));
                }
            }
        }
        syn::UseTree::Path(path) => collect_dependency_use_exports(&path.tree, items, names),
        syn::UseTree::Glob(_) => {}
    }
}

fn use_root(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(path) => Some(path.ident.to_string()),
        syn::UseTree::Name(name) => Some(name.ident.to_string()),
        syn::UseTree::Rename(rename) => Some(rename.ident.to_string()),
        syn::UseTree::Glob(_) | syn::UseTree::Group(_) => None,
    }
}

fn public_item_names(item: &syn::Item) -> Vec<&syn::Ident> {
    match item {
        syn::Item::Const(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::Enum(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::Fn(value) if public(&value.vis) => vec![&value.sig.ident],
        syn::Item::Mod(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::Static(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::Struct(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::Trait(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::Type(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::Union(value) if public(&value.vis) => vec![&value.ident],
        syn::Item::ForeignMod(value) => {
            value.items.iter().filter_map(public_foreign_name).collect()
        }
        _ => Vec::new(),
    }
}

fn public_foreign_name(item: &syn::ForeignItem) -> Option<&syn::Ident> {
    match item {
        syn::ForeignItem::Fn(value) if public(&value.vis) && !value.attrs.is_empty() => {
            Some(&value.sig.ident)
        }
        syn::ForeignItem::Static(value) if public(&value.vis) && !value.attrs.is_empty() => {
            Some(&value.ident)
        }
        _ => None,
    }
}

fn public(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}
