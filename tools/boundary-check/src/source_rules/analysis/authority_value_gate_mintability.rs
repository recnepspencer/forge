//! Caller-mintability decision over indexed authority-marker producers.

use std::collections::{BTreeMap, BTreeSet};

use super::authority_value_gate_defs::{DefinitionKey, TypeDef, TypeDefKind};

pub(super) fn mintability_reason_for(
    marker_key: &DefinitionKey,
    def: &TypeDef,
    defaults: &BTreeSet<DefinitionKey>,
    constructors: &BTreeSet<DefinitionKey>,
    public_values: &BTreeSet<DefinitionKey>,
    trait_factories: &BTreeSet<DefinitionKey>,
    definitions: &BTreeMap<DefinitionKey, TypeDef>,
) -> Option<String> {
    if produced_as(marker_key, defaults, definitions) {
        return Some("public_default".to_owned());
    }
    if produced_as(marker_key, constructors, definitions) {
        return Some("public_constructor".to_owned());
    }
    if produced_as(marker_key, public_values, definitions) {
        return Some("public_const_or_static".to_owned());
    }
    if produced_as(marker_key, trait_factories, definitions) {
        return Some("public_trait_factory".to_owned());
    }

    if let TypeDefKind::Alias { targets } = &def.kind {
        if targets.is_empty() {
            return Some("unresolved_alias_target".to_owned());
        }
        return targets.iter().find_map(|target| {
            let underlying = definitions.get(target)?;
            underlying.is_public.then(|| {
                mintability_reason_for(
                    target,
                    underlying,
                    defaults,
                    constructors,
                    public_values,
                    trait_factories,
                    definitions,
                )
            })?
        });
    }

    match &def.kind {
        TypeDefKind::Struct { constructible, .. } if *constructible => {
            Some("public_unit_or_fields".to_owned())
        }
        TypeDefKind::Enum { constructible, .. } if *constructible => {
            Some("public_enum_variant".to_owned())
        }
        TypeDefKind::Union { constructible, .. } if *constructible => {
            Some("public_union_field".to_owned())
        }
        TypeDefKind::Alias { targets } if targets.is_empty() => {
            Some("opaque_type_alias".to_owned())
        }
        _ => None,
    }
}

fn produced_as(
    marker_key: &DefinitionKey,
    producers: &BTreeSet<DefinitionKey>,
    definitions: &BTreeMap<DefinitionKey, TypeDef>,
) -> bool {
    producers
        .iter()
        .any(|produced| reaches_marker(produced, marker_key, definitions, &mut BTreeSet::new()))
}

fn reaches_marker(
    current: &DefinitionKey,
    marker: &DefinitionKey,
    definitions: &BTreeMap<DefinitionKey, TypeDef>,
    visited: &mut BTreeSet<DefinitionKey>,
) -> bool {
    if current == marker {
        return true;
    }
    if !visited.insert(current.clone()) {
        return false;
    }
    match definitions.get(current).map(|definition| &definition.kind) {
        Some(TypeDefKind::Alias { targets }) => targets
            .iter()
            .any(|next| reaches_marker(next, marker, definitions, visited)),
        Some(TypeDefKind::Struct { owned_payloads, .. })
        | Some(TypeDefKind::Enum { owned_payloads, .. })
        | Some(TypeDefKind::Union { owned_payloads, .. }) => owned_payloads
            .iter()
            .any(|next| reaches_marker(next, marker, definitions, visited)),
        None => false,
    }
}
