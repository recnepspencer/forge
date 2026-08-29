use std::collections::BTreeSet;

use super::truth_record_access::sort_authoritative_relation_records;
use super::*;

#[derive(Debug)]
pub struct BoundedFrontierAdjacencyTruthRead {
    records: Vec<RelationReadRecord>,
    adjacency_lists_read: usize,
    relation_records_examined: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrontierAdjacencyTruthReadLimitExceeded {
    adjacency_lists_read: usize,
    relation_records_examined: usize,
    endpoint_records_reserved: usize,
}

impl<'runtime> VisibilityReadContext<'runtime> {
    pub fn bounded_outgoing_relations_for_frontier_at_version(
        &self,
        entity_ids: &BTreeSet<crate::identity::data::EntityId>,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
        maximum_work_units: usize,
    ) -> Result<BoundedFrontierAdjacencyTruthRead, FrontierAdjacencyTruthReadLimitExceeded> {
        self.bounded_relations_for_frontier_at_version(
            entity_ids,
            kind_id,
            version_id,
            FrontierAdjacencyDirection::Outgoing,
            maximum_work_units,
        )
    }

    pub fn bounded_incoming_relations_for_frontier_at_version(
        &self,
        entity_ids: &BTreeSet<crate::identity::data::EntityId>,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
        maximum_work_units: usize,
    ) -> Result<BoundedFrontierAdjacencyTruthRead, FrontierAdjacencyTruthReadLimitExceeded> {
        self.bounded_relations_for_frontier_at_version(
            entity_ids,
            kind_id,
            version_id,
            FrontierAdjacencyDirection::Incoming,
            maximum_work_units,
        )
    }

    fn bounded_relations_for_frontier_at_version(
        &self,
        entity_ids: &BTreeSet<crate::identity::data::EntityId>,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
        direction: FrontierAdjacencyDirection,
        maximum_work_units: usize,
    ) -> Result<BoundedFrontierAdjacencyTruthRead, FrontierAdjacencyTruthReadLimitExceeded> {
        let current_version = version_id == self.runtime.current_version_id();
        let mut records = Vec::new();
        let mut adjacency_lists_read = 0_usize;
        let mut relation_records_examined = 0_usize;
        for entity_id in entity_ids {
            charge_frontier_work(
                maximum_work_units,
                adjacency_lists_read,
                relation_records_examined,
                records.len(),
            )?;
            adjacency_lists_read += 1;
            let relation_ids = self
                .runtime
                .partitions
                .partition(entity_id.partition_id)
                .and_then(|partition| {
                    let adjacency = match direction {
                        FrontierAdjacencyDirection::Outgoing => {
                            partition.adjacency.get(entity_id.slot_index())
                        }
                        FrontierAdjacencyDirection::Incoming => {
                            partition.reverse_adjacency.get(entity_id.slot_index())
                        }
                    }?;
                    Some(if current_version {
                        adjacency.current_kind_slice(kind_id).to_vec()
                    } else {
                        adjacency.historical_kind_slice(kind_id).to_vec()
                    })
                })
                .unwrap_or_default();
            for relation_id in relation_ids.iter().copied() {
                charge_frontier_work(
                    maximum_work_units,
                    adjacency_lists_read,
                    relation_records_examined,
                    records.len(),
                )?;
                relation_records_examined += 1;
                let Some(record) =
                    self.authoritative_relation_record_at_version(relation_id, version_id)
                else {
                    continue;
                };
                if record.kind.kind_id != kind_id
                    || record.lifecycle != crate::storage::data::RecordLifecycleState::Live
                    || !direction.matches_endpoint(&record, *entity_id)
                {
                    continue;
                }
                charge_frontier_work(
                    maximum_work_units,
                    adjacency_lists_read,
                    relation_records_examined,
                    records.len(),
                )?;
                records.push(record);
            }
        }
        sort_authoritative_relation_records(&mut records);
        Ok(BoundedFrontierAdjacencyTruthRead {
            records,
            adjacency_lists_read,
            relation_records_examined,
        })
    }
}

impl BoundedFrontierAdjacencyTruthRead {
    pub const fn adjacency_lists_read(&self) -> usize {
        self.adjacency_lists_read
    }

    pub const fn relation_records_examined(&self) -> usize {
        self.relation_records_examined
    }

    pub const fn endpoint_records_reserved(&self) -> usize {
        self.records.len()
    }

    pub const fn work_units(&self) -> usize {
        self.adjacency_lists_read
            .saturating_add(self.relation_records_examined)
            .saturating_add(self.records.len())
    }

    pub fn into_records(self) -> Vec<RelationReadRecord> {
        self.records
    }
}

impl FrontierAdjacencyTruthReadLimitExceeded {
    const fn new(
        adjacency_lists_read: usize,
        relation_records_examined: usize,
        endpoint_records_reserved: usize,
    ) -> Self {
        Self {
            adjacency_lists_read,
            relation_records_examined,
            endpoint_records_reserved,
        }
    }

    pub const fn adjacency_lists_read(self) -> usize {
        self.adjacency_lists_read
    }

    pub const fn relation_records_examined(self) -> usize {
        self.relation_records_examined
    }

    pub const fn endpoint_records_reserved(self) -> usize {
        self.endpoint_records_reserved
    }

    pub const fn consumed_work_units(self) -> usize {
        self.adjacency_lists_read
            .saturating_add(self.relation_records_examined)
            .saturating_add(self.endpoint_records_reserved)
    }
}

fn charge_frontier_work(
    maximum_work_units: usize,
    adjacency_lists_read: usize,
    relation_records_examined: usize,
    endpoint_records_reserved: usize,
) -> Result<(), FrontierAdjacencyTruthReadLimitExceeded> {
    if adjacency_lists_read
        .saturating_add(relation_records_examined)
        .saturating_add(endpoint_records_reserved)
        >= maximum_work_units
    {
        Err(FrontierAdjacencyTruthReadLimitExceeded::new(
            adjacency_lists_read,
            relation_records_examined,
            endpoint_records_reserved,
        ))
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum FrontierAdjacencyDirection {
    Outgoing,
    Incoming,
}

impl FrontierAdjacencyDirection {
    fn matches_endpoint(
        self,
        record: &RelationReadRecord,
        entity_id: crate::identity::data::EntityId,
    ) -> bool {
        match self {
            Self::Outgoing => record.source == entity_id,
            Self::Incoming => record.target == entity_id,
        }
    }
}
