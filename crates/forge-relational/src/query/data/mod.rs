use serde::{Deserialize, Serialize};

use crate::identity::data::PartitionId;
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionHint {
    pub partition_id: PartitionId,
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
pub struct QueryWorkPacket {
    pub label: String,
    pub partition_hint: Option<PartitionHint>,
    pub execution_shape: QueryExecutionShape,
    pub reduction: ReductionDiscipline,
    pub targets: Vec<RecordRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadPacketPlan {
    pub label: String,
    pub entity_chunk_indexes: Vec<usize>,
    pub relation_chunk_indexes: Vec<usize>,
    pub target_count: usize,
}

impl QueryWorkPacket {
    pub fn bulk(label: impl Into<String>, targets: Vec<RecordRef>) -> Self {
        Self {
            label: label.into(),
            partition_hint: None,
            execution_shape: QueryExecutionShape::BulkPacketized,
            reduction: ReductionDiscipline::DeterministicMerge,
            targets,
        }
    }
}
