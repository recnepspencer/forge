use std::collections::BTreeSet;

use crate::symbols::data::{InternedString, StringInterner, SymbolPolicy};

use super::{CreateIntent, EntityMutationIntent, MutationIntent, RelationMutationIntent};
use crate::transactions::data::EntityReference;

impl MutationIntent {
    pub(crate) fn collect_raw_client_keys(&self, raw_values: &mut BTreeSet<String>) {
        match self {
            Self::Create(CreateIntent::Entity(spec)) => {
                if let InternedString::Raw(raw) = &spec.client_key {
                    raw_values.insert(raw.clone());
                }
            }
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                collect_bulk_raw_client_keys(&spec.client_keys, raw_values);
            }
            Self::Create(CreateIntent::BulkRelations(spec)) => {
                collect_bulk_raw_client_keys(&spec.client_keys, raw_values);
                for (source, target) in &spec.endpoints {
                    collect_entity_reference_raw_client_key(source, raw_values);
                    collect_entity_reference_raw_client_key(target, raw_values);
                }
            }
            Self::Create(CreateIntent::Relation(spec)) => {
                if let InternedString::Raw(raw) = &spec.client_key {
                    raw_values.insert(raw.clone());
                }
                collect_entity_reference_raw_client_key(&spec.source, raw_values);
                collect_entity_reference_raw_client_key(&spec.target, raw_values);
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                if let InternedString::Raw(raw) = &spec.replacement.client_key {
                    raw_values.insert(raw.clone());
                }
            }
            Self::Relation(RelationMutationIntent::UpdateEndpoints(spec)) => {
                collect_entity_reference_raw_client_key(&spec.source, raw_values);
                collect_entity_reference_raw_client_key(&spec.target, raw_values);
            }
            Self::Entity(_) | Self::Relation(RelationMutationIntent::Delete(_)) => {}
        }
    }

    pub(crate) fn normalize_client_keys(
        &mut self,
        interner: &mut StringInterner,
        policy: SymbolPolicy,
    ) {
        match self {
            Self::Create(CreateIntent::Entity(spec)) => {
                spec.client_key =
                    normalize_interned_string(interner, policy, spec.client_key.clone());
            }
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                normalize_bulk_client_keys(&mut spec.client_keys, interner, policy);
            }
            Self::Create(CreateIntent::BulkRelations(spec)) => {
                normalize_bulk_client_keys(&mut spec.client_keys, interner, policy);
                for (source, target) in &mut spec.endpoints {
                    normalize_entity_reference_client_key(source, interner, policy);
                    normalize_entity_reference_client_key(target, interner, policy);
                }
            }
            Self::Create(CreateIntent::Relation(spec)) => {
                spec.client_key =
                    normalize_interned_string(interner, policy, spec.client_key.clone());
                normalize_entity_reference_client_key(&mut spec.source, interner, policy);
                normalize_entity_reference_client_key(&mut spec.target, interner, policy);
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                spec.replacement.client_key = normalize_interned_string(
                    interner,
                    policy,
                    spec.replacement.client_key.clone(),
                );
            }
            Self::Relation(RelationMutationIntent::UpdateEndpoints(spec)) => {
                normalize_entity_reference_client_key(&mut spec.source, interner, policy);
                normalize_entity_reference_client_key(&mut spec.target, interner, policy);
            }
            Self::Entity(_) | Self::Relation(RelationMutationIntent::Delete(_)) => {}
        }
    }
}

fn collect_bulk_raw_client_keys(client_keys: &[InternedString], raw_values: &mut BTreeSet<String>) {
    for client_key in client_keys {
        if let InternedString::Raw(raw) = client_key {
            raw_values.insert(raw.clone());
        }
    }
}

fn normalize_bulk_client_keys(
    client_keys: &mut [InternedString],
    interner: &mut StringInterner,
    policy: SymbolPolicy,
) {
    for client_key in client_keys {
        *client_key = normalize_interned_string(interner, policy, client_key.clone());
    }
}

fn collect_entity_reference_raw_client_key(
    entity_reference: &EntityReference,
    raw_values: &mut BTreeSet<String>,
) {
    if let EntityReference::Created(created) = entity_reference {
        if let InternedString::Raw(raw) = &created.client_key {
            raw_values.insert(raw.clone());
        }
    }
}

fn normalize_entity_reference_client_key(
    entity_reference: &mut EntityReference,
    interner: &mut StringInterner,
    policy: SymbolPolicy,
) {
    if let EntityReference::Created(created) = entity_reference {
        created.client_key =
            normalize_interned_string(interner, policy, created.client_key.clone());
    }
}

fn normalize_interned_string(
    interner: &mut StringInterner,
    policy: SymbolPolicy,
    value: InternedString,
) -> InternedString {
    match policy {
        SymbolPolicy::Disabled => value,
        SymbolPolicy::PreferInterned | SymbolPolicy::RequireInterned => interner.normalize(value),
    }
}
