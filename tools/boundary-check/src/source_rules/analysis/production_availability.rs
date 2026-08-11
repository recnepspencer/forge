//! Identify declarations that cannot exist in an ordinary production build.

use super::crate_modules::ModuleGraph;
use quote::ToTokens;
use std::collections::{BTreeMap, BTreeSet};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Attribute, Item, Meta, Token};

pub(super) fn item_is_production_available(item: &Item) -> bool {
    attributes_are_production_available(item_attributes(item))
}

pub(super) fn attributes_are_production_available(attributes: &[Attribute]) -> bool {
    attributes.iter().all(attribute_allows_production)
        && !attributes_have_contradictory_cfg(attributes)
}

fn attributes_have_contradictory_cfg(attributes: &[Attribute]) -> bool {
    let mut positive = BTreeSet::new();
    let mut negative = BTreeSet::new();
    for attribute in attributes {
        let Meta::List(list) = &attribute.meta else {
            continue;
        };
        if !list.path.is_ident("cfg") {
            continue;
        }
        if let Some(predicates) = parse_predicates(list) {
            for predicate in &predicates {
                collect_required_literals(predicate, false, &mut positive, &mut negative);
            }
        }
    }
    positive.iter().any(|atom| negative.contains(atom))
}

fn collect_required_literals(
    predicate: &Meta,
    negated: bool,
    positive: &mut BTreeSet<String>,
    negative: &mut BTreeSet<String>,
) {
    match predicate {
        Meta::List(list) if list.path.is_ident("all") => {
            if let Some(predicates) = parse_predicates(list) {
                for predicate in &predicates {
                    collect_required_literals(predicate, negated, positive, negative);
                }
            }
        }
        Meta::List(list) if list.path.is_ident("not") => {
            if let Some(predicates) = parse_predicates(list) {
                if predicates.len() == 1 {
                    collect_required_literals(&predicates[0], !negated, positive, negative);
                }
            }
        }
        Meta::List(list) if list.path.is_ident("any") => {}
        atom => {
            let key = atom.to_token_stream().to_string();
            if negated {
                negative.insert(key);
            } else {
                positive.insert(key);
            }
        }
    }
}

pub(super) fn module_is_production_available(graph: &ModuleGraph, path: &[String]) -> bool {
    for depth in 1..=path.len() {
        let parent = &path[..depth - 1];
        let child_name = &path[depth - 1];
        let Some(parent_node) = graph.modules.get(parent) else {
            return false;
        };
        let declarations: Vec<_> = parent_node
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(item_mod) if item_mod.ident == child_name => Some(item_mod),
                _ => None,
            })
            .collect();
        if !declarations.is_empty()
            && declarations
                .iter()
                .all(|item_mod| !attributes_are_production_available(&item_mod.attrs))
        {
            return false;
        }
    }
    true
}

fn attribute_allows_production(attribute: &Attribute) -> bool {
    meta_allows_production(&attribute.meta)
}

fn meta_allows_production(meta: &Meta) -> bool {
    match meta {
        Meta::List(list) if list.path.is_ident("cfg") => parse_predicates(list)
            .map(|predicates| {
                predicates
                    .iter()
                    .all(|predicate| predicate_possibilities(predicate).can_be_true)
            })
            .unwrap_or(true),
        Meta::List(list) if list.path.is_ident("cfg_attr") => cfg_attr_allows_production(list),
        _ => true,
    }
}

fn cfg_attr_allows_production(list: &syn::MetaList) -> bool {
    let Some(arguments) = parse_predicates(list) else {
        return true;
    };
    let mut arguments = arguments.iter();
    let Some(condition) = arguments.next() else {
        return true;
    };
    let possibilities = predicate_possibilities(condition);
    if possibilities.can_be_false {
        return true;
    }
    arguments.all(meta_allows_production)
}

#[derive(Clone, Copy)]
pub(super) struct PredicatePossibilities {
    pub(super) can_be_true: bool,
    pub(super) can_be_false: bool,
}

fn predicate_possibilities(predicate: &Meta) -> PredicatePossibilities {
    match predicate {
        Meta::Path(path) if path.is_ident("test") || path.is_ident("doctest") => {
            PredicatePossibilities {
                can_be_true: false,
                can_be_false: true,
            }
        }
        Meta::List(list) if list.path.is_ident("all") => parse_predicates(list)
            .map(|predicates| combine_all(&predicates))
            .unwrap_or_else(unknown_possibilities),
        Meta::List(list) if list.path.is_ident("any") => parse_predicates(list)
            .map(|predicates| combine_any(&predicates))
            .unwrap_or_else(unknown_possibilities),
        Meta::List(list) if list.path.is_ident("not") => parse_predicates(list)
            .filter(|predicates| predicates.len() == 1)
            .map(|predicates| {
                let inner = predicate_possibilities(&predicates[0]);
                PredicatePossibilities {
                    can_be_true: inner.can_be_false,
                    can_be_false: inner.can_be_true,
                }
            })
            .unwrap_or_else(unknown_possibilities),
        _ => unknown_possibilities(),
    }
}

pub(super) fn attributes_are_available_in_world(
    attributes: &[Attribute],
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> bool {
    attributes
        .iter()
        .all(|attribute| meta_is_available_in_world(&attribute.meta, enabled_features, cfg_atoms))
}

fn meta_is_available_in_world(
    meta: &Meta,
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> bool {
    match meta {
        Meta::List(list) if list.path.is_ident("cfg") => parse_predicates(list)
            .map(|predicates| {
                predicates.iter().all(|predicate| {
                    world_possibilities(predicate, enabled_features, cfg_atoms).can_be_true
                })
            })
            .unwrap_or(true),
        Meta::List(list) if list.path.is_ident("cfg_attr") => {
            cfg_attr_is_available_in_world(list, enabled_features, cfg_atoms)
        }
        _ => true,
    }
}

fn cfg_attr_is_available_in_world(
    list: &syn::MetaList,
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> bool {
    let Some(arguments) = parse_predicates(list) else {
        return true;
    };
    let mut arguments = arguments.iter();
    let Some(condition) = arguments.next() else {
        return true;
    };
    let condition = world_possibilities(condition, enabled_features, cfg_atoms);
    condition.can_be_false
        || arguments.all(|nested| meta_is_available_in_world(nested, enabled_features, cfg_atoms))
}

pub(super) fn world_possibilities(
    predicate: &Meta,
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> PredicatePossibilities {
    if let Meta::NameValue(value) = predicate {
        if value.path.is_ident("feature") {
            let syn::Expr::Lit(value) = &value.value else {
                return unknown_possibilities();
            };
            let syn::Lit::Str(feature) = &value.lit else {
                return unknown_possibilities();
            };
            let enabled = enabled_features.contains(&feature.value());
            return PredicatePossibilities {
                can_be_true: enabled,
                can_be_false: !enabled,
            };
        }
    }
    match predicate {
        Meta::Path(path) if path.is_ident("test") || path.is_ident("doctest") => {
            PredicatePossibilities {
                can_be_true: false,
                can_be_false: true,
            }
        }
        Meta::List(list) if list.path.is_ident("all") => parse_predicates(list)
            .map(|predicates| combine_all_in_world(&predicates, enabled_features, cfg_atoms))
            .unwrap_or_else(unknown_possibilities),
        Meta::List(list) if list.path.is_ident("any") => parse_predicates(list)
            .map(|predicates| combine_any_in_world(&predicates, enabled_features, cfg_atoms))
            .unwrap_or_else(unknown_possibilities),
        Meta::List(list) if list.path.is_ident("not") => parse_predicates(list)
            .filter(|predicates| predicates.len() == 1)
            .map(|predicates| {
                let inner = world_possibilities(&predicates[0], enabled_features, cfg_atoms);
                PredicatePossibilities {
                    can_be_true: inner.can_be_false,
                    can_be_false: inner.can_be_true,
                }
            })
            .unwrap_or_else(unknown_possibilities),
        _ => {
            let enabled = cfg_atoms
                .get(&cfg_atom_key(predicate))
                .copied()
                .unwrap_or(false);
            PredicatePossibilities {
                can_be_true: enabled,
                can_be_false: !enabled,
            }
        }
    }
}

fn combine_all_in_world(
    predicates: &Punctuated<Meta, Token![,]>,
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> PredicatePossibilities {
    PredicatePossibilities {
        can_be_true: predicates.iter().all(|predicate| {
            world_possibilities(predicate, enabled_features, cfg_atoms).can_be_true
        }),
        can_be_false: predicates.iter().any(|predicate| {
            world_possibilities(predicate, enabled_features, cfg_atoms).can_be_false
        }),
    }
}

fn combine_any_in_world(
    predicates: &Punctuated<Meta, Token![,]>,
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> PredicatePossibilities {
    PredicatePossibilities {
        can_be_true: predicates.iter().any(|predicate| {
            world_possibilities(predicate, enabled_features, cfg_atoms).can_be_true
        }),
        can_be_false: predicates.iter().all(|predicate| {
            world_possibilities(predicate, enabled_features, cfg_atoms).can_be_false
        }),
    }
}

pub(super) fn cfg_atom_key(meta: &Meta) -> String {
    meta.to_token_stream().to_string()
}

fn combine_all(predicates: &Punctuated<Meta, Token![,]>) -> PredicatePossibilities {
    PredicatePossibilities {
        can_be_true: predicates
            .iter()
            .all(|predicate| predicate_possibilities(predicate).can_be_true),
        can_be_false: predicates
            .iter()
            .any(|predicate| predicate_possibilities(predicate).can_be_false),
    }
}

fn combine_any(predicates: &Punctuated<Meta, Token![,]>) -> PredicatePossibilities {
    PredicatePossibilities {
        can_be_true: predicates
            .iter()
            .any(|predicate| predicate_possibilities(predicate).can_be_true),
        can_be_false: predicates
            .iter()
            .all(|predicate| predicate_possibilities(predicate).can_be_false),
    }
}

fn unknown_possibilities() -> PredicatePossibilities {
    PredicatePossibilities {
        can_be_true: true,
        can_be_false: true,
    }
}

pub(super) fn parse_predicates(list: &syn::MetaList) -> Option<Punctuated<Meta, Token![,]>> {
    Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(list.tokens.clone())
        .ok()
}

pub(super) fn item_attributes(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::ExternCrate(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::ForeignMod(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Static(item) => &item.attrs,
        Item::Struct(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::TraitAlias(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Use(item) => &item.attrs,
        Item::Verbatim(_) => &[],
        _ => &[],
    }
}
