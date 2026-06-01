use super::*;

pub(super) fn bulk_relation_create_intents(
    relation_specs: &[crate::transactions::data::RelationSpec],
) -> Vec<MutationIntent> {
    let mut by_partition: BTreeMap<
        (PartitionId, KindId),
        (
            Vec<crate::symbols::data::ClientKey>,
            Vec<(
                crate::transactions::data::EntityReference,
                crate::transactions::data::EntityReference,
            )>,
            Vec<crate::transactions::data::AspectFieldPatch>,
        ),
    > = BTreeMap::new();

    for relation in relation_specs {
        let entry = by_partition
            .entry((relation.partition_id, relation.kind_id))
            .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new()));
        entry.0.push(relation.client_key.clone());
        entry
            .1
            .push((relation.source.clone(), relation.target.clone()));
        entry.2.push(relation.fields.clone());
    }

    by_partition
        .into_iter()
        .map(
            |((partition_id, kind_id), (client_keys, endpoints, field_patches))| {
                MutationIntent::Create(CreateIntent::BulkRelations(
                    crate::transactions::data::BulkRelationCreateIntent {
                        partition_id,
                        kind_id,
                        client_keys,
                        endpoints,
                        field_patches,
                    },
                ))
            },
        )
        .collect()
}

pub(super) fn bulk_entity_create_intents(
    entity_specs: &[crate::transactions::data::EntitySpec],
) -> Vec<MutationIntent> {
    let mut by_partition: BTreeMap<
        (PartitionId, KindId),
        (
            Vec<crate::symbols::data::ClientKey>,
            Vec<crate::transactions::data::AspectFieldPatch>,
        ),
    > = BTreeMap::new();

    for entity in entity_specs {
        let entry = by_partition
            .entry((entity.partition_id, entity.kind_id))
            .or_insert_with(|| (Vec::new(), Vec::new()));
        entry.0.push(entity.client_key.clone());
        entry.1.push(entity.fields.clone());
    }

    by_partition
        .into_iter()
        .map(|((partition_id, kind_id), (client_keys, field_patches))| {
            MutationIntent::Create(CreateIntent::BulkEntities(
                crate::transactions::data::BulkEntityCreateIntent {
                    partition_id,
                    kind_id,
                    client_keys,
                    field_patches,
                },
            ))
        })
        .collect()
}
