use serde::{Deserialize, Serialize};

use crate::data::identity::{EntityId, RelationId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionHint {
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryExecutionShape {
    SingleEntity,
    BulkPacketized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReductionDiscipline {
    DeterministicMerge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadTarget {
    Entity(EntityId),
    Relation(RelationId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryWorkPacket {
    pub label: String,
    pub partition_hint: Option<PartitionHint>,
    pub execution_shape: QueryExecutionShape,
    pub reduction: ReductionDiscipline,
    pub targets: Vec<ReadTarget>,
}

impl QueryWorkPacket {
    pub fn bulk(label: impl Into<String>, targets: Vec<ReadTarget>) -> Self {
        Self {
            label: label.into(),
            partition_hint: None,
            execution_shape: QueryExecutionShape::BulkPacketized,
            reduction: ReductionDiscipline::DeterministicMerge,
            targets,
        }
    }
}
