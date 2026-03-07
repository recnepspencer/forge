use serde::{Deserialize, Serialize};

use crate::data::identity::{SpecNodeId, SpecRelationId};
use crate::data::schema::RelationKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationRecord {
    pub id: SpecRelationId,
    pub kind: RelationKind,
    pub source: SpecNodeId,
    pub target: SpecNodeId,
    pub ordinal: u32,
}
