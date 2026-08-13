//! Owned payload edges for caller-extractable nominal return wrappers.

use std::collections::BTreeSet;

use syn::{Field, Fields, ItemEnum, Visibility};

use super::authority_value_gate_defs::DefinitionKey;
use super::authority_value_gate_returns::return_type_keys;
use super::crate_modules::ModuleGraph;

pub(super) fn public_field_payloads(
    graph: &ModuleGraph,
    module_path: &[String],
    fields: &Fields,
) -> BTreeSet<DefinitionKey> {
    match fields {
        Fields::Named(fields) => public_named_payloads(graph, module_path, &fields.named),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .filter(|field| matches!(field.vis, Visibility::Public(_)))
            .flat_map(|field| payload_keys(graph, module_path, field))
            .collect(),
        Fields::Unit => BTreeSet::new(),
    }
}

pub(super) fn public_named_payloads(
    graph: &ModuleGraph,
    module_path: &[String],
    fields: &syn::punctuated::Punctuated<Field, syn::Token![,]>,
) -> BTreeSet<DefinitionKey> {
    fields
        .iter()
        .filter(|field| matches!(field.vis, Visibility::Public(_)))
        .flat_map(|field| payload_keys(graph, module_path, field))
        .collect()
}

pub(super) fn enum_payloads(
    graph: &ModuleGraph,
    module_path: &[String],
    item: &ItemEnum,
) -> BTreeSet<DefinitionKey> {
    item.variants
        .iter()
        .flat_map(|variant| variant.fields.iter())
        .flat_map(|field| payload_keys(graph, module_path, field))
        .collect()
}

fn payload_keys(
    graph: &ModuleGraph,
    module_path: &[String],
    field: &Field,
) -> BTreeSet<DefinitionKey> {
    return_type_keys(graph, module_path, &field.ty, None)
}
