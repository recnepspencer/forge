//! Project parsed items into one exact Cargo production world.

use super::crate_modules::{is_public_visibility, ModuleGraph, ModuleNode};
pub(super) use super::production_cargo_world::ProductionWorld;
use std::collections::BTreeMap;
use syn::{Fields, ImplItem, Item, TraitItem, Variant};

pub(super) fn project(graph: &ModuleGraph, world: &ProductionWorld) -> ModuleGraph {
    let modules = graph
        .modules
        .iter()
        .filter(|(path, _)| module_exists(graph, path, world))
        .map(|(path, node)| {
            (
                path.clone(),
                ModuleNode {
                    relative_source: node.relative_source.clone(),
                    public_from_parent: module_is_public(graph, path, node, world),
                    items: node
                        .items
                        .iter()
                        .filter_map(|item| project_item(item, world))
                        .collect(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    ModuleGraph { modules }
}

fn module_exists(graph: &ModuleGraph, path: &[String], world: &ProductionWorld) -> bool {
    (1..=path.len()).all(|depth| {
        let parent = &path[..depth - 1];
        let child = &path[depth - 1];
        graph.modules.get(parent).is_some_and(|node| {
            let declarations = node.items.iter().filter_map(|item| match item {
                Item::Mod(item_mod) if item_mod.ident == child.as_str() => Some(item_mod),
                _ => None,
            });
            let declarations = declarations.collect::<Vec<_>>();
            declarations.is_empty()
                || declarations
                    .iter()
                    .any(|declaration| world.includes(&declaration.attrs))
        })
    })
}

fn module_is_public(
    graph: &ModuleGraph,
    path: &[String],
    node: &ModuleNode,
    world: &ProductionWorld,
) -> bool {
    let Some((child, parent_path)) = path.split_last() else {
        return true;
    };
    let Some(parent) = graph.modules.get(parent_path) else {
        return node.public_from_parent;
    };
    let declarations = parent.items.iter().filter_map(|item| match item {
        Item::Mod(item_mod)
            if item_mod.ident == child.as_str() && world.includes(&item_mod.attrs) =>
        {
            Some(item_mod)
        }
        _ => None,
    });
    let declarations = declarations.collect::<Vec<_>>();
    !declarations.is_empty()
        && declarations
            .iter()
            .any(|declaration| is_public_visibility(&declaration.vis))
}

fn project_item(item: &Item, world: &ProductionWorld) -> Option<Item> {
    if !world.includes(super::production_availability::item_attributes(item)) {
        return None;
    }
    let mut item = item.clone();
    project_item_attributes(&mut item, world);
    match &mut item {
        Item::Struct(item) => project_fields(&mut item.fields, world),
        Item::Enum(item) => {
            item.variants = item
                .variants
                .iter()
                .filter(|variant| world.includes(&variant.attrs))
                .cloned()
                .map(|mut variant: Variant| {
                    variant.attrs = world.project_attributes(&variant.attrs);
                    project_fields(&mut variant.fields, world);
                    variant
                })
                .collect();
        }
        Item::Union(item) => {
            item.fields.named = item
                .fields
                .named
                .iter()
                .filter(|field| world.includes(&field.attrs))
                .cloned()
                .map(|mut field| {
                    field.attrs = world.project_attributes(&field.attrs);
                    field
                })
                .collect();
        }
        Item::Impl(item) => project_impl_items(item, world),
        Item::Trait(item) => project_trait_items(item, world),
        _ => {}
    }
    Some(item)
}

fn project_impl_items(item: &mut syn::ItemImpl, world: &ProductionWorld) {
    item.items = item
        .items
        .iter()
        .filter(|member| world.includes(impl_item_attributes(member)))
        .cloned()
        .map(|mut member| {
            project_impl_item_attributes(&mut member, world);
            member
        })
        .collect();
}

fn project_trait_items(item: &mut syn::ItemTrait, world: &ProductionWorld) {
    item.items = item
        .items
        .iter()
        .filter(|member| world.includes(trait_item_attributes(member)))
        .cloned()
        .map(|mut member| {
            project_trait_item_attributes(&mut member, world);
            member
        })
        .collect();
}

fn project_fields(fields: &mut Fields, world: &ProductionWorld) {
    match fields {
        Fields::Named(fields) => {
            fields.named = fields
                .named
                .iter()
                .filter(|field| world.includes(&field.attrs))
                .cloned()
                .map(|mut field| {
                    field.attrs = world.project_attributes(&field.attrs);
                    field
                })
                .collect();
        }
        Fields::Unnamed(fields) => {
            fields.unnamed = fields
                .unnamed
                .iter()
                .filter(|field| world.includes(&field.attrs))
                .cloned()
                .map(|mut field| {
                    field.attrs = world.project_attributes(&field.attrs);
                    field
                })
                .collect();
        }
        Fields::Unit => {}
    }
}

fn project_item_attributes(item: &mut Item, world: &ProductionWorld) {
    let attributes = match item {
        Item::Const(item) => &mut item.attrs,
        Item::Enum(item) => &mut item.attrs,
        Item::ExternCrate(item) => &mut item.attrs,
        Item::Fn(item) => &mut item.attrs,
        Item::ForeignMod(item) => &mut item.attrs,
        Item::Impl(item) => &mut item.attrs,
        Item::Macro(item) => &mut item.attrs,
        Item::Mod(item) => &mut item.attrs,
        Item::Static(item) => &mut item.attrs,
        Item::Struct(item) => &mut item.attrs,
        Item::Trait(item) => &mut item.attrs,
        Item::TraitAlias(item) => &mut item.attrs,
        Item::Type(item) => &mut item.attrs,
        Item::Union(item) => &mut item.attrs,
        Item::Use(item) => &mut item.attrs,
        Item::Verbatim(_) => return,
        _ => return,
    };
    *attributes = world.project_attributes(attributes);
}

fn project_impl_item_attributes(item: &mut ImplItem, world: &ProductionWorld) {
    let attributes = match item {
        ImplItem::Const(item) => &mut item.attrs,
        ImplItem::Fn(item) => &mut item.attrs,
        ImplItem::Type(item) => &mut item.attrs,
        ImplItem::Macro(item) => &mut item.attrs,
        ImplItem::Verbatim(_) => return,
        _ => return,
    };
    *attributes = world.project_attributes(attributes);
}

fn project_trait_item_attributes(item: &mut TraitItem, world: &ProductionWorld) {
    let attributes = match item {
        TraitItem::Const(item) => &mut item.attrs,
        TraitItem::Fn(item) => &mut item.attrs,
        TraitItem::Type(item) => &mut item.attrs,
        TraitItem::Macro(item) => &mut item.attrs,
        TraitItem::Verbatim(_) => return,
        _ => return,
    };
    *attributes = world.project_attributes(attributes);
}

fn impl_item_attributes(item: &ImplItem) -> &[syn::Attribute] {
    match item {
        ImplItem::Const(item) => &item.attrs,
        ImplItem::Fn(item) => &item.attrs,
        ImplItem::Type(item) => &item.attrs,
        ImplItem::Macro(item) => &item.attrs,
        ImplItem::Verbatim(_) => &[],
        _ => &[],
    }
}

fn trait_item_attributes(item: &TraitItem) -> &[syn::Attribute] {
    match item {
        TraitItem::Const(item) => &item.attrs,
        TraitItem::Fn(item) => &item.attrs,
        TraitItem::Type(item) => &item.attrs,
        TraitItem::Macro(item) => &item.attrs,
        TraitItem::Verbatim(_) => &[],
        _ => &[],
    }
}
