use forge_relational::facade::identity::EntityId;
use schema::facade::{EntityKind, RelationKind};

use super::errors::TopologyMaterializationError;
use crate::derived_topology::materialized_graph::traits::HasEntityId;

pub(super) fn ensure_relation_types(
    relation_kind: RelationKind,
    actual_source: EntityKind,
    expected_source: EntityKind,
    actual_target: EntityKind,
    expected_target: EntityKind,
) -> Result<(), TopologyMaterializationError> {
    if actual_source != expected_source || actual_target != expected_target {
        return Err(TopologyMaterializationError::new(format!(
            " relation `{}` expected {} -> {} but saw {} -> {}",
            relation_kind.kind_name(),
            expected_source.kind_name(),
            expected_target.kind_name(),
            actual_source.kind_name(),
            actual_target.kind_name(),
        )));
    }

    Ok(())
}

pub(super) fn push_child_to_parent<T, F>(
    records: &mut [T],
    entity_id: EntityId,
    child_id: EntityId,
    children: F,
) -> Result<(), TopologyMaterializationError>
where
    F: Fn(&mut T) -> &mut Vec<EntityId>,
    T: HasEntityId,
{
    let record = find_record_mut(records, entity_id)?;
    let targets = children(record);
    if !targets.contains(&child_id) {
        targets.push(child_id);
    }
    Ok(())
}

pub(super) fn set_optional_parent<T, F>(
    records: &mut [T],
    entity_id: EntityId,
    parent_id: EntityId,
    field: F,
) -> Result<(), TopologyMaterializationError>
where
    F: Fn(&mut T) -> &mut Option<EntityId>,
    T: HasEntityId,
{
    let record = find_record_mut(records, entity_id)?;
    *field(record) = Some(parent_id);
    Ok(())
}

pub(super) fn find_record_mut<T>(
    records: &mut [T],
    entity_id: EntityId,
) -> Result<&mut T, TopologyMaterializationError>
where
    T: HasEntityId,
{
    records
        .iter_mut()
        .find(|record| record.entity_id() == entity_id)
        .ok_or_else(|| {
            TopologyMaterializationError::new(format!(
                " topology materialization could not find entity {:?} while wiring structure",
                entity_id
            ))
        })
}
