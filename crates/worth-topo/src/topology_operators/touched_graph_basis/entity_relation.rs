use forge_relational::facade::identity::{EntityId, RelationId};
use serde::Serialize;

use super::BasisDigestPart;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct TopologyTouchedEntity {
    entity_id: EntityId,
}

impl TopologyTouchedEntity {
    pub(crate) const fn new(entity_id: EntityId) -> Self {
        Self { entity_id }
    }

    pub const fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl BasisDigestPart for TopologyTouchedEntity {
    fn digest_part(&self) -> String {
        format!(
            "entity:{}:{}:{}",
            self.entity_id.partition_id.0, self.entity_id.local_slot.0, self.entity_id.generation.0
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct TopologyTouchedRelation {
    relation_id: RelationId,
}

impl TopologyTouchedRelation {
    pub(crate) const fn new(relation_id: RelationId) -> Self {
        Self { relation_id }
    }

    pub const fn relation_id(&self) -> RelationId {
        self.relation_id
    }
}

impl BasisDigestPart for TopologyTouchedRelation {
    fn digest_part(&self) -> String {
        format!(
            "relation:{}:{}:{}",
            self.relation_id.partition_id.0,
            self.relation_id.local_slot.0,
            self.relation_id.generation.0
        )
    }
}
