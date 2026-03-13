use std::collections::BTreeSet;

use crate::symbols::data::{InternedString, StringInterner, SymbolPolicy};

use super::{CreateIntent, MutationIntent};

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
            }
            Self::Create(CreateIntent::Relation(spec)) => {
                if let InternedString::Raw(raw) = &spec.client_key {
                    raw_values.insert(raw.clone());
                }
            }
            Self::Entity(_) | Self::Relation(_) => {}
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
            }
            Self::Create(CreateIntent::Relation(spec)) => {
                spec.client_key =
                    normalize_interned_string(interner, policy, spec.client_key.clone());
            }
            Self::Entity(_) | Self::Relation(_) => {}
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
