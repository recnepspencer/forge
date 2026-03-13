use crate::identity::data::PartitionId;
use crate::indexes::data::DerivedIndexId;
use crate::transactions::data::RecordRef;
use crate::validation::data::{InvariantExecutionPoint, InvariantGroupSet};
use crate::validation::engine::InvariantObservationKind;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ValidationReductionKey {
    pub(crate) execution_point: InvariantExecutionPoint,
    pub(crate) observation_kind: InvariantObservationKind,
    pub(crate) partition_scope: Vec<PartitionId>,
    pub(crate) invariant_group_scope_mask: u32,
    pub(crate) packet_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DiffReductionKey {
    pub(crate) target: RecordRef,
    pub(crate) kind_order: u8,
    pub(crate) packet_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct IndexReductionKey {
    pub(crate) index_id: DerivedIndexId,
    pub(crate) packet_index: usize,
}

impl DiffReductionKey {
    pub(crate) fn new(target: RecordRef, kind_order: u8, packet_index: usize) -> Self {
        Self {
            target,
            kind_order,
            packet_index,
        }
    }
}

impl IndexReductionKey {
    pub(crate) fn new(index_id: DerivedIndexId, packet_index: usize) -> Self {
        Self {
            index_id,
            packet_index,
        }
    }
}

impl ValidationReductionKey {
    pub(crate) fn new(
        execution_point: InvariantExecutionPoint,
        observation_kind: InvariantObservationKind,
        partition_scope: Vec<PartitionId>,
        invariant_group_scope: InvariantGroupSet,
        packet_index: usize,
    ) -> Self {
        Self {
            execution_point,
            observation_kind,
            partition_scope,
            invariant_group_scope_mask: invariant_group_scope.mask(),
            packet_index,
        }
    }
}
