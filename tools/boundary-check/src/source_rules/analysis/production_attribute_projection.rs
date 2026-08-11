//! Normalize cfg and cfg_attr attributes for one compiler-selected world.

use super::production_availability::{parse_predicates, world_possibilities};
use std::collections::{BTreeMap, BTreeSet};
use syn::{Attribute, Meta};

pub(super) fn projected_attributes_in_world(
    attributes: &[Attribute],
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> Vec<Attribute> {
    attributes
        .iter()
        .flat_map(|attribute| project_attribute(attribute, enabled_features, cfg_atoms))
        .collect()
}

fn project_attribute(
    attribute: &Attribute,
    enabled_features: &BTreeSet<String>,
    cfg_atoms: &BTreeMap<String, bool>,
) -> Vec<Attribute> {
    let Meta::List(list) = &attribute.meta else {
        return vec![attribute.clone()];
    };
    if list.path.is_ident("cfg") {
        return Vec::new();
    }
    if !list.path.is_ident("cfg_attr") {
        return vec![attribute.clone()];
    }
    let Some(arguments) = parse_predicates(list) else {
        return vec![attribute.clone()];
    };
    let mut arguments = arguments.iter();
    let Some(condition) = arguments.next() else {
        return vec![attribute.clone()];
    };
    let condition = world_possibilities(condition, enabled_features, cfg_atoms);
    if condition.can_be_true && condition.can_be_false {
        return vec![attribute.clone()];
    }
    if !condition.can_be_true {
        return Vec::new();
    }
    arguments
        .flat_map(|nested| {
            let nested: Attribute = syn::parse_quote!(#[#nested]);
            project_attribute(&nested, enabled_features, cfg_atoms)
        })
        .collect()
}
