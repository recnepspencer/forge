use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalReadView;
use schema::facade::{EntityKind, NamingRelationKind, RelationKind};

use crate::validation::error::TopologyValidationError;
use crate::validation::shared::err;

pub fn validate_named_topology_truth(
    read_view: &RelationalReadView,
) -> Result<(), TopologyValidationError> {
    let entity_kind_map: BTreeMap<EntityId, EntityKind> = read_view
        .entities()
        .iter()
        .filter_map(|record| {
            EntityKind::from_kind_id(record.kind.kind_id).map(|kind| (record.entity_id, kind))
        })
        .collect();

    let topology_entity_ids: BTreeSet<_> = entity_kind_map
        .iter()
        .filter_map(|(entity_id, kind)| {
            matches!(kind, EntityKind::Topology(_)).then_some(*entity_id)
        })
        .collect();

    let mut targeted_by_name: BTreeMap<EntityId, BTreeSet<EntityId>> = BTreeMap::new();

    for relation in read_view.relations() {
        let Some(kind) = RelationKind::from_kind_id(relation.kind.kind_id) else {
            continue;
        };
        if kind != RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity) {
            continue;
        }

        let Some(source_kind) = entity_kind_map.get(&relation.source).copied() else {
            return Err(err(
                "naming.persistent_name_coverage",
                format!(
                    "persistent-name relation {:?} references missing source entity {:?}",
                    relation.relation_id, relation.source
                ),
            ));
        };
        let Some(target_kind) = entity_kind_map.get(&relation.target).copied() else {
            return Err(err(
                "naming.persistent_name_coverage",
                format!(
                    "persistent-name relation {:?} references missing target entity {:?}",
                    relation.relation_id, relation.target
                ),
            ));
        };

        if !matches!(source_kind, EntityKind::Naming(_)) {
            return Err(err(
                "naming.persistent_name_coverage",
                format!(
                    "persistent-name relation {:?} uses non-naming source {:?}",
                    relation.relation_id, relation.source
                ),
            ));
        }
        if !matches!(target_kind, EntityKind::Topology(_)) {
            continue;
        }

        targeted_by_name
            .entry(relation.target)
            .or_default()
            .insert(relation.source);
    }

    for entity_id in topology_entity_ids {
        match targeted_by_name
            .get(&entity_id)
            .map(BTreeSet::len)
            .unwrap_or(0)
        {
            1 => {}
            0 => {
                return Err(err(
                    "naming.persistent_name_coverage",
                    format!(
                        "topology entity {:?} has no persistent name target",
                        entity_id
                    ),
                ));
            }
            count => {
                return Err(err(
                    "naming.persistent_name_coverage",
                    format!(
                        "topology entity {:?} has {} persistent name targets (expected 1)",
                        entity_id, count
                    ),
                ));
            }
        }
    }

    Ok(())
}
