use std::collections::BTreeMap;

use worth_relational::facade::{
    identity::{EntityId, PartitionId},
    transactions::RecordRef,
};
use worth_runtime_bridge::facade::{BridgeInstalledConditionalLowering, BridgeSemanticLocality};

use crate::domain_computation::primary_graph::conditional_operation::temporal_reconstruction::WorthQueryReconstructedTemporalIntent;

pub(super) fn temporal_commit_routes<Clock, Input>(
    intents: &BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
    lowering: &BridgeInstalledConditionalLowering,
) -> (Vec<RecordRef>, bool) {
    let records = intents
        .values()
        .map(|intent| {
            let record = intent.source_record();
            RecordRef::Entity(EntityId::new(
                PartitionId(record.partition_id()),
                record.local_slot(),
                record.generation(),
            ))
        })
        .collect();
    let whole_graph = (0..lowering.contract().dependency_count()).any(|ordinal| {
        matches!(
            lowering.dependency_locality(ordinal),
            Some(BridgeSemanticLocality::WholeLogicalGraph)
        )
    });
    (records, whole_graph)
}
