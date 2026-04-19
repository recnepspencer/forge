use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;

use crate::data::topology_view::WorthTopologyView;
use crate::validators::error::WorthTopologyValidationError;

pub fn err(validator: &'static str, message: impl Into<String>) -> WorthTopologyValidationError {
    WorthTopologyValidationError::new(validator, message)
}

pub fn unique_ids<'a, T, F>(
    records: &'a [T],
    entity_id: F,
) -> Result<BTreeSet<EntityId>, WorthTopologyValidationError>
where
    F: Fn(&'a T) -> EntityId,
{
    let mut ids = BTreeSet::new();
    for record in records {
        let id = entity_id(record);
        if !ids.insert(id) {
            return Err(err(
                "duplicate_entity_id",
                format!("duplicate entity {:?}", id),
            ));
        }
    }
    Ok(ids)
}

pub fn face_outer_loop_map(view: &WorthTopologyView) -> BTreeMap<EntityId, EntityId> {
    view.faces
        .iter()
        .filter_map(|face| face.outer_loop_id.map(|loop_id| (face.entity_id, loop_id)))
        .collect()
}

pub fn loop_face_map(view: &WorthTopologyView) -> BTreeMap<EntityId, EntityId> {
    let mut map = BTreeMap::new();
    for loop_record in &view.loops {
        for face_id in &loop_record.face_ids {
            map.insert(loop_record.entity_id, *face_id);
        }
    }
    map
}
