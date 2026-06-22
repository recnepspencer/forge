#![allow(dead_code)]

use forge_query::facade::{ForgeQueryEntityIdentity, RelationalBridgeRecordIdentityParts};

pub fn relational_test_entity_identity(label: &str) -> ForgeQueryEntityIdentity {
    parse_typed_relational_record_parts(label)
        .or_else(|| parse_collection_slot_relational_record_parts(label))
        .or_else(|| relational_test_entity_identity_named_fixture(label))
        .map(ForgeQueryEntityIdentity::from_relational_record)
        .unwrap_or_else(|| {
            ForgeQueryEntityIdentity::admit_authored_entity_token(
                forge_query::facade::QueryExternalIdentityToken::new(std::sync::Arc::from(label)),
            )
        })
}

fn parse_typed_relational_record_parts(label: &str) -> Option<RelationalBridgeRecordIdentityParts> {
    let mut segments = label.split(':');
    let kind = segments.next()?;
    let partition_id = segments.next()?.parse().ok()?;
    let local_slot = segments.next()?.parse().ok()?;
    let generation = segments.next()?.parse().ok()?;
    if segments.next().is_some() {
        return None;
    }
    match kind {
        "entity" => Some(RelationalBridgeRecordIdentityParts::entity(
            partition_id,
            local_slot,
            generation,
        )),
        "relation" => Some(RelationalBridgeRecordIdentityParts::relation(
            partition_id,
            local_slot,
            generation,
        )),
        _ => None,
    }
}

fn parse_collection_slot_relational_record_parts(
    label: &str,
) -> Option<RelationalBridgeRecordIdentityParts> {
    let (collection, slot_text) = label.rsplit_once(':')?;
    let local_slot = slot_text.parse().ok()?;
    if relational_test_entity_identity_is_relation_collection(collection) {
        Some(RelationalBridgeRecordIdentityParts::relation(
            2, local_slot, 0,
        ))
    } else {
        Some(RelationalBridgeRecordIdentityParts::entity(
            1, local_slot, 0,
        ))
    }
}

fn relational_test_entity_identity_is_relation_collection(collection: &str) -> bool {
    collection.ends_with("Relation") || matches!(collection, "Edge" | "TaskEdge")
}

fn relational_test_entity_identity_named_fixture(
    label: &str,
) -> Option<RelationalBridgeRecordIdentityParts> {
    match label {
        "task-existing" | "vertex-a" | "face-1" => {
            Some(RelationalBridgeRecordIdentityParts::entity(1, 1, 0))
        }
        "task-existing-left" | "vertex-b" => {
            Some(RelationalBridgeRecordIdentityParts::entity(1, 2, 0))
        }
        "task-existing-right" => Some(RelationalBridgeRecordIdentityParts::entity(1, 3, 0)),
        _ => None,
    }
}
