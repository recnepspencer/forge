use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue,
};
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::authority::EntityReference;

pub(super) fn canonical_entity_reference_entry(
    locus: impl Into<String>,
    reference: &EntityReference,
) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::new(
        locus,
        ForgeQueryDeclarationCanonicalEntryKind::Identity,
        ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_reference_value(reference)),
    )
}

fn canonical_entity_reference_value(reference: &EntityReference) -> String {
    match reference {
        EntityReference::Existing(entity_id) => format!(
            "existing:{}:{}:{}",
            entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
        ),
        EntityReference::Created(create_key) => format!("created:{}", create_key.as_str()),
    }
}

pub(super) fn canonical_entity_id(entity_id: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
    )
}

pub(super) fn canonical_relation_id(relation_id: RelationId) -> String {
    format!(
        "relation:{}:{}:{}",
        relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
    )
}
